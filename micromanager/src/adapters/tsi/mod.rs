/// Thorlabs Scientific Imaging camera adapter (TSI SDK3).
///
/// Wraps the Thorlabs Scientific Camera SDK3 C API behind the MicroManager
/// `Camera` trait via a thin C shim (`src/shim.c`).
///
/// # Setup
///
/// 1. Install the [Thorlabs Scientific Camera SDK](https://www.thorlabs.com/software_pages/ViewSoftwarePage.cfm?Code=ThorCam)
///    for your platform.
/// 2. Build with: `cargo build -p mm-adapter-tsi --features tsi`
///
/// Set `TSI_SDK_ROOT` to the SDK installation root if it is not found
/// automatically.
///
/// # Properties
///
/// | Property | R/W | Description |
/// |---|---|---|
/// | `CameraID`    | R/W (pre-init) | Camera ID string from SDK discovery; empty = first found |
/// | `Exposure`    | R/W | Exposure time in **milliseconds** (SDK3 uses µs internally) |
/// | `Binning`     | R/W | Symmetric horizontal+vertical binning factor |
/// | `Width`       | R   | Active image width in pixels |
/// | `Height`      | R   | Active image height in pixels |
/// | `BitDepth`    | R   | Significant bits per pixel |
/// | `SensorType`  | R   | "Monochrome", "Bayer", or "Polarized" |
/// | `SerialNumber`| R   | Camera serial number |
/// | `FirmwareVer` | R   | Firmware version string |
///
/// # Snap vs. sequence
///
/// `snap_image()` arms the camera for 1 frame, issues a software trigger,
/// waits for the frame callback, then disarms.
/// `start_sequence_acquisition()` arms for unlimited frames and issues a
/// single software trigger; the camera then delivers frames continuously.
/// Subsequent `snap_image()` calls wait for and return the next available
/// frame.

#[cfg(feature = "tsi")]
pub mod ffi;
#[cfg(feature = "tsi")]
pub mod camera;
#[cfg(feature = "tsi")]
pub use camera::TSICamera;
