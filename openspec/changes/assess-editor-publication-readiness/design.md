# Editor Publication Readiness Triage Design

## 対象

- `v0.18.7` 永久欠番と `v0.18.8` 不使用の判断整理。
- VS Code / Zed 拡張の現状調査。
- Marketplace 公開 No-Go の理由整理。
- 後続 OpenSpec change の分割タスク化。

## 非対象

- `v0.19.0` 系の editor 機能実装。
- `v0.20.0` の Marketplace 公開実装。
- VS Code / Zed 拡張の機能追加。
- release 実行、tag 作成、公開処理。

## 調査結果

1. VS Code 拡張
   - `kml.executablePath` から `kml lsp` を起動する。
   - Markdown ファイルへ診断・整形・安全な修正候補を出せる。
   - ただし LSP 側が project config を解決していないため、`.markdownlint.json` / `.markdownlint.jsonc` の設定を反映できない。

2. Zed 拡張
   - Zed の language server extension 境界から `kml lsp` を起動する。
   - LSP が返す診断・整形・安全な修正候補に依存する。
   - VS Code と同じく、LSP 側の project config 未解決が公開 blocker になる。

3. LSP
   - 現状の診断・修正は `LintOptions::default()` に依存している。
   - `kml check` / `kml fix` と同等に config を探索・解決する経路がない。
   - linter として、設定通りのコード上表示とそこからの安全な修正ができない状態は公開不可。

## 判断

- この change は release version を持つ実装計画ではなく、調査結果を後続タスクへ変換するための change とする。
- `v0.19.0` 系は、別 change で editor 機能不足を解消する。
- `v0.20.0` は、さらに別 change で Marketplace 公開準備・公開検証を扱う。
- `v0.20.0` 公開判断には、最終 dogfood の evidence を必須にする。

## 後続changeの分割方針

- `v0.19.0` 系 change: LSP config 解決、設定反映済み診断、整形、安全な修正候補、editor integration 検証。
- `v0.19.0` 最終検証 change: 実リポジトリでの editor dogfood、誤診断・誤修正・設定反映漏れの回帰確認。
- `v0.20.0` change: VS Code / Zed Marketplace 公開、公開前提条件、公開後 verification。
