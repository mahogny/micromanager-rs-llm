/**
 * shim.c — Thin C wrapper around the Thorlabs Scientific Camera SDK3 C API.
 *
 * Exposes a simplified opaque-context API suitable for Rust FFI.  The SDK3
 * API is callback-based; this shim uses a volatile flag + polling sleep to
 * provide synchronous snap semantics without platform-specific event objects.
 *
 * Exposure time unit: milliseconds in the public API, microseconds internally
 * (TSI SDK3 uses µs).
 */

/* ── Platform includes ────────────────────────────────────────────────────── */

#ifdef _WIN32
#  define WIN32_LEAN_AND_MEAN
#  include <windows.h>
   static void shim_sleep_ms(int ms) { Sleep((DWORD)ms); }
#else
#  include <unistd.h>
   static void shim_sleep_ms(int ms) { usleep((useconds_t)(ms) * 1000u); }
#endif

#include "tl_camera_sdk.h"

#include <stdlib.h>
#include <string.h>
#include <stdint.h>

/* ── Context struct ───────────────────────────────────────────────────────── */

typedef struct TsiCtx {
    void*     handle;           /* TSI SDK camera handle */

    /* Cached sensor properties (read once on open). */
    int       sensor_type;      /* 0 = mono, 1 = bayer, 2 = polarized */
    int       bit_depth;
    int       bytes_per_pixel;  /* always 2 for SDK3 uint16 output */
    int       sensor_width;     /* full sensor, no binning */
    int       sensor_height;

    /* Current image dimensions (after ROI + binning). */
    int       img_width;
    int       img_height;

    /* Internal frame buffer (owned). */
    uint16_t* image_buf;
    size_t    image_buf_bytes;
    int       frame_bytes;      /* bytes in the last delivered frame */

    /* Synchronisation: callback sets frame_ready = 1; waiter polls. */
    volatile int frame_ready;

    /* State flags. */
    int       in_sequence;      /* 1 = continuous mode active */
} TsiCtx;

/* ── Frame-available callback ─────────────────────────────────────────────── */

static void frame_callback(
        void*           sender,
        unsigned short* image_buffer,
        int             frame_count,
        unsigned char*  metadata,
        int             metadata_size,
        void*           context)
{
    TsiCtx* ctx = (TsiCtx*)context;
    if (!ctx || !image_buffer) return;

    /* Compute actual frame size from current dimensions. */
    int fb = ctx->img_width * ctx->img_height * ctx->bytes_per_pixel;
    if (fb <= 0) return;

    /* (Re-)allocate internal buffer if needed. */
    if (fb > (int)ctx->image_buf_bytes) {
        free(ctx->image_buf);
        ctx->image_buf = (uint16_t*)malloc((size_t)fb);
        if (!ctx->image_buf) { ctx->image_buf_bytes = 0; return; }
        ctx->image_buf_bytes = (size_t)fb;
    }

    memcpy(ctx->image_buf, image_buffer, (size_t)fb);
    ctx->frame_bytes = fb;
    ctx->frame_ready = 1;   /* signal to waiter — must be last write */
}

/* ── Helpers ──────────────────────────────────────────────────────────────── */

static void refresh_image_dims(TsiCtx* ctx) {
    if (!ctx || !ctx->handle) return;
    tl_camera_get_image_width(ctx->handle,  &ctx->img_width);
    tl_camera_get_image_height(ctx->handle, &ctx->img_height);
}

static int wait_for_frame(TsiCtx* ctx, int timeout_ms) {
    int elapsed = 0;
    while (!ctx->frame_ready) {
        if (elapsed >= timeout_ms) return -1;
        shim_sleep_ms(1);
        elapsed++;
    }
    ctx->frame_ready = 0;
    return 0;
}

/* ── SDK lifecycle ────────────────────────────────────────────────────────── */

int tsi_sdk_open(void) {
    /* On Windows the MicroManager adapter calls tl_camera_sdk_dll_initialize()
       first; on macOS/Linux it is not needed (direct shared library link). */
#ifdef _WIN32
    if (tl_camera_sdk_dll_initialize() != 0) return -1;
#endif
    return tl_camera_open_sdk() == 0 ? 0 : -1;
}

void tsi_sdk_close(void) {
    tl_camera_close_sdk();
#ifdef _WIN32
    tl_camera_sdk_dll_uninitialize();
#endif
}

/* ── Camera discovery ─────────────────────────────────────────────────────── */

/**
 * Fill `buf` with a space-separated list of discovered camera ID strings.
 * Returns number of cameras found, or -1 on error.
 */
