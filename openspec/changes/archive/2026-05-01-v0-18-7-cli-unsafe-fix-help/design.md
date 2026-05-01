# v0.18.7 CLI unsafe fix help design

## 方針

既存の修正実行経路は維持し、CLI help とテスト契約だけを補強する。

`check --fix` は `fix_mode=true` で `run_check_like` を通り、`resolve_unsafe_fix_policy` と `apply_fixes_until_stable` を共有している。
そのため `--unsafe --yes` の処理を新設せず、既存経路が `check --fix` でも使えることをテストで固定する。

## CLI help

`kml check --help` の options に `--unsafe --yes` を追加する。
global help は `fix command` 限定の表現をやめ、`fix or check --fix` として説明する。

日本語 help も同じ内容に更新する。

## 回帰テスト

`MD036` は unsafe fix を持つ既存ルールなので、`**Important**` を `# Important` に変換する fixture で確認する。

確認する契約:

- `kml check --fix --unsafe --yes --output json` が成功する
- file content が unsafe fix 後の内容になる
- JSON の `command` は `check` のまま
- `fix_details` に `MD036` が記録される
- `kml check --help` に `--unsafe --yes` が表示される

## リリース判断

これは既存機能の入口説明と契約テストの欠落修正であり、挙動追加ではない。
ただし CLI の安全性に関わる help 契約のため、`v0.18.7` の通常リリースとして扱う。
