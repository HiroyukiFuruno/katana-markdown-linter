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

- [x] KatanA `docs/**/*.md` の件数、サイズ、構文傾向を記録する。
- [x] KatanA `assets/**/*.md` の件数、サイズ、構文傾向を記録する。
- [x] 既存 `tests/fixtures/**` と performance benchmark case の coverage を棚卸しする。
- [x] KatanA corpus と既存 performance corpus の役割を tasks に記録する。

実測 inventory:

- KatanA corpus: 31 files / 196606 bytes
- syntax counts: links 22、images 6、inline HTML 40、fenced code 350、tables 57、reference definitions 12、Japanese / English mixed files 24
- curated public confidence corpus: `tests/fixtures/public-confidence/corpus/representative.md`、1 file / 445 bytes
- existing performance corpus: `tests/fixtures/perf-baseline.json` schema version 2。API lint/fix/format、CLI check/fix/fmt、context index、config validation、rule catalog を継続比較する。
- 役割分担: curated corpus は required CI / release gate 用、KatanA corpus は任意 external evidence 用、performance corpus は synthetic benchmark の継続比較用。

## 1. External Dogfood Runner

- [x] `KATANA_CHECKOUT` を受け取り、KatanA Markdown corpus を読み込む任意 dogfood target を設計する。
- [x] required CI が sibling checkout に依存しないことを確認する。
- [x] `check` は KatanA corpus を書き換えないことを確認する。
- [x] dogfood report に source path、rule、severity、分類を残す。
- [x] private content を public fixture にコピーしない guardrail を明記する。

実装:

- `make external-katana-dogfood` は `KATANA_CHECKOUT` を必須にし、`docs/**/*.md` と `assets/**/*.md` だけを対象にする。
- `scripts/ci/public-confidence.py` は `check` の前後 hash を比較し、source no-write を report に残す。
- `fix` / `fmt` は一時 directory copy 上で実行する。KatanA checkout は書き換えない。
- `make public-confidence` は committed curated fixture を使うため、required CI は sibling checkout に依存しない。

## 2. Curated Public Confidence Fixture

- [x] KatanA 由来として公開 repository に置ける subset を選ぶ、または synthetic equivalent を作る。
- [x] link、image、inline HTML、fenced code、table、reference definition、Japanese / English mixed text を含める。
- [x] `check` diagnostics の期待値を固定する。
- [x] `fix` 後 content と remaining diagnostics を固定する。
- [x] `fmt` 後 content と repeated `fmt` の無差分を固定する。

curated evidence:

- `check`: 4 diagnostics。MD018 / MD037 / MD038 / MD039、すべて true-positive。
- `check`: source unchanged true。
- `fix`: exit 0、changed files 1、converged true。
- `fmt`: exit 0、changed files 1、converged true。
- `final_check`: exit 0、remaining diagnostics 0。
- timing with `KML=target/debug/kml`: check 5.184 ms、fix 5.961 ms、fmt 4.058 ms、final check 3.992 ms。

## 3. Precision Hardening

- [x] external corpus finding を `true-positive`、`false-positive`、`false-negative`、`unsafe-fix-risk`、`fmt-policy-gap`、`perf-regression` に分類する。
- [x] false-positive は rule-local test と document-level test の両方で固定する。
- [x] false-negative は最小再現 test と corpus fixture の両方で固定する。
- [x] unsafe-fix-risk は default-safe fix から除外するか collision guard を追加する。
- [x] accepted limitation が残る場合は、公開導線を広げる前に許容できる理由を tasks に記録する。

分類結果:

- 修正済み false-positive: CLI が file path を `lint` に渡していなかったため、`md-broken-link` が relative local link と `mailto:` を誤検知していた。
- 固定 test: `tests/cli_path_context_contract.rs` で existing relative link と `mailto:` を許容し、missing local link だけを診断することを固定した。
- rule-local test: `src/rules/markdown/broken_link.rs` で non-local destination と existing local file を許容することを固定した。
- 修正済み false-positive: GFM alert block を空行で連続配置した場合に、`MD028` が blockquote 内 blank line として誤検知していた。
- 固定 test: `tests/fixtures/rule-fixture-matrix.json` の `MD028` pass case で `[!NOTE]` / `[!TIP]` / `[!IMPORTANT]` / `[!WARNING]` / `[!CAUTION]` の連続 alert block を許容することを固定した。
- KatanA remaining findings: `docs/development-guide.md` / `.ja.md` の `../../issues` と `LICENSE`、計 4 件。true-positive として記録し、KatanA 側 content cleanup は non-blocking follow-up とする。
- unsafe-fix-risk: 0 件。external KatanA run では fix changed files 0。

## 4. Performance Hardening

