use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    // Vendor directory paths
    let v4_path = PathBuf::from("vendor/V4");
    let v4front_path = PathBuf::from("vendor/V4-front");
    let v4repl_path = PathBuf::from("vendor/V4-repl");

    // Build V4 VM library
    let v4_dst = Config::new(&v4_path).define("BUILD_TESTS", "OFF").build();

    // Build V4-front compiler library
    let v4front_dst = Config::new(&v4front_path)
        .define("BUILD_TESTS", "OFF")
        .build();

    // Build V4-repl library (depends on V4 and V4-front)
    let v4repl_dst = Config::new(&v4repl_path)
        .define("BUILD_TESTS", "OFF")
        .define("WITH_FILESYSTEM", "OFF") // Disable filesystem for embedded compatibility
        .define("V4_LOCAL_PATH", v4_path.to_str().unwrap())
        .define("V4FRONT_LOCAL_PATH", v4front_path.to_str().unwrap())
        .build();

    // Link libraries
    println!("cargo:rustc-link-search=native={}/lib", v4_dst.display());
    println!(
        "cargo:rustc-link-search=native={}/lib",
        v4front_dst.display()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib",
        v4repl_dst.display()
    );

    println!("cargo:rustc-link-lib=static=v4");
    println!("cargo:rustc-link-lib=static=v4front");
    println!("cargo:rustc-link-lib=static=v4repl");

    // Link C++ standard library (required by V4-repl)
    println!("cargo:rustc-link-lib=stdc++");

    // Rebuild triggers
    println!("cargo:rerun-if-changed=vendor/V4/src");
    println!("cargo:rerun-if-changed=vendor/V4/include");
    println!("cargo:rerun-if-changed=vendor/V4-front/src");
    println!("cargo:rerun-if-changed=vendor/V4-front/include");
    println!("cargo:rerun-if-changed=vendor/V4-repl/src");
    println!("cargo:rerun-if-changed=vendor/V4-repl/include");
}
