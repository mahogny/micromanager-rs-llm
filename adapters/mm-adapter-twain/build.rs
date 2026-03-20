fn main() {
    #[cfg(not(feature = "twain"))]
    {
        return;
    }

    #[cfg(feature = "twain")]
    build_twain();
}

#[cfg(feature = "twain")]
fn build_twain() {
    use std::path::PathBuf;

    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    // The TWAIN 2.x header (twain.h) — look in the reference source tree first,
    // then fall back to the SDK root override.
    let ref_include: PathBuf = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../mmCoreAndDevices/DeviceAdapters/TwainCamera");

    let sdk_root = std::env::var("TWAIN_SDK_ROOT").ok().map(PathBuf::from);

    let mut build = cc::Build::new();
    build.file("src/shim.c");

    // Always add the reference source tree for twain.h access.
    if ref_include.exists() {
        build.include(&ref_include);
    }

    match target_os.as_str() {
        "windows" => {
            // TWAIN DSM is loaded at runtime via LoadLibrary; no static lib needed.
            // Link user32 and gdi32 for the hidden-window Win32 APIs.
            println!("cargo:rustc-link-lib=user32");
            println!("cargo:rustc-link-lib=gdi32");

            if let Some(root) = sdk_root {
                build.include(root.join("include"));
            }
        }
        "linux" => {
            // libtwain-dev provides TWAINDSM shared library and twain.h.
            let inc = sdk_root
                .map(|r| r.join("include"))
                .unwrap_or_else(|| PathBuf::from("/usr/include"));
            build.include(inc);
            // Runtime-loaded via dlopen; no static link needed.
            println!("cargo:rustc-link-lib=dl");
        }
        _ => {
            eprintln!("cargo:warning=TWAIN is not supported on this platform");
        }
    }

    build.compile("twain_shim");
    println!("cargo:rerun-if-changed=src/shim.c");
    println!("cargo:rerun-if-env-changed=TWAIN_SDK_ROOT");
}
