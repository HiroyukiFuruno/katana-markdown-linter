## 0. 前提確認

- [x] 0.1 `v0.12.20` が release 済みで、`api_rule_catalog` cache 後の performance blocker が残っていないことを確認する
- [x] 0.2 `v0.12.19` の結果を確認し、`MD028` が safe-fix 実装済みか by-design 候補かを確定する
- [x] 0.3 KatanA checkout の場所を確認し、`KATANA_CHECKOUT` を使う実行方法を決める
- [x] 0.4 予約領域 directory の既定除外と明示 opt-in 方針を `v0.12.21` の実装対象として確定する

## 1. CLI traversal default exclusions

- [x] 1.1 `node_modules`、`.git`、build/cache output など通常 git 管理しない directory を既定の再帰走査対象外にする仕様を確定する
- [x] 1.2 明示 opt-in option で予約領域も対象にできる CLI 契約を定義する
- [x] 1.3 `check`、`fix`、`fmt` の directory traversal regression test を先に追加する
- [x] 1.4 production code を修正し、既定では予約領域配下の Markdown を読まず、書き換えないことを確認する
- [x] 1.5 opt-in 実行時だけ予約領域配下の Markdown が対象になることを確認する
- [x] 1.6 `fix --output json` と `check --fix --output json` が file ごとの applied fix detail を出力する regression test を追加する
- [x] 1.7 gitignore 済み directory を明示 input として渡した場合だけ対象へ戻せる scoped opt-in を追加する

## 2. KatanA feedback sweep

- [x] 2.1 KatanA 側の対象 Markdown 文書群を確認する
- [x] 2.2 `/Users/hiroyuki_furuno/works/private/katana` から `/tmp` 配下へ git worktree を作成し、local branch `verify` を切る
- [x] 2.3 元 checkout にある git 管理外 Markdown があれば検証 worktree へ取り込み、baseline commit に含める
- [x] 2.4 `KATANA_CHECKOUT=/tmp/... make external-katana-dogfood` を実行する
- [x] 2.5 検証 worktree で `kml check --output json` を実行し、fix 前の diagnostics と全対象 file inventory を保存する
- [x] 2.6 check 結果は diagnostics が出た file だけでなく、diagnostics 0 件の file も含めて全対象 file を check 評価台帳に記録する
- [x] 2.7 check 評価台帳には全対象 file ごとに「diagnostics 数」「rule 一覧」「check 精査結果」「根拠メモ」「kml 側対応要否」を記録する
- [x] 2.8 check 評価台帳の行数が全対象 file 数と一致し、`未評価` が 0 件になるまで check 評価を完了扱いにしない
- [x] 2.9 全対象 file について「diagnostics あり」「diagnostics なし」「check 誤検知候補」「check 見逃し候補」「未評価」を分けて check の正当性を評価する
- [x] 2.9a markdownlint など他実装との差が仕様差か実装側 issue かを判断するため、必要な rule は upstream issue / PR も確認して台帳に根拠を残す
- [x] 2.10 check 側に release-blocking な false-positive / false-negative がある場合、先に kml 側の bugfix を行い、再度 `kml check --output json` を実行して基準 check 結果を更新する
- [x] 2.11 最適化後の基準 check 結果を前提に、検証 worktree で `kml fix --output json` を実行し、どの rule に対して何が適用されたかを保存する
- [x] 2.12 fix 結果は差分が出た file だけでなく、差分 0 件の file も含めて全対象 file を fix 評価台帳に記録する
- [x] 2.13 fix 評価台帳には全対象 file ごとに「applied fix 数」「rule 一覧」「diff 有無」「fix 精査結果」「根拠メモ」「kml 側対応要否」を記録する
- [x] 2.14 fix 評価台帳の行数が全対象 file 数と一致し、`未評価` が 0 件になるまで fix 評価を完了扱いにしない
- [x] 2.15 fix 後の git diff を保存し、最適化後の check 結果、fix 結果、diff を突き合わせる
- [x] 2.16 全対象 file について check 評価と fix 評価を突き合わせ、「check 正当かつ diagnostics 0 件なのに fix 差分あり」を `check-fix-inconsistency` として検出する
- [x] 2.17 fix 後の差分を file / hunk ごとに周辺 Markdown 文脈込みで確認し、各差分について「基準 diagnostic は正当か」「どの rule に対する fix か」「fix は正当か」「kml 側 bug か」を評価する
- [x] 2.18 必要に応じて KatanA 側の追加ドキュメントを読み込み、再現条件を repo-local fixture に落とせるか判断する
- [x] 2.19 finding を `false-positive`、`false-negative`、`check-fix-inconsistency`、`unsafe-fix-risk`、`bad-fix`、`fmt-policy-gap`、`perf-regression`、`docs-only` に分類する
- [x] 2.20 release-blocking issue がある場合、`v0.12.21` の bugfix task として明記する
- [x] 2.21 release-blocking ではない issue は後続版 follow-up として記録する

## 3. Feedback 由来 bugfix

