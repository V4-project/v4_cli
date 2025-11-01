use cmake::Config;
use std::path::PathBuf;

fn main() {
    // Get absolute paths to vendor directories
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let v4_path = manifest_dir.join("vendor/V4");
    let v4front_path = manifest_dir.join("vendor/V4-front");

    // Build V4 VM library first
    let v4_dst = Config::new(&v4_path)
        .define("V4_BUILD_TESTS", "OFF")
        .define("V4_BUILD_TOOLS", "OFF")
        .define("V4_ENABLE_MOCK_HAL", "OFF")
        .out_dir(manifest_dir.join("target/v4"))
        .build();

    // Build V4-front compiler library (depends on V4)
    let v4front_dst = Config::new(&v4front_path)
        .define("V4FRONT_BUILD_TESTS", "OFF")
        .define("V4_SRC_DIR", v4_path.to_str().unwrap())
        .out_dir(manifest_dir.join("target/v4front"))
        .build();

    // Link libraries
    println!("cargo:rustc-link-search=native={}/lib", v4_dst.display());
    println!(
        "cargo:rustc-link-search=native={}/lib",
        v4front_dst.display()
    );
    // V4-front doesn't have install target, so link directly from build directory
    println!(
        "cargo:rustc-link-search=native={}/build",
        v4front_dst.display()
    );

    println!("cargo:rustc-link-lib=static=v4vm");
    println!("cargo:rustc-link-lib=static=v4front");

    // Link C++ standard library (required by V4-front)
    // macOS uses libc++, other platforms use libstdc++
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++");
    #[cfg(not(target_os = "macos"))]
    println!("cargo:rustc-link-lib=stdc++");

    // Rebuild triggers
    println!("cargo:rerun-if-changed={}/src", v4_path.display());
    println!("cargo:rerun-if-changed={}/include", v4_path.display());
    println!("cargo:rerun-if-changed={}/src", v4front_path.display());
    println!("cargo:rerun-if-changed={}/include", v4front_path.display());
}
