// https://bevy-cheatbook.github.io/platforms/windows.html#disabling-the-windows-console
// https://crates.io/crates/embed-resource

extern crate embed_resource;

use chrono;

fn main() {
    let now = chrono::Utc::now();
    let datetime = now.format("%Y-%m-%d").to_string();
    println!("cargo:rustc-env=BUILD_DATETIME={}", datetime);

    let target = std::env::var("TARGET").unwrap();
    if target.contains("windows") {
        embed_resource::compile("build/windows/icon.rc", embed_resource::NONE);
        embed_resource::compile("build/windows/manifest.rc", embed_resource::NONE);
    }
}
