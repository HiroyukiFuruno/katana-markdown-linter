# Tasks

## 目標

- `v0.18.7` の再公開不可事故を明文化し、`v0.18` 系の bump ルールを固定する。
- `v0.19.0` 進行条件を、実際の marketplace 公開実行がある場合に限定する。
- release gate に「同一 version 再publish拒否」「manual publish guard」「published/deferred説明」を入れる。
- 変更前に kml 自己検査（dogfood）で自己整合性を確認する。

## 1. Releaseバージョンポリシー（準備）

- [ ] 1.1 `v0.18.7` を「失敗版」かつ再公開不可として記録する
- [ ] 1.2 `v0.18.x` patch bump の前提を「check/fix/format の報告ベース bugfix」のみに固定する
- [ ] 1.3 `release` 失敗時の停止条件（既存版検知）を release ルールに反映する

## 2. Gateの明文化

- [ ] 2.1 `verify-release-target` で既存チャネル公開済み版の判定を再現可能な形で固定する
- [ ] 2.2 `release-check` で上記ガードを明示的に実行する
- [ ] 2.3 `release-verify` が published / deferred の両状態を説明できること
- [ ] 2.4 marketplace publish 進行前提条件未達時は fail-fast を仕様化する

## 3. v0.19.0 Go/No-Go

- [ ] 3.1 `v0.19.0` の対象条件（VS Code / Zed marketplace 公開の実行が入る）を 1 箇所に集約する
- [ ] 3.2 条件不達時は `v0.18.z` で継続し、`v0.19.0` に進まないことを明示する

## 4. Evidence

- [ ] 4.1 release-readiness の再現手順（1ページ）を作る
- [ ] 4.2 `v0.18.7` の扱いを roadmap / release runbook として共有する
- [ ] 4.3 次の release から適用する version policy を変更履歴に残す

## 5. Dogfood 実行（task0）

- [ ] 5.1 `just dogfood` を実行し、`target/dogfood-report.json` を生成する
- [ ] 5.2 `cat target/dogfood-report.json` で新規・増加がないことを確認する
- [ ] 5.3 変更意図がある場合のみ `just dogfood-refresh-baseline` を実行し、`tests/fixtures/dogfood-baseline.json` を更新する
- [ ] 5.4 `just dogfood-json` を使い、必要なら出力分類（rule / severity / source）を evidence 化する
