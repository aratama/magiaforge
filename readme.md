# magiaforge

Online Battle Royale Twin-stick Shooter. WIP.

![](misc/screenshot_title.png)

![](misc/screenshot.png)

### Demo

[![Release](https://github.com/aratama/magiaforge/actions/workflows/release.yaml/badge.svg)](https://github.com/aratama/magiaforge/actions/workflows/release.yaml)

https://magiaforge.app/ (**Desktop Chrome only** for now)

### Development

- `cargo run` to launch app locally
- `cargo build --profile dist` to build desktop app in release mode. (Note `cargo build` is for WASM build)
- `trunk serve` to run locally on browser with [trunk](https://trunkrs.dev/)
- `trunk build` to build web app and publish on GitHub Pages

Add `--features debug` to launch app in debug mode.

### Save Data Location

On Windows, save data is stored in `C:\Users\<USERNAME>\AppData\Roaming\magiaforge`.

In browsers, save data is stored in local storage under the key `magiaforge.magiaforgeconfig`.

### Asset credits

#### Sound Effects

- 芝生の上を歩く https://soundeffect-lab.info/sound/various/
- ひよこの鳴き声 https://soundeffect-lab.info/sound/animal/
- 打撃 1 https://soundeffect-lab.info/sound/battle/
- パンチ素振り https://soundeffect-lab.info/sound/battle/
- 革靴で歩く https://soundeffect-lab.info/sound/various/
- アスファルトの上を歩く 2 https://soundeffect-lab.info/sound/various/
- 決定ボタンを押す 48 https://soundeffect-lab.info/sound/button/
- 建物が少し崩れる 1 https://soundeffect-lab.info/sound/battle/
- メニューを開く 2 https://soundeffect-lab.info/sound/battle/
- ワープ https://soundeffect-lab.info/sound/battle/
- 体育館で走る https://soundeffect-lab.info/sound/various/
- 回復魔法 1 https://soundeffect-lab.info/sound/battle/
- カーソル移動 2 https://soundeffect-lab.info/sound/button/
- 爆発 3 https://soundeffect-lab.info/sound/battle/battle2.html

#### BGMs

- They https://dova-s.jp/bgm/play21009.html
- God's realm https://dova-s.jp/bgm/play20967.html
- 茫漠たる庭 https://dova-s.jp/bgm/play21154.html
- 荒れ地の先へ https://dova-s.jp/bgm/play17195.html
- 悪魔との戦闘 https://dova-s.jp/bgm/play11175.html

#### Fonts

- DotGothic16 https://fonts.google.com/share?selection.family=DotGothic16
