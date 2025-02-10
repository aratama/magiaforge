# API

`yield` で発行するコマンドについては、cmd.rs の Cmd を参照してください。
以下はスクリプトのグローバルスコープに公開されるもので、
デフォルトでは読み取り専用です。

## actorPosition: { x: number, y: number }

スクリプト起動のトリガーとなったアクターの位置。

## console.log(arg: any): void

ログを出力します

## discoveries: string[]

現在までに発見済みの呪文一覧です

## inventory: string[]

プレイヤーのインベントリ。呪文名が文字列の配列で格納されます。

## spellListOpen: boolean

HUD の発見済みの呪文一覧が開いているかどうかを設定します。
このプロパティには書き込みが可能です。
