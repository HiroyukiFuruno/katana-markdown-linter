## Definition of Ready

- [x] `check-diagnostic-i18n` が archive 済み、または全 task 完了済みであること
- [x] `golden-edge-coverage-expansion` が完了済み、またはこの change で触る rule の golden/edge gap が明示されていること
- [x] fixture matrix の `manual_required` が現状と一致していること
- [x] `mdxxx.rs` の責務を check/fix に限定する設計方針が維持されていること
- [x] safe subset と unsafe subset を rule ごとに fixture 名で区別できること
- [x] `--unsafe` / interactive confirmation は `unsafe-fix-mode-and-confirmation` に分離済みであること

## 1. Fix Strategy Infrastructure

- [x] 1.1 diagnostics から fix candidates を rule id / range / block type ごとに分類する
- [x] 1.2 independent fix と block strategy fix の適用順序を定義する
- [x] 1.3 overlap skip と strategy merge の違いを unit test で固定する
- [x] 1.4 CLI fix の反復適用が strategy fix 後も収束することを確認する

## 2. List Strategy

- [x] 2.1 `MD005` の safe subset fixture を追加する
- [x] 2.2 `MD030` の safe subset fixture を追加する
- [x] 2.3 `MD005` / `MD007` / `MD030` の conflict case を追加する
- [x] 2.4 safe subset のみ fix_info または strategy fix を返す
- [x] 2.5 unsafe subset は `manual_required` に残す

## 3. Table Strategy

- [x] 3.1 `MD060` は official `fixable: false` として strategy layer 対象外にする
- [x] 3.2 `MD060` の explicit style config を fixture 化する
- [x] 3.3 `MD060` は before/after fix fixture ではなく manual-required として残す
- [x] 3.4 compact/spaced table edge fixture を追加する
- [x] 3.5 `MD055` / `MD056` / `MD058` との conflict は unsafe fix mode 側へ送る

## 4. Matrix And Dashboard

- [x] 4.1 newly safe な fix を `tests/fixtures/rule-fixture-matrix.json` に反映する
- [x] 4.2 `manual_required` から解消済み理由を削除する
- [x] 4.3 `docs/rule-coverage-dashboard.md` を更新する
- [x] 4.4 未解消の `manual_required` は rule id、unsafe 理由、次の解消条件を含むこと
- [x] 4.5 `v0.4.0` の dashboard は check / safe fix / manual-required unsafe candidate を区別して表示する

## Verification

- [x] `cargo test --workspace` が成功する
- [x] `make check` が成功する
- [x] `make bench-cross-tools-fix` が available tools で成功または skip 理由を出す
- [x] dogfood fix が source fixture を破壊しない
- [x] default `kml fix` が unsafe candidate を適用しないことを fixture で確認する
- [x] `git diff --check` が成功する

## Definition of Done

- [x] `MD005`, `MD030` の safe fix subset が fixture で固定されていること
- [x] unsafe subset が diagnostic-only または `manual_required` として明示されていること
- [x] fix strategy と rule local fix の責務境界が崩れていないこと
- [x] fix 対応数が増えない場合でも、その理由が rule ごとに検証可能であること
- [x] `v0.5.0` 向け unsafe fix mode と混線せず、default safe behavior が維持されていること