int tsi_discover_cameras(char* buf, int len) {
    if (!buf || len <= 0) return -1;
    buf[0] = '\0';
    if (tl_camera_discover_available_cameras(buf, len) != 0) return -1;

    /* Count space-separated tokens. */
    if (buf[0] == '\0') return 0;
    int count = 1;
    for (int i = 0; buf[i]; i++) {
        if (buf[i] == ' ' && buf[i + 1] != '\0') count++;
    }
    return count;
}

/* ── Camera open / close ──────────────────────────────────────────────────── */

TsiCtx* tsi_open_camera(const char* camera_id) {
    if (!camera_id) return NULL;

    void* handle = NULL;
    if (tl_camera_open_camera(camera_id, &handle) != 0 || !handle)
        return NULL;

    TsiCtx* ctx = (TsiCtx*)calloc(1, sizeof(TsiCtx));
    if (!ctx) { tl_camera_close_camera(handle); return NULL; }
    ctx->handle = handle;

    /* Software-triggered mode; frames_per_trigger = 1 for snap (overridden per call). */
    tl_camera_set_operation_mode(handle, TL_CAMERA_OPERATION_MODE_SOFTWARE_TRIGGERED);

    /* Read sensor properties. */
    tl_camera_get_camera_sensor_type(handle, &ctx->sensor_type);
    tl_camera_get_bit_depth(handle, &ctx->bit_depth);
    ctx->bytes_per_pixel = 2;   /* SDK3 always delivers uint16 */

    tl_camera_get_image_width(handle,  &ctx->sensor_width);
    tl_camera_get_image_height(handle, &ctx->sensor_height);
    ctx->img_width  = ctx->sensor_width;
    ctx->img_height = ctx->sensor_height;

    /* Register frame callback. */
    tl_camera_set_frame_available_callback(handle, frame_callback, ctx);

    return ctx;
}

void tsi_close_camera(TsiCtx* ctx) {
    if (!ctx) return;
    if (ctx->in_sequence) {
        tl_camera_disarm(ctx->handle);
        ctx->in_sequence = 0;
    }
    tl_camera_close_camera(ctx->handle);
    free(ctx->image_buf);
    free(ctx);
}

/* ── Property getters ─────────────────────────────────────────────────────── */

int tsi_get_image_width(TsiCtx* ctx)         { return ctx ? ctx->img_width  : 0; }
int tsi_get_image_height(TsiCtx* ctx)        { return ctx ? ctx->img_height : 0; }
int tsi_get_sensor_width(TsiCtx* ctx)        { return ctx ? ctx->sensor_width  : 0; }
int tsi_get_sensor_height(TsiCtx* ctx)       { return ctx ? ctx->sensor_height : 0; }
int tsi_get_bit_depth(TsiCtx* ctx)           { return ctx ? ctx->bit_depth : 0; }
int tsi_get_bytes_per_pixel(TsiCtx* ctx)     { return ctx ? ctx->bytes_per_pixel : 0; }
int tsi_get_sensor_type(TsiCtx* ctx)         { return ctx ? ctx->sensor_type : 0; }

int tsi_get_serial_number(TsiCtx* ctx, char* buf, int len) {
    if (!ctx || !buf) return -1;
    return tl_camera_get_serial_number(ctx->handle, buf, len) == 0 ? 0 : -1;
}

int tsi_get_firmware_version(TsiCtx* ctx, char* buf, int len) {
    if (!ctx || !buf) return -1;
    return tl_camera_get_firmware_version(ctx->handle, buf, len) == 0 ? 0 : -1;
}

/* ── Exposure (milliseconds in API, microseconds in SDK) ─────────────────── */

long long tsi_get_exposure_us(TsiCtx* ctx) {
    if (!ctx) return -1;
    long long v = 0;
    tl_camera_get_exposure_time(ctx->handle, &v);
    return v;
}

int tsi_set_exposure_us(TsiCtx* ctx, long long us) {
    if (!ctx) return -1;
    return tl_camera_set_exposure_time(ctx->handle, us) == 0 ? 0 : -1;
}

int tsi_get_exposure_range_us(TsiCtx* ctx, long long* min_out, long long* max_out) {
    if (!ctx) return -1;
    return tl_camera_get_exposure_time_range(ctx->handle, min_out, max_out) == 0 ? 0 : -1;
}

/* ── ROI ──────────────────────────────────────────────────────────────────── */

/* SDK3 ROI is specified as (x_top_left, y_top_left, x_bottom_right, y_bottom_right).
   All coordinates are in unbinned pixels. */

int tsi_set_roi(TsiCtx* ctx, int x, int y, int w, int h) {
    if (!ctx) return -1;
    int x2 = x + w - 1;
    int y2 = y + h - 1;
    if (tl_camera_set_roi(ctx->handle, x, y, x2, y2) != 0) return -1;
    refresh_image_dims(ctx);
    return 0;
}

