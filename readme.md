# magiaboost

![screenshot](screenshot.png)

### Usage

To run locally (as native) with [cargo-watch](https://github.com/watchexec/cargo-watch):

```bash
$ cargo install cargo-watch --locked

$ cargo watch -x 'run'
```

To run locally on browser with [trunk](https://trunkrs.dev/):

```bash
$ rustup target install wasm32-unknown-unknown

$ cargo install --locked trunk

$ trunk serve --features debug
```

To build and publish on GitHub Pages:

```bash
$ trunk build
```

### Demo

https://aratama.github.io/bevy-wasm-example/ (**Desktop Chrome only** for now)

### Notes

- https://trunkrs.dev/
- https://bevy-cheatbook.github.io/platforms/wasm.html
- https://gist.github.com/nakedible/f6a0d4bcbea1df7768e9ed425f6f33db
- Linux 用 Windows サブシステム で Linux GUI アプリを実行する https://learn.microsoft.com/ja-jp/windows/wsl/tutorials/gui-apps
- https://github.com/jgayfer/bevy_light_2d
- https://github.com/zaycev/bevy-magic-light-2d
- https://github.com/rustwasm/wasm-pack/issues/1434 Windows で trunk 0.21.1 をインストールしようとするとビルドエラーになる
- https://cloud.google.com/blog/products/serverless/cloud-run-gets-websockets-http-2-and-grpc-bidirectional-streams?hl=en

### Asset credits

- 芝生の上を歩く https://soundeffect-lab.info/sound/various/
- ひよこの鳴き声 https://soundeffect-lab.info/sound/animal/
- 打撃 1 https://soundeffect-lab.info/sound/battle/
- 決定ボタンを押す 48 https://soundeffect-lab.info/sound/button/
- They https://dova-s.jp/bgm/play21009.html
- God's realm https://dova-s.jp/bgm/play20967.html
- 建物が少し崩れる 1 https://soundeffect-lab.info/sound/battle/
- DotGothic16 https://fonts.google.com/share?selection.family=DotGothic16
