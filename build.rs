use cmake::Config;
use std::path::PathBuf;

fn main() {
    // Get absolute paths to V4 repositories
    // In V4-project workspace, v4_cli is sibling to V4-engine and V4-front
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let v4_path = manifest_dir.parent().unwrap().join("V4-engine");
    let v4front_path = manifest_dir.parent().unwrap().join("V4-front");

    // Use the engine's error definitions as the only source of VM messages.
    let errors_path = v4_path.join("include/v4/errors.def");
    let definitions = std::fs::read_to_string(&errors_path).expect("read engine errors.def");
    let mut messages =
        String::from("pub fn vm_error_message(code: i32) -> &'static str {\nmatch code {\n");
    for line in definitions.lines().map(str::trim) {
        if let Some(entry) = line.strip_prefix("ERR(") {
            let entry = entry
                .strip_suffix(')')
                .expect("engine ERR entry terminator");
            let fields: Vec<_> = entry.splitn(3, ',').map(str::trim).collect();
            let code: i32 = fields[1].parse().expect("engine error number");
            messages.push_str(&format!("{code} => {},\n", fields[2]));
        }
    }
    messages.push_str("_ => \"unknown error\",\n}\n}\n");
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").expect("OUT_DIR"));
    std::fs::write(out_dir.join("vm_errors.rs"), messages).expect("write VM error messages");
    println!("cargo:rerun-if-changed={}", errors_path.display());

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
