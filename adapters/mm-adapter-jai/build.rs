/// Build script for mm-adapter-jai.
///
/// When `--features jai` is active, this script:
///   1. Locates the Pleora eBUS SDK (via `EBUS_SDK_ROOT` env var or common
///      install paths).
///   2. Compiles `src/shim.cpp` — a thin C wrapper around the C++ eBUS SDK
///      that exposes a plain C API for Rust's `extern "C"` FFI.
///   3. Links the eBUS SDK shared libraries.
///
/// Install eBUS SDK from https://www.pleora.com/support-center/ebus-sdk/
/// Default install location:
///   macOS  → /opt/pleora/ebus_sdk/Darwin-{arm64,x86_64}-release/
///   Linux  → /opt/pleora/ebus_sdk/Ubuntu-x86_64-release/   (distro varies)
///   Windows → C:\Program Files\Pleora Technologies Inc\eBUS SDK\

fn main() {
    if std::env::var("CARGO_FEATURE_JAI").is_err() {
        return;
    }

    let sdk_root = find_sdk_root();

    // ── Compile the C shim ────────────────────────────────────────────────────
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .file("src/shim.cpp")
        .include(format!("{}/Includes", sdk_root))
        .flag_if_supported("-std=c++14")
        .flag_if_supported("-Wno-deprecated-declarations");

    // On macOS/Linux the eBUS SDK headers use PvBase without namespace;
    // suppress common warnings from vendor headers.
    if cfg!(target_os = "macos") {
        build.flag_if_supported("-Wno-unknown-pragmas");
    }

    build.compile("jai_shim");

    // ── Link eBUS SDK libraries ───────────────────────────────────────────────
    let lib_dir = format!("{}/Libraries", sdk_root);
    println!("cargo:rustc-link-search=native={}", lib_dir);

    // Core eBUS libraries required for device enumeration, connection, and
    // streaming.  The exact set depends on SDK version; extras are ignored if
    // absent on the linker search path.
    for lib in &[
        "PvBase",
        "PvDevice",
        "PvBuffer",
        "PvStream",
        "PvGenICam",
        "EbNetworkLib",
        "EbUSBLib",
        "PvTransmitter",
    ] {
        println!("cargo:rustc-link-lib={}", lib);
    }

    // Re-run if the env var changes.
    println!("cargo:rerun-if-env-changed=EBUS_SDK_ROOT");
    println!("cargo:rerun-if-changed=src/shim.cpp");
}

fn find_sdk_root() -> String {
    // 1. Explicit env var wins.
    if let Ok(root) = std::env::var("EBUS_SDK_ROOT") {
        return root;
    }

    // 2. Scan common macOS install prefixes.
    #[cfg(target_os = "macos")]
    {
        let base = "/opt/pleora/ebus_sdk";
        if let Ok(entries) = std::fs::read_dir(base) {
            let mut candidates: Vec<_> = entries
                .filter_map(|e| e.ok())
                .filter(|e| {
                    let name = e.file_name();
                    let s = name.to_string_lossy();
                    s.starts_with("Darwin-")
                })
                .collect();
            candidates.sort_by_key(|e| e.file_name());
            if let Some(last) = candidates.last() {
                return last.path().to_string_lossy().into_owned();
            }
        }
    }

    // 3. Linux common install.
    #[cfg(target_os = "linux")]
    {
        let base = "/opt/pleora/ebus_sdk";
        if let Ok(entries) = std::fs::read_dir(base) {
            let mut candidates: Vec<_> = entries.filter_map(|e| e.ok()).collect();
            candidates.sort_by_key(|e| e.file_name());
            if let Some(last) = candidates.last() {
                return last.path().to_string_lossy().into_owned();
            }
        }
    }

    // 4. Windows default.
    #[cfg(target_os = "windows")]
    {
        let win = r"C:\Program Files\Pleora Technologies Inc\eBUS SDK";
        if std::path::Path::new(win).exists() {
            return win.to_string();
        }
    }

    panic!(
        "eBUS SDK not found. Install the Pleora eBUS SDK and set EBUS_SDK_ROOT \
         to its root directory (the one containing Includes/ and Libraries/)."
    );
}
