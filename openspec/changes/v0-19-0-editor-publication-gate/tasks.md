# Tasks

## 目標

- `v0.18.7` の再公開不可事故を明文化し、`v0.18` 系の bump ルールを固定する。
- `v0.19.0` 進行条件を、実際の marketplace 公開実行がある場合に限定する。
- release gate に「同一 version 再publish拒否」「manual publish guard」「published/deferred説明」を入れる。
- 変更前に kml 自己検査（dogfood）で自己整合性を確認する。

## Definition of Ready（着手条件）

- [ ] 0.1 `v0.18.7` が再公開不可事故として明文化され、`v0.18.7` の公開不可理由と対象チャネル（npm / crates.io / PyPI / GitHub Release / Homebrew）が確認できること
- [ ] 0.2 `verify-release-target` / `release-check` / `release-verify` の対象範囲が本 change の Scope 内で一致していること
- [ ] 0.3 dogfood 実行に必要な前提（`DOGFOOD_TARGETS`, `KML`, ベースラインファイル）が確定していること
- [ ] 0.4 v0.19.0 を許容する marketplace 条件（account / publisher / package / token / verification）を 1 箇所に集約できていること
- [ ] 0.5 進行評価で使う runbook への保存先（`/tmp` 等）と更新責任者が決まっていること
- [ ] 0.6 proposal / design / tasks / spec の中核用語（再公開不可、Go/No-Go、dogfood 条件）が同一定義でそろっていること

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
- [ ] 3.3 `v0.19.0` 判定を行うタイミングと評価資料（release-check 結果 + 実装差分）を記録する

## 4. Evidence

- [ ] 4.1 `release-readiness` の再現手順（1ページ）を作る
- [ ] 4.2 `v0.18.7` の扱いを roadmap / release runbook として共有する
- [ ] 4.3 次の release から適用する version policy を変更履歴に残す
- [ ] 4.4 release-check / release-verify 実行結果と dogfood 結果を同一証跡（同じ runbook）に紐づける
- [ ] 4.5 proposal / design / tasks / spec の整合性差分を 1 つの evidence に残す

## 5. Dogfood 実行（task0）

- [ ] 5.1 `just dogfood` を実行し、`target/dogfood-report.json` を生成する
- [ ] 5.2 `cat target/dogfood-report.json` で新規・増加がないことを確認する
- [ ] 5.3 変更意図がある場合のみ `just dogfood-refresh-baseline` を実行し、`tests/fixtures/dogfood-baseline.json` を更新する
- [ ] 5.4 `just dogfood-json` を使い、必要なら出力分類（rule / severity / source）を evidence 化する
- [ ] 5.5 自己検査で増分がある場合は、増分理由と判断を `release-readiness` ページに書く

## Definition of Done（完了条件）

- [ ] D1 5.0 の task0 を release-readiness の前提として実行し、`target/dogfood-report.json` と `tests/fixtures/dogfood-baseline.json` が証跡として残っていること
- [ ] D2 `v0.18.7` 再公開不可・既存版検知・manual publish guard が design/spec/tasks の 3 本立てで矛盾なく表現されていること
- [ ] D3 v0.18.x patch と v0.19.0 判定条件の紐づけが 1 箇所に集約され、`0.19` 進行ロジックで説明可能になっていること
- [ ] D4 `release-check` / `release-verify` の結果により、v0.19.0 の Go / No-Go が再現可能であること
- [ ] D5 変更が roadmap / runbook / evidence に反映され、未反映項目がないこと
