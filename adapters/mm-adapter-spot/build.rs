fn main() {
    #[cfg(not(feature = "spot"))]
    {
        return;
    }

    #[cfg(feature = "spot")]
    build_spot();
}

#[cfg(feature = "spot")]
fn build_spot() {
    use std::path::PathBuf;

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    let sdk_root = std::env::var("SPOT_SDK_ROOT").ok().map(PathBuf::from);

    let mut build = cc::Build::new();
    build.file("src/shim.c");

    match target_os.as_str() {
        "macos" => {
            // SpotCam.framework — default location
            let framework_base = sdk_root
                .unwrap_or_else(|| PathBuf::from("/Library/Frameworks"));

            let headers = framework_base.join("SpotCam.framework/Headers");
            if headers.exists() {
                build.include(&headers);
            }

            println!("cargo:rustc-link-search=framework={}", framework_base.display());
            println!("cargo:rustc-link-lib=framework=SpotCam");
        }
        "windows" => {
            // SpotCam.dll is loaded at runtime via LoadLibrary on Windows;
            // link against the import lib if SPOT_SDK_ROOT is set, otherwise
            // the shim uses GetProcAddress (dynamic loading).
            if let Some(root) = sdk_root {
                let lib_dir = root.join("lib");
                build.include(root.join("include"));
                println!("cargo:rustc-link-search={}", lib_dir.display());
                println!("cargo:rustc-link-lib=SpotCam");
            }
            // If SPOT_SDK_ROOT is not set, shim falls back to runtime DLL load.
        }
        "linux" => {
            // No official Linux SDK; flag it.
            eprintln!("cargo:warning=SpotCam SDK is not available on Linux");
        }
        _ => {}
    }

    build.compile("spot_shim");
    println!("cargo:rerun-if-changed=src/shim.c");
    println!("cargo:rerun-if-env-changed=SPOT_SDK_ROOT");
}
