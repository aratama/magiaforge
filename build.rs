// https://bevy-cheatbook.github.io/platforms/windows.html#disabling-the-windows-console
// https://crates.io/crates/embed-resource

extern crate embed_resource;

fn main() {
    let target = std::env::var("TARGET").unwrap();
    if target.contains("windows") {
        embed_resource::compile("magiaboost-icon.rc", embed_resource::NONE);
        embed_resource::compile("magiaboost-manifest.rc", embed_resource::NONE);
    }
}
