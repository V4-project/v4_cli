use cmake::Config;
use std::path::PathBuf;

fn main() {
    // Get absolute paths to vendor directories
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let v4_path = manifest_dir.join("vendor/V4");
    let v4front_path = manifest_dir.join("vendor/V4-front");

    // Build V4 VM library first
    let mut v4_config = Config::new(&v4_path);
    v4_config
        .define("V4_BUILD_TESTS", "OFF")
        .define("V4_BUILD_TOOLS", "OFF")
        .define("V4_ENABLE_MOCK_HAL", "OFF")
        .out_dir(manifest_dir.join("target/v4"));

    // Set build profile for Windows
    #[cfg(target_os = "windows")]
    {
        if cfg!(debug_assertions) {
            v4_config.profile("Debug");
        } else {
            v4_config.profile("Release");
        }
    }

    let v4_dst = v4_config.build();

    // Build V4-front compiler library (depends on V4)
    let mut v4front_config = Config::new(&v4front_path);
    v4front_config
        .define("V4FRONT_BUILD_TESTS", "OFF")
        .define("V4_SRC_DIR", v4_path.to_str().unwrap())
        .out_dir(manifest_dir.join("target/v4front"));

    // Set build profile for Windows
    #[cfg(target_os = "windows")]
    {
        if cfg!(debug_assertions) {
            v4front_config.profile("Debug");
        } else {
            v4front_config.profile("Release");
        }
    }

    let v4front_dst = v4front_config.build();

    // Link libraries
    // On Windows, CMake generates libraries in Debug/Release subdirectories
    #[cfg(target_os = "windows")]
    {
        let profile = if cfg!(debug_assertions) {
            "Debug"
        } else {
            "Release"
        };
        // V4 VM has install target, libraries go to lib/
        let v4_lib_path = v4_dst.join("lib");
        println!("cargo:rustc-link-search=native={}", v4_lib_path.display());
        println!("cargo:warning=V4 lib path: {}", v4_lib_path.display());

        // V4-front doesn't have install target, check multiple possible locations
        let v4front_build_path = v4front_dst.join("build").join(&profile);
        println!(
            "cargo:rustc-link-search=native={}",
            v4front_build_path.display()
        );
        println!(
            "cargo:warning=V4-front build path: {}",
            v4front_build_path.display()
        );

        // Also try the build directory root (sometimes CMake puts libs there)
        let v4front_build_root = v4front_dst.join("build");
        println!(
            "cargo:rustc-link-search=native={}",
            v4front_build_root.display()
        );

        // List directory contents for debugging
        if let Ok(entries) = std::fs::read_dir(&v4front_build_path) {
            println!("cargo:warning=V4-front build/{} contents:", profile);
            for entry in entries.flatten() {
                println!("cargo:warning=  - {}", entry.file_name().to_string_lossy());
            }
        } else {
            println!("cargo:warning=V4-front build/{} does not exist!", profile);
        }

        // Also check the parent build directory to see what's there
        if let Ok(entries) = std::fs::read_dir(&v4front_build_root) {
            println!("cargo:warning=V4-front build/ root contents:");
            for entry in entries.flatten() {
                let name = entry.file_name();
                println!("cargo:warning=  - {}", name.to_string_lossy());
            }
        } else {
            println!("cargo:warning=V4-front build/ root does not exist!");
        }

        // Check v4front_dst directory
        if let Ok(entries) = std::fs::read_dir(&v4front_dst) {
            println!(
                "cargo:warning=V4-front dst ({}) contents:",
                v4front_dst.display()
            );
            for entry in entries.flatten() {
                println!("cargo:warning=  - {}", entry.file_name().to_string_lossy());
            }
        }
    }

    // On Unix, libraries are in lib/ or build/
    #[cfg(not(target_os = "windows"))]
    {
        // V4 VM has install target, libraries go to lib/
        println!("cargo:rustc-link-search=native={}/lib", v4_dst.display());
        // V4-front doesn't have install target, link directly from build directory
        println!(
            "cargo:rustc-link-search=native={}/build",
            v4front_dst.display()
        );
    }

    println!("cargo:rustc-link-lib=static=v4engine");
    println!("cargo:rustc-link-lib=static=v4front");

    // Link C++ standard library (required by V4-front)
    // macOS uses libc++, Windows uses built-in, other platforms use libstdc++
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++");
    #[cfg(all(not(target_os = "macos"), not(target_os = "windows")))]
    println!("cargo:rustc-link-lib=stdc++");

    // Rebuild triggers
    println!("cargo:rerun-if-changed={}/src", v4_path.display());
    println!("cargo:rerun-if-changed={}/include", v4_path.display());
    println!("cargo:rerun-if-changed={}/src", v4front_path.display());
    println!("cargo:rerun-if-changed={}/include", v4front_path.display());
}
