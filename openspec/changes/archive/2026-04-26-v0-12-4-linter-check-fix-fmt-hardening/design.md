# Design

## Goals

- `linter` / `check` / `fix` / `fmt` の中核契約を明確にする。
- 誤検知、検出漏れ、安全修正、整形の回帰をテストで固定する。
- 速度改善を、正しさの検証と同じ流れで扱う。
- 配布展開より前に、CLI 利用者が触る最初の体験を安定させる。

## Non-Goals

- MCP Registry や Hub への公開は行わない。
- unsafe fix の既定有効化は行わない。
- formatter を lint fix の実行器として扱わない。
- 速度数値だけを目的に rule semantics を変えない。

## Command Contract

### `check`

`check` は診断専用のコマンドとする。

- 対象 Markdown を lint する。
- ファイルを変更しない。
- 違反があれば既存方針どおり非 zero exit code を返す。
- text output と JSON output を混在させない。
- 設定解決、除外、stdin 入力の挙動を結合テストで固定する。

### `check --fix`

`check --fix` は check workflow に safe fix を組み込んだものとする。

- 初回診断を safe fix application に再利用する。
- 既定では default-safe fix だけを適用する。
- fix 後に再診断し、残存違反を報告する。
- unsafe fix、意味変更を伴う修正、仕様化されていない fallback は行わない。
- fix の衝突・重複・順序依存をテストで固定する。

### `fix`

`fix` は明示的な自動修正コマンドとする。

- default-safe fix だけを適用する。
- 適用済み修正と残存違反を報告する。
- 同じ入力に対して収束するまでの上限と、再実行時の冪等性を検証する。
- JSON output では applied fixes と diagnostics を機械可読に分離する。

### `fmt`

`fmt` は lint fix の別名ではなく、決定的なレイアウト整形コマンドとする。

- Markdown の意味や rule 適用状態を変える目的では使わない。
- 空行、末尾改行、リスト周辺、table 周辺など、formatter の責務範囲だけを整形する。
- 同じ入力に複数回実行しても出力が変わらない。
- `--stdin` / stdout 利用時は editor integration に適した出力契約を維持する。
- 未解決の lint 違反があっても、整形自体が成功した場合は formatter としての成功を返す。

## Issue Inventory

実装開始時に、課題を次の粒度で棚卸しする。

- `bug`: 誤検知、検出漏れ、誤修正、formatter の破壊的変更
- `test-gap`: 実装はありそうだが、再発防止の fixture が不足しているもの
- `perf-risk`: 高負荷文書、directory traversal、config resolution、fix/fmt 繰り返しで遅くなるもの
- `design-debt`: parser 共有化や大きめの設計変更が必要で、patch release に混ぜるべきでないもの
- `ci-gap`: CI や release gate の責務不足

棚卸し結果は tasks に記録し、`v0.12.4` で対応するものと後続 change に送るものを分ける。

## Test Strategy

### Unit Tests

- rule-local test で最小再現を固定する。
- fix candidate の重複、隣接、競合、順序依存を検証する。
- formatter の範囲ごとに入力と期待出力を固定する。
- config 解決と rule enable / disable の境界を確認する。

### Integration Tests

- CLI の `check` / `check --fix` / `fix` / `fmt` を実ファイルで検証する。
- `--format json` / `--output json` で stdout payload が混在しないことを確認する。
- stdin/stdout 利用時の editor-friendly behavior を確認する。
- dogfood 対象の Markdown で中核コマンドの使い勝手を確認する。

### Corpus Tests

複数構文が混ざる Markdown fixture を用意し、次を検証する。

- code block、inline code、HTML block、table、reference link を通常本文として誤診断しない。
- fix 後に不要な差分が増えない。
- fmt 後に lint fix 相当の意味変更が起きない。
- fix / fmt の再実行で差分が増えない。

## Performance Strategy

- 実装前に `make perf-check` または同等の benchmark を記録する。
- `check`、`fix`、`fmt`、directory check、diagnostics-heavy corpus を代表経路に含める。
- wall-clock 数値は release gate の絶対条件にせず、前後比較と異常検知の証拠として扱う。
- 速度改善後は diagnostics、fixed content、formatted content、exit code、JSON shape を確認してから baseline を更新する。

## Release Readiness

`v0.12.4` の release 前に次を確認する。

- `check` は no-write contract を守る。
- `fix` と `check --fix` は safe fix contract を守る。
- `fmt` は formatter contract を守り、lint fix の別名に戻っていない。
- dogfood、unit test、integration test、performance check が通っている。
- changelog は `v0.12.4` の中核改善として整理されている。
