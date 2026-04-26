# Tasks

## Definition of Ready

- [x] `v0.12.8` の release と release verification が完了している。
- [x] `v0.12.8` の stable score が 100 点である。
- [x] `v0.12.8` の technical hard blocker が 0 件である。
- [x] `v0.12.9` は精度、速度、安定性だけを扱う方針である。
- [x] 配布展開、MCP Registry、MCP package、remote MCP はこの change で進めない。
- [x] 進行中に task 外の高リスク不足が露見した場合だけ、作業を中断してユーザー判断を仰ぐ。

初期判断:

- KatanA `docs/` は Markdown 実文書 corpus として有効。
- KatanA `assets/` は binary asset ではなく、`assets/**/*.md` を lint corpus として使う。
- 既存 performance corpus は速度の継続比較に使う。
- KatanA corpus は required CI に直接依存させず、任意 external dogfood または curated fixture として扱う。
- 評価軸は「診断数が少ないこと」ではなく、実文書で結果を説明できること、壊さないこと、再実行で収束すること、説明不能に遅くならないことに置く。

評価対象:

| Command | 評価すること | release-blocking condition |
| --- | --- | --- |
| `check` | source file を書き換えないこと、diagnostic が true-positive / false-positive / false-negative に分類できること、未分類 high-risk finding が残らないこと | `check` が書き換える、または未分類の誤検知 / 検出漏れが残る |
| `fix` | default-safe fix だけを適用すること、link / image / inline HTML / code block / table / reference definition を壊さないこと、再実行で追加差分が出ないこと | unsafe fix 混入、構文破壊、再実行差分 |
| `fmt` | formatter policy の範囲だけを変更すること、Markdown の意味と link target を変えないこと、再実行で追加差分が出ないこと | 意味変更、link target 変更、再実行差分 |
| performance | KatanA corpus と既存 performance corpus の両方で check / fix / fmt の時間を説明できること | 説明不能な重大 regression |
| evidence | finding 分類、変更差分、残存 limitation、次版判断が tasks に残ること | 根拠なしの 100 点判定 |

## 0. Corpus Inventory

- [ ] KatanA `docs/**/*.md` の件数、サイズ、構文傾向を記録する。
- [ ] KatanA `assets/**/*.md` の件数、サイズ、構文傾向を記録する。
- [ ] 既存 `tests/fixtures/**` と performance benchmark case の coverage を棚卸しする。
- [ ] KatanA corpus と既存 performance corpus の役割を tasks に記録する。

## 1. External Dogfood Runner

- [ ] `KATANA_CHECKOUT` を受け取り、KatanA Markdown corpus を読み込む任意 dogfood target を設計する。
- [ ] required CI が sibling checkout に依存しないことを確認する。
- [ ] `check` は KatanA corpus を書き換えないことを確認する。
- [ ] dogfood report に source path、rule、severity、分類を残す。
- [ ] private content を public fixture にコピーしない guardrail を明記する。

## 2. Curated Public Confidence Fixture

- [ ] KatanA 由来として公開 repository に置ける subset を選ぶ、または synthetic equivalent を作る。
- [ ] link、image、inline HTML、fenced code、table、reference definition、Japanese / English mixed text を含める。
- [ ] `check` diagnostics の期待値を固定する。
- [ ] `fix` 後 content と remaining diagnostics を固定する。
- [ ] `fmt` 後 content と repeated `fmt` の無差分を固定する。

## 3. Precision Hardening

- [ ] external corpus finding を `true-positive`、`false-positive`、`false-negative`、`unsafe-fix-risk`、`fmt-policy-gap`、`perf-regression` に分類する。
- [ ] false-positive は rule-local test と document-level test の両方で固定する。
- [ ] false-negative は最小再現 test と corpus fixture の両方で固定する。
- [ ] unsafe-fix-risk は default-safe fix から除外するか collision guard を追加する。
- [ ] accepted limitation が残る場合は、公開導線を広げる前に許容できる理由を tasks に記録する。

## 4. Performance Hardening

- [ ] 実装前に `make perf-check` を実行し、baseline との差分を記録する。
- [ ] KatanA corpus または curated fixture を使った check / fix / fmt の実測を記録する。
- [ ] 既存 performance corpus と実文書 corpus のどちらで regress したかを分けて説明する。
- [ ] 重大 regression がある場合は release blocker として扱う。
- [ ] baseline refresh が必要な場合は、正しさの gate 後に理由付きで行う。

## 5. Release Confidence Score

- [ ] `v0.12.9` 用の public confidence score を定義する。
- [ ] external corpus confidence、precision regression、command convergence、performance stability、release reproducibility を採点する。
- [ ] score が 90 点未満、または hard blocker が 1 件以上の場合は `v0.13.0` へ進まない。
- [ ] score、hard blocker、known limitation、次版判断を tasks に記録する。

採点案:

| Category | Points | 評価対象 |
| --- | ---: | --- |
| External corpus confidence | 30 | KatanA corpus / curated fixture、finding 分類、release-blocking issue 0 件 |
| Precision regression | 25 | 誤検知、検出漏れ、rule-local / document-level test |
| Command convergence | 20 | `check` no-write、`fix` / `fmt` 冪等性、safe fix separation |
| Performance stability | 15 | existing perf corpus、external corpus timing、説明不能 regression なし |
| Release reproducibility | 10 | release-check、OS matrix、install / MCP / action smoke |

## 6. Release Preparation

- [ ] crate version を `0.12.9` に更新する。
- [ ] `CHANGELOG.md` に public confidence hardening を英語で記載する。
- [ ] OpenSpec delta を main specs に同期する。
- [ ] release 前に `make release-check VERSION=v0.12.9` を通す。
- [ ] 完了後に OpenSpec change を archive する。

## Verification

- [ ] `make fmt-check`
- [ ] `make lint`
- [ ] `make ast-lint`
- [ ] `cargo test --workspace --locked`
- [ ] `cargo test --workspace --all-features --locked`
- [ ] `make dogfood`
- [ ] external KatanA dogfood または curated public confidence fixture
- [ ] `make perf-check`
- [ ] `make release-check VERSION=v0.12.9`
- [ ] GitHub Actions required CI
- [ ] `git diff --check`

## Definition of Done

- [ ] KatanA docs / assets Markdown またはそれに相当する curated fixture の evidence が残っている。
- [ ] external corpus finding が分類済みで、release-blocking issue が 0 件である。
- [ ] `check` / `fix` / `fmt` の実運用寄り収束性が確認されている。
- [ ] performance regression がない、または説明と許容判断が tasks に残っている。
- [ ] `v0.13.0` へ進むかどうかのユーザー判断材料が揃っている。
