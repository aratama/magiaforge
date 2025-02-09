# magiaforge

Online Battle Royale Twin-stick Shooter. WIP.

![](misc/screenshot_title.png)

![](misc/screenshot.png)

### Demo

[![Release](https://github.com/aratama/magiaforge/actions/workflows/release.yaml/badge.svg)](https://github.com/aratama/magiaforge/actions/workflows/release.yaml)

https://magiaforge.app/ (**Desktop Chrome only** for now)

- `w/a/s/d`: Move
- `Mouse left button`: Cast primary spells
- `Mouse right button`: Cast secondary spells
- `Mouse wheel`: Switch primary spells slot
- `Tab`: Open inventory
- `Esc`: Open pause menu

### Development

- `cargo run` to launch app locally
- `cargo build --profile dist` to build desktop app in release mode. (Note `cargo build` is for WASM build)
- `trunk serve` to run locally on browser with [trunk](https://trunkrs.dev/)
- `trunk build` to build web app and publish on GitHub Pages
- `cargo +nightly fmt` to format

###### Feature Flags

`cargo run --features xxxx,yyyy` to enable some debug features:

- `debug` to enable debug draw.
- `ingame` to skip main menu.
- `item` to enable spell item cheat.

### Authoring Tools

This project depends on the following tools:

- [LDtk](https://ldtk.io/) for level design
- [Aseprite](https://www.aseprite.org/) for pixel art drawing

### Save Data Location

On Windows, save data is stored in `%HOMEPATH%\AppData\Roaming\magiaforge`.

In browsers, save data is stored in local storage under the key `magiaforge.magiaforgeconfig`.

### Asset Credits

This game depends on free musical assets from https://dova-s.jp/ .
See [the asset creadit file](assets/registry.game.ron) for more details (Also you can see BGM credit in pause menu).
All Sound Effects are from https://soundeffect-lab.info/ .