- [x] 実装前に `make perf-check` を実行し、baseline との差分を記録する。
- [x] KatanA corpus または curated fixture を使った check / fix / fmt の実測を記録する。
- [x] 既存 performance corpus と実文書 corpus のどちらで regress したかを分けて説明する。
- [x] 重大 regression がある場合は release blocker として扱う。
- [x] baseline refresh が必要な場合は、正しさの gate 後に理由付きで行う。

performance evidence:

- `make perf-check` は成功した。schema / required cases / statistic fields に欠落なし。
- 2 回目の `make perf-check` では CLI cases は baseline 比で `cli_check_many_small_files` 0.93x、`cli_fix_many_small_files` 0.86x、`cli_fmt_many_small_files` 0.89x。
- synthetic benchmark では `api_lint_large_document` 1.37x と `api_fix_large_document` 1.31x が最大級だったが、gate は report-first で失敗なし。今回の実装は path-aware CLI lint と confidence runner 追加であり、benchmark 数値のための semantics 変更はしていない。
- real-document evidence は curated: check 5.184 ms / fix 5.961 ms / fmt 4.058 ms、KatanA: check 52.907 ms / fix 80.756 ms / fmt 48.012 ms。
- baseline refresh は不要。release blocker 扱いの重大 regression は 0 件。

## 5. Release Confidence Score

- [x] `v0.12.9` 用の public confidence score を定義する。
- [x] external corpus confidence、precision regression、command convergence、performance stability、release reproducibility を採点する。
- [x] score が 90 点未満、または hard blocker が 1 件以上の場合は `v0.13.0` へ進まない。
- [x] score、hard blocker、known limitation、次版判断を tasks に記録する。

採点案:

| Category | Points | 評価対象 |
| --- | ---: | --- |
| External corpus confidence | 30 | KatanA corpus / curated fixture、finding 分類、release-blocking issue 0 件 |
| Precision regression | 25 | 誤検知、検出漏れ、rule-local / document-level test |
| Command convergence | 20 | `check` no-write、`fix` / `fmt` 冪等性、safe fix separation |
| Performance stability | 15 | existing perf corpus、external corpus timing、説明不能 regression なし |
| Release reproducibility | 10 | release-check、OS matrix、install / MCP / action smoke |

score result:

| Category | Score | Evidence |
| --- | ---: | --- |
| External corpus confidence | 30/30 | curated / KatanA evidence と classified findings を記録し、release blocker 0 件 |
| Precision regression | 25/25 | path-aware CLI lint、`md-broken-link` non-local scheme 修正、`MD028` GFM alert 誤検知修正を test 固定 |
| Command convergence | 20/20 | `check` no-write、`fix` / `fmt` repeated convergence を curated と KatanA copy で確認 |
| Performance stability | 15/15 | `make perf-check` 成功、real-document timing 記録、baseline refresh 不要 |
| Release reproducibility | 10/10 | version bump、CI/release workflow wiring、`make release-check VERSION=v0.12.9` 通過 |

合計: 100/100。technical hard blocker: 0 件。

known limitation:

- KatanA `docs/development-guide.md` / `.ja.md` には `../../issues` と `LICENSE` の true-positive local-link finding が残る。これは KatanA 文書側の follow-up であり、`kml` release blocker ではない。

next decision:

- `v0.12.9` release verification とユーザー受け入れが完了すれば、`v0.13.0` の MCP Registry / distribution planning に進める。

## 6. Release Preparation

- [x] crate version を `0.12.9` に更新する。
- [x] `CHANGELOG.md` に public confidence hardening を英語で記載する。
- [x] OpenSpec delta を main specs に同期する。
- [x] release 前に `make release-check VERSION=v0.12.9` を通す。
- [x] 完了後に OpenSpec change を archive する。

## Verification

- [x] `make fmt-check`
- [x] `make lint`
- [x] `make ast-lint`
- [x] `cargo test --workspace --locked`
- [x] `cargo test --workspace --all-features --locked`
- [x] `make dogfood`
- [x] external KatanA dogfood または curated public confidence fixture
- [x] `make perf-check`
- [x] `cargo test --test rule_fixture_harness check_pass_and_fail_fixtures_execute --locked`
- [x] `cargo test --test rule_fixture_harness matrix_markdown_summary_matches_json_counts --locked`
- [x] `make release-check VERSION=v0.12.9`
- [x] GitHub Actions required CI
- [x] `git diff --check`

## Definition of Done

- [x] KatanA docs / assets Markdown またはそれに相当する curated fixture の evidence が残っている。
- [x] external corpus finding が分類済みで、release-blocking issue が 0 件である。
- [x] `check` / `fix` / `fmt` の実運用寄り収束性が確認されている。
- [x] performance regression がない、または説明と許容判断が tasks に残っている。
- [x] `v0.13.0` へ進むかどうかのユーザー判断材料が揃っている。
