## Why

`kml check` の診断内容と CLI error は現在すべて英語の固定文で表示される。
`katana-markdown-linter` は library-first だが、CLI は人間が直接読む導線でもあるため、OS の言語設定に沿った表示ができないと日本語環境での導入障壁が残る。

この change は最優先対応事項として、check 実行時の user-facing message を locale-aware にする。

## What Changes

- `kml check` に locale 指定 option `--local <locale>` と shorthand `-l <locale>` を追加する
- `--local` が省略された場合は OS default locale を使用する
- OS default locale が取得できない、または未対応 locale の場合は English に fallback する
- 最初の対応 locale は English と Japanese とする
- text output の lint diagnostics、config errors、filesystem errors、summary/fix status を locale-aware にする
- internal rule id、file path、line/column、exit code、JSON schema の互換性は維持する
- translation coverage が不足している message は English fallback で表示する

## Impact

- 日本語 OS 環境では `kml check` の人間向け message が日本語になる
- CI や automation で English 固定にしたい場合は `kml check --local en ...` を指定できる
- library API は generic linter boundary を維持し、consumer-specific UI 文言を持たない
- future locale 追加時の message catalog と fallback policy が明確になる

## User Decisions

- option name は `--local`
- shorthand は `-l`
- option 省略時は OS default locale を使う

## Non-Goals

- Markdown source content の翻訳
- rule id、alias、file path、line/column の翻訳
- consumer app の Problems panel や editor UI 文言の翻訳
- full ICU message formatting の導入
- English 以外の全 locale を初回で網羅すること
