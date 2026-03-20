/// Build script for mm-adapter-picam.
///
/// When `--features picam` is active, compiles `src/shim.c` (a thin C wrapper
/// around the PVCAM C API) and links the PVCAM library.
///
/// Install PVCAM SDK from https://www.princetoninstruments.com/products/software-library/pvcam
///   macOS  → installs as /Library/Frameworks/PICAM.framework/
///   Linux  → /usr/local/lib/libpvcam.so  (or /usr/lib/)
///
/// Override with PVCAM_ROOT env var pointing to the directory that contains
/// `Includes/` and `Libraries/` (or the framework on macOS).

fn main() {
    if std::env::var("CARGO_FEATURE_PICAM").is_err() {
        return;
    }

    let mut build = cc::Build::new();
    build.file("src/shim.c").warnings(false);

    #[cfg(target_os = "macos")]
    {
        // Princeton Instruments ships PVCAM as a macOS Framework.
        // Headers live inside the framework bundle.
        let framework_root = std::env::var("PVCAM_ROOT")
            .unwrap_or_else(|_| "/Library/Frameworks/PICAM.framework".into());
        build.include(format!("{}/Headers", framework_root));
        build.compile("picam_shim");
        // Link the framework.
        println!("cargo:rustc-link-lib=framework=PICAM");
        // Framework search path (default system path is already searched).
        println!("cargo:rustc-link-search=framework=/Library/Frameworks");
    }

    #[cfg(target_os = "linux")]
    {
        let root = std::env::var("PVCAM_ROOT").unwrap_or_else(|_| "/usr/local".into());
        build.include(format!("{}/include/pvcam", root));
        build.compile("picam_shim");
        println!("cargo:rustc-link-search=native={}/lib", root);
        println!("cargo:rustc-link-lib=pvcam");
    }

    #[cfg(target_os = "windows")]
    {
        // On Windows PVCAM is distributed as Picam.lib (newer SDK).
        // Fall back to pvcam32.lib for legacy systems.
        let root = std::env::var("PVCAM_ROOT").unwrap_or_else(|_| {
            r"C:\Program Files\Princeton Instruments\PVCAM".into()
        });
        build.include(format!("{}\\SDK\\inc", root));
        build.compile("picam_shim");
        println!("cargo:rustc-link-search=native={}\\SDK\\lib", root);
        println!("cargo:rustc-link-lib=pvcam32");
    }

    println!("cargo:rerun-if-env-changed=PVCAM_ROOT");
    println!("cargo:rerun-if-changed=src/shim.c");
}