- [x] 3.0 KatanA worktree の差分自体を成果物にせず、検出した pattern を kml 側の regression test / implementation に戻す
- [x] 3.1 release-blocking false-positive / false-negative / bad-fix がある場合、rule-local regression test を先に追加する
- [x] 3.2 必要に応じて document-level fixture または public confidence fixture に再発条件を追加する
- [x] 3.3 production code を修正し、テスト都合だけの挙動変更を避ける
- [x] 3.4 fix 事故がある場合、default safe-fix allowlist または fix range を見直す
- [x] 3.5 KatanA 由来の false-positive / bad-fix が 0 件になるまで release-ready としない
- [x] 3.6 作業中に検出した `MD007` 初回 bad-fix を regression test 化し、ordered list 配下の unordered child を本文開始位置に保つ
- [x] 3.7 `make lint` で検出した needless borrow を修正し、lint 回避ではなく実装修正で解消する
- [x] 3.8 `make ast-lint` で検出した README rule map の不整合を、rule metadata / fixture / docs の整合修正で解消する

## 4. by-design 宣言

- [x] 4.1 `MD001`、`MD013`、`MD024`、`MD033`、`MD041`、`MD042`、`MD043`、`MD045`、`MD059` の safe-fix 非提供理由を確定する
- [x] 4.2 `MD028` が `v0.12.19` で safe-fix 実装されなかった場合、by-design 対象に追加する
- [x] 4.3 `README.md` の rule map で `Needs triage` を by-design 理由付き表示へ更新する
- [x] 4.4 `tests/fixtures/rule-fixture-matrix.json` と `.md` を README と揃える
- [x] 4.5 `docs/rule-fix-feasibility.md` と `docs/rule-coverage-dashboard.md` を必要に応じて更新する

## 5. リリース記録更新

- [x] 5.1 `CHANGELOG.md` に `v0.12.21` を追加する
- [x] 5.2 `Cargo.toml` を `0.12.21` に更新し、`Cargo.lock` を更新する
- [x] 5.3 `openspec/changes/active-roadmap.md` に 0.12.x DONE と `v0.13.0` へ進む条件を反映する
- [x] 5.4 KatanA feedback の release-blocking issue が 0 件であることを release evidence に残す

## 5a. 品質評価スコア

Release 条件は `100/100`、release-blocking issue `0`、check / fix 台帳の `未評価` `0`、`check-fix-inconsistency` `0` とする。
score が `100` 未満、または gate が失敗した場合は、この `tasks.md` に追加 task を記録し、修正して同じ gate を再実行する。

| 項目 | 配点 | 現在 | 完了条件 |
| --- | ---: | ---: | --- |
| check 精度評価 | 20 | 20 | 524 files 全件評価、false-positive / false-negative release blocker 0 |
| fix 精度評価 | 20 | 20 | 524 files / 82 hunks 全件評価、bad-fix release blocker 0 |
| 再発防止 | 15 | 15 | KatanA 由来 pattern を regression test と production fix に戻す |
| traversal 安全性 | 10 | 10 | reserved / ignored directory の default exclude と opt-in contract が test 済み |
| by-design 整合 | 10 | 10 | README / fixture matrix / docs / dashboard が manual-required 理由で一致 |
| release 記録 | 10 | 10 | CHANGELOG / version / roadmap / release evidence が更新済み |
| 品質 gate | 15 | 15 | `fmt-check`、`lint`、`test`、`ast-lint`、`dogfood`、`public-confidence`、`release-check`、`release-task-ledger-check` が成功 |
| 合計 | 100 | 100 | 100/100 で release-ready |

## 6. Quality Gates

- [x] 6.1 `make fmt-check` を実行する
- [x] 6.2 `make lint` を実行する
- [x] 6.3 `make test` を実行する
- [x] 6.4 `make ast-lint` を実行する
- [x] 6.5 `make dogfood` を実行する
- [x] 6.6 `make public-confidence` を実行する
- [x] 6.7 `make release-check VERSION=v0.12.21` を実行する
- [x] 6.8 `make release-task-ledger-check VERSION=v0.12.21` を実行する
- [x] 6.9 `scripts/openspec validate v0-12-21-katana-feedback-and-012x-closeout --strict` を実行する

## 7. 作業中に検出した追加作業

- [x] 7.1 古い初回 fix 証跡は残し、最終評価には `fix-after-check-review.*` を使うことを明記する
- [x] 7.2 `vendor/egui_commonmark_upstream/**` の 2 件は kml の誤検知 / 誤修正ではなく、consumer 側の対象範囲判断として後続検討に分類する
- [x] 7.3 `rule-fixture-matrix.json` の `manual_required` 件数を 13 に更新し、dashboard を再生成する
- [x] 7.4 作業管理の抜け漏れを防ぐため、`make release-task-ledger-check` を追加し、`tasks.md` の未完了 task と 100/100 スコアを機械的に検査する
- [x] 7.5 `make dogfood` が `MD052` で失敗したため、字下げコードブロック内の参照風テキストを無視し、shortcut syntax 有効時も full reference を二重検出しない regression test / 実装修正を追加し、OpenSpec の `[Risk]` 表記は `Risk:` へ変更する
- [x] 7.6 `make release-check` が coverage で失敗したため、coverage gate を `--lib --bins` 限定から workspace integration test 込みへ戻し、AST lint で再発防止する
- [x] 7.7 最終 `release-check` 再実行で追加 failure がないことを確認する
