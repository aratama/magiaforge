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

Add `--features debug` to launch app in debug mode.
Add `--features save` to launch app with Save feature.

### Save Data Location

On Windows, save data is stored in `%HOMEPATH%\AppData\Roaming\magiaforge`.

In browsers, save data is stored in local storage under the key `magiaforge.magiaforgeconfig`.

### Asset credits

This game depends on the following assets:

- 茫漠たる庭 https://dova-s.jp/bgm/play21154.html
- 最果てのルージュ https://dova-s.jp/bgm/play19795.html
- 荒れ地の先へ https://dova-s.jp/bgm/play17195.html
- 悪魔との戦闘 https://dova-s.jp/bgm/play11175.html
- Tides of Adventure https://dova-s.jp/bgm/play21129.html
- ダンジョンを踏破せし者 https://dova-s.jp/bgm/play20822.html
- 森のいざない https://dova-s.jp/bgm/play12135.html
- 迷宮 https://dova-s.jp/bgm/play8802.html
- 忘れられた神殿 https://dova-s.jp/bgm/play629.html
- アクション・バトル https://dova-s.jp/bgm/play12133.html
- Decisive Battle https://dova-s.jp/bgm/play5746.html
- 炎神の吐息 https://dova-s.jp/bgm/play298.html
- Sacred Sacrifice https://dova-s.jp/bgm/play6142.html
- FINAL BATTLE (TRAILER MUSIC) https://pixabay.com/music/electronic-final-battle-trailer-music-217488/
- Human vs Machine (dark orchestral cinematic epic action) https://pixabay.com/music/main-title-human-vs-machine-dark-orchestral-cinematic-epic-action-271968/
- Midnight Forest https://pixabay.com/music/ambient-midnight-forest-184304/
- 発見 https://dova-s.jp/bgm/play20256.html
- (All Sound Effects) https://soundeffect-lab.info/
