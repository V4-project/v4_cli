use cmake::Config;
use std::env;

fn main() {
    // V4-frontのパスを環境変数から取得（デフォルト: ../V4-front）
    let v4front_path = env::var("V4_FRONT_PATH").unwrap_or_else(|_| "../V4-front".to_string());

    // CMake経由でV4-frontをビルド
    let dst = Config::new(&v4front_path)
        .define("BUILD_TESTS", "OFF")
        .build();

    // リンカーにライブラリパスを指定
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=v4front");

    // 再ビルドトリガー
    println!("cargo:rerun-if-changed={}/src", v4front_path);
    println!("cargo:rerun-if-changed={}/include", v4front_path);
    println!("cargo:rerun-if-env-changed=V4_FRONT_PATH");
}
