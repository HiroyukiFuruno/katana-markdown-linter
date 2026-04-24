# Tasks

## DoR

- [x] `openspec list --json` でこの change の前提となる未完了作業を確認する
- [x] v0.2.0 release 済みであることを確認し、この change が release flow 修正を主目的にしないことを確認する
- [x] `kml` の現在の CLI entrypoint と library API の公開範囲を確認する
- [x] archived OpenSpec documents を既定の修正対象に含めるかを design の方針どおり確認する

## Implementation

- [x] dogfood 対象と除外対象を定義する
- [x] `make dogfood` 相当の check-only target を追加する
- [x] `make dogfood-fix` 相当の明示 fix target を追加する
- [x] dogfood 結果を記録する report file を追加する
- [x] 初回 dogfood check を実行し、違反を report に分類する
- [x] safe fix 可能な Markdown だけを修正する
- [x] CLI usability finding の記録形式を追加する
- [x] Rust public API example を追加する
- [x] public API example を compile 対象に含める
- [x] README から CLI usage と library usage の導線を整理する

## DoD

- [x] `make dogfood` が repository の既定 Markdown target に対して実行できる
- [x] `make dogfood-fix` が check-only target と分離されている
- [x] `openspec/changes/archive/**` が既定の自動修正対象から外れている
- [x] dogfood report に初回結果、既知の除外、未対応 findings が記録されている
- [x] Rust embedding examples が compile される
- [x] README から CLI と library API の両方に到達できる
- [x] `openspec status --change dogfood-cli-and-api-usage --json` で apply-ready である
