# Tasks

## 1. DoR

- [x] 1.1 active OpenSpec change が `cross-tool-cli-benchmark` のみであることを確認する
- [x] 1.2 required CI gate に third-party tool installation を追加しないことを確認する
- [x] 1.3 `kml`、`mado`、`rumdl` の default comparison と common-subset comparison を別物として扱うことを確認する
- [x] 1.4 `fix` benchmark は source corpus を直接変更しない方針を確認する

## 2. Corpus And Report Contract

- [x] 2.1 clean corpus と diagnostics-heavy corpus の fixture directory を追加する
- [x] 2.2 fix benchmark 用の mutable workspace copy 方針を実装前にテスト可能な形で決める
- [x] 2.3 cross-tool report JSON schema を定義する
- [x] 2.4 Markdown summary の出力形式を定義する
- [x] 2.5 report に tool version、timing method、mode、enabled rules、skipped reason を含める

## 3. Harness Implementation

- [x] 3.1 `kml` release binary を準備して benchmark 対象にする
- [x] 3.2 `mado` binary discovery と missing-tool skip を実装する
- [x] 3.3 `rumdl` binary discovery と missing-tool skip を実装する
- [x] 3.4 diagnostics-heavy check の expected violation exit code normalization を実装する
- [x] 3.5 clean check の zero-exit validation を実装する
- [x] 3.6 fix workflow の per-run temporary workspace copy を実装する
- [x] 3.7 `hyperfine` が利用可能な場合は hyperfine runner を使う
- [x] 3.8 `hyperfine` が未導入の場合の fallback timing runner を実装する

## 4. Comparison Modes

- [x] 4.1 default comparison mode を実装する
- [x] 4.2 common-subset comparison mode を実装する
- [x] 4.3 common-subset candidate を `MD001,MD004,MD005,MD009,MD010,MD012,MD013,MD014,MD021,MD022,MD023,MD024,MD025,MD026,MD028,MD029,MD030,MD031,MD033,MD034,MD035,MD036,MD037,MD038,MD039,MD040,MD041,MD046,MD047` として定義する
- [x] 4.4 各 tool の config format に合わせた generated config を作成する
- [x] 4.5 unsupported rule または option を detected limitation として report する

## 5. Developer Interface

- [x] 5.1 `make bench-cross-tools` を追加する
- [x] 5.2 `make bench-cross-tools-default` を追加する
- [x] 5.3 `make bench-cross-tools-common` を追加する
- [x] 5.4 `make bench-cross-tools-fix` を追加する
- [x] 5.5 benchmark setup と optional tool installation を docs に記載する
- [x] 5.6 cross-tool benchmark が `make check` と required CI に含まれないことを確認する

## 6. Tests And Verification

- [x] 6.1 missing `mado` / `rumdl` が skipped case として report される unit test を追加する
- [x] 6.2 expected violation exit code normalization の unit test を追加する
- [x] 6.3 clean corpus non-zero exit を failure とする unit test を追加する
- [x] 6.4 fix benchmark が source corpus を変更しないことを確認する test を追加する
- [x] 6.5 report schema snapshot または fixture test を追加する
- [x] 6.6 `make bench-cross-tools` が `kml` case を実行し、missing optional tools を report することを確認する

## 7. DoD

- [x] 7.1 `openspec status --change cross-tool-cli-benchmark --json` で apply-ready である
- [x] 7.2 `make fmt-check` が成功する
- [x] 7.3 `make ast-lint` が成功する
- [x] 7.4 `cargo test --workspace` が成功する
- [x] 7.5 `make bench-cross-tools` が report を生成する
- [x] 7.6 OpenSpec main spec に delta を同期する
- [x] 7.7 active OpenSpec change が archive 可能な状態である
