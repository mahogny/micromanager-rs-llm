/// Build script for mm-adapter-tsi.
///
/// When `--features tsi` is active, compiles `src/shim.c` (a thin C wrapper
/// around the Thorlabs Scientific Camera SDK3 C API) and links the SDK library.
///
/// Install the Thorlabs Scientific Camera SDK from
///   https://www.thorlabs.com/software_pages/ViewSoftwarePage.cfm?Code=ThorCam
///
/// Default search locations:
///   macOS  → /Library/Application Support/Thorlabs/Scientific Camera SDK/
///   Linux  → /opt/thorlabs/tsi_sdk/
///
/// Override with the `TSI_SDK_ROOT` environment variable pointing to the
/// directory that contains `include/` (or `includes/`) and `lib/` (or `libs/`)
/// sub-directories.

fn main() {
    if std::env::var("CARGO_FEATURE_TSI").is_err() {
        return;
    }

    let sdk_root = find_sdk_root();

    // ── Compile the C shim ────────────────────────────────────────────────────
    let mut build = cc::Build::new();
    // Try both conventional include directory spellings used by Thorlabs.
    for sub in &["include", "includes", "SDK/include",
                 "Scientific Camera Interfaces/SDK/Native Toolkit/include"] {
        let p = format!("{}/{}", sdk_root, sub);
        if std::path::Path::new(&p).exists() {
            build.include(&p);
        }
    }
    build.file("src/shim.c").warnings(false).compile("tsi_shim");

    // ── Link the SDK library ──────────────────────────────────────────────────
    for sub in &["lib", "libs", "SDK/lib",
                 "Scientific Camera Interfaces/SDK/Native Toolkit/lib"] {
        let p = format!("{}/{}", sdk_root, sub);
        if std::path::Path::new(&p).exists() {
            println!("cargo:rustc-link-search=native={}", p);
        }
    }

    // Thorlabs SDK3 library name is the same on all platforms.
    println!("cargo:rustc-link-lib=tl_camera_sdk");

    // On macOS also link pthread (already available on Linux via glibc).
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=pthread");

    println!("cargo:rerun-if-env-changed=TSI_SDK_ROOT");
    println!("cargo:rerun-if-changed=src/shim.c");
}

fn find_sdk_root() -> String {
    if let Ok(root) = std::env::var("TSI_SDK_ROOT") {
        return root;
    }

    #[cfg(target_os = "macos")]
    {
        let candidates = [
            "/Library/Application Support/Thorlabs/Scientific Camera SDK",
            "/usr/local/thorlabs/tsi_sdk",
        ];
        for c in &candidates {
            if std::path::Path::new(c).exists() {
                return c.to_string();
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        let candidates = [
            "/opt/thorlabs/tsi_sdk",
            "/usr/local/tsi_sdk",
        ];
        for c in &candidates {
            if std::path::Path::new(c).exists() {
                return c.to_string();
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        let c = r"C:\Program Files\Thorlabs\Scientific Imaging\Scientific Camera SDK";
        if std::path::Path::new(c).exists() {
            return c.to_string();
        }
    }

    panic!(
        "Thorlabs Scientific Camera SDK not found. \
         Install it from https://www.thorlabs.com/software_pages/ViewSoftwarePage.cfm?Code=ThorCam \
         and set TSI_SDK_ROOT to the SDK root directory."
    );
}
