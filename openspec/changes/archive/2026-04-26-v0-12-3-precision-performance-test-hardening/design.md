## Design

`v0.12.3` は 3 つの作業軸で進める。

1. 精度強化（Precision hardening）
2. 速度強化（Performance hardening）
3. テスト強化（Test hardening）

## Precision Hardening

最初に誤検知と検出漏れを棚卸しする。

分類は次を使う。

- `bug`: 実際の誤検知、検出漏れ、誤修正
- `test-gap`: 実装は妥当だが再発防止の fixture がない
- `design-debt`: 単一行判定や ad hoc parse に依存している
- `perf-risk`: 精度改善で走査回数や allocation が増え得る

優先対象は `v0.12.2` で後続送りにしたインラインコード（inline code）/ HTML / link / command prompt 周辺、特に `MD034` と `MD059` の行単位（line-local）判定である。
修正は必ずルール単位（rule-local）の最小再現と、複数構文を混ぜた文書単位の固定テスト（document-level fixture）の両方で固定する。

## Performance Hardening

速度改善は測定から始める。

実装前に `make bench` または `make perf-check` で基準値（baseline）を取り、次の観点で高負荷経路（hot path）を選ぶ。

- directory traversal と ignore handling
- 設定探索（config discovery）/ 検証（validation）
- `DocumentContext` 構築
- rule dispatch と rule-local repeated scan
- JSON / text reporting
- fix application と post-fix validation

CI は時計時間の閾値（wall-clock threshold）に依存しない。
ただし、基準値更新（baseline refresh）を行う場合は、変更理由、対象 case、before / after、正しさの検査（correctness gate）を tasks に記録する。

## Test Hardening

単体テスト（UT）は小さい再現条件を保つ。

- rule-local behavior
- config option の境界値
- path normalization と Windows short path / verbatim path
- fix idempotence
- formatter / fix collision

結合テスト（IT）は実際の CLI や workflow に近い単位を扱う。

- CLI workspace traversal
- clean / dirty corpus の exit code
- `--fix` が source corpus を壊さないこと
- 自己適用検査（dogfood）の基準値（baseline）
- 複数ツール比較ベンチマーク（cross-tool benchmark）script の failure normalization
- Windows CI でしか再現しない shell / path / suffix 差分

テストのためだけに商用コードを曲げない。
必要な接続点が見つかった場合は、仕様上の責務として設計に戻す。

## Release Gate

完了条件は、少なくとも次の gate を満たすこと。

- `make fmt-check`
- `make lint`
- `make ast-lint`
- `cargo test --workspace --locked`
- `cargo test --all-features --locked`
- `make dogfood`
- `make perf-check`
- `make release-check VERSION=v0.12.3`
- GitHub Actions の Ubuntu / macOS / Windows CI

## Non-Goals

- 新機能としての UI / editor integration。
- Registry package metadata 実装。
- unsafe fix の対象拡大。
- 速度改善（performance）のための互換性低下。