int tsi_clear_roi(TsiCtx* ctx) {
    if (!ctx) return -1;
    tl_camera_set_roi(ctx->handle, 0, 0,
                      ctx->sensor_width  - 1,
                      ctx->sensor_height - 1);
    refresh_image_dims(ctx);
    return 0;
}

int tsi_get_roi(TsiCtx* ctx, int* x, int* y, int* w, int* h) {
    if (!ctx) return -1;
    int x1 = 0, y1 = 0, x2 = 0, y2 = 0;
    if (tl_camera_get_roi(ctx->handle, &x1, &y1, &x2, &y2) != 0) return -1;
    *x = x1;  *y = y1;
    *w = x2 - x1 + 1;  *h = y2 - y1 + 1;
    return 0;
}

/* ── Binning ──────────────────────────────────────────────────────────────── */

int tsi_get_binx(TsiCtx* ctx) {
    if (!ctx) return 1;
    int v = 1;
    tl_camera_get_binx(ctx->handle, &v);
    return v;
}

int tsi_get_biny(TsiCtx* ctx) {
    if (!ctx) return 1;
    int v = 1;
    tl_camera_get_biny(ctx->handle, &v);
    return v;
}

int tsi_set_binx(TsiCtx* ctx, int val) {
    if (!ctx) return -1;
    if (tl_camera_set_binx(ctx->handle, val) != 0) return -1;
    refresh_image_dims(ctx);
    return 0;
}

int tsi_set_biny(TsiCtx* ctx, int val) {
    if (!ctx) return -1;
    if (tl_camera_set_biny(ctx->handle, val) != 0) return -1;
    refresh_image_dims(ctx);
    return 0;
}

int tsi_get_binx_range(TsiCtx* ctx, int* min_out, int* max_out) {
    if (!ctx) return -1;
    return tl_camera_get_binx_range(ctx->handle, min_out, max_out) == 0 ? 0 : -1;
}

/* ── Snap (single frame, blocking) ───────────────────────────────────────── */

/**
 * Snap one frame in software-trigger mode.
 * `timeout_ms` is the maximum wait including exposure + readout time.
 * Returns 0 on success, -1 on error or timeout.
 */
int tsi_snap(TsiCtx* ctx, int timeout_ms) {
    if (!ctx || ctx->in_sequence) return -1;

    ctx->frame_ready = 0;
    tl_camera_set_frames_per_trigger_zero_for_unlimited(ctx->handle, 1);

    if (tl_camera_arm(ctx->handle, 2) != 0) return -1;
    if (tl_camera_issue_software_trigger(ctx->handle) != 0) {
        tl_camera_disarm(ctx->handle);
        return -1;
    }

    int ret = wait_for_frame(ctx, timeout_ms);
    tl_camera_disarm(ctx->handle);
    return ret;
}

const uint16_t* tsi_get_frame_ptr(TsiCtx* ctx) {
    return ctx ? ctx->image_buf : NULL;
}

int tsi_get_frame_bytes(TsiCtx* ctx) {
    return ctx ? ctx->frame_bytes : 0;
}

/* ── Sequence acquisition ─────────────────────────────────────────────────── */

/**
 * Start continuous acquisition using software triggers.
 * One software trigger starts an unlimited frame stream.
 * Subsequent frames continue arriving until tsi_stop_cont().
 */
int tsi_start_cont(TsiCtx* ctx) {
    if (!ctx || ctx->in_sequence) return -1;

    ctx->frame_ready = 0;
    /* frames_per_trigger = 0 means unlimited after a single trigger. */
    tl_camera_set_frames_per_trigger_zero_for_unlimited(ctx->handle, 0);

    if (tl_camera_arm(ctx->handle, 8) != 0) return -1;
    if (tl_camera_issue_software_trigger(ctx->handle) != 0) {
        tl_camera_disarm(ctx->handle);
        return -1;
    }
    ctx->in_sequence = 1;
    return 0;
}

/**
 * Wait for the next frame from the continuous stream.
 * Returns 0 on success (frame copied to internal buffer), -1 on timeout.
 */
int tsi_get_next_frame(TsiCtx* ctx, int timeout_ms) {
    if (!ctx || !ctx->in_sequence) return -1;
    ctx->frame_ready = 0;
    return wait_for_frame(ctx, timeout_ms);
}

int tsi_stop_cont(TsiCtx* ctx) {
    if (!ctx) return -1;
    if (!ctx->in_sequence) return 0;
    tl_camera_disarm(ctx->handle);
    ctx->in_sequence = 0;
    return 0;
}
