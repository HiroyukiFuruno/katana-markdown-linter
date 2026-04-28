## 0. 前提確認

- [ ] 0.1 `v0.12.20` が release 済みで、performance blocker が残っていないことを確認する
- [ ] 0.2 `v0.12.19` の結果を確認し、`MD028` が safe-fix 実装済みか by-design 候補かを確定する
- [ ] 0.3 KatanA checkout の場所を確認し、`KATANA_CHECKOUT` を使う実行方法を決める

## 1. KatanA feedback sweep

- [ ] 1.1 KatanA 側の対象 Markdown 文書群を確認する
- [ ] 1.2 `KATANA_CHECKOUT=/path/to/katana make external-katana-dogfood` を実行する
- [ ] 1.3 必要に応じて KatanA 側の追加ドキュメントを読み込み、再現条件を repo-local fixture に落とせるか判断する
- [ ] 1.4 finding を `false-positive`、`false-negative`、`unsafe-fix-risk`、`fmt-policy-gap`、`perf-regression`、`docs-only` に分類する
- [ ] 1.5 release-blocking issue がある場合、`v0.12.21` の bugfix task として明記する
- [ ] 1.6 release-blocking ではない issue は後続版 follow-up として記録する

## 2. Feedback 由来 bugfix

- [ ] 2.1 release-blocking false-positive / false-negative がある場合、rule-local regression test を先に追加する
- [ ] 2.2 必要に応じて document-level fixture または public confidence fixture に再発条件を追加する
- [ ] 2.3 production code を修正し、テスト都合だけの挙動変更を避ける
- [ ] 2.4 fix 事故がある場合、default safe-fix allowlist または fix range を見直す

## 3. by-design 宣言

- [ ] 3.1 `MD001`、`MD013`、`MD024`、`MD033`、`MD041`、`MD042`、`MD043`、`MD045`、`MD059` の safe-fix 非提供理由を確定する
- [ ] 3.2 `MD028` が `v0.12.19` で safe-fix 実装されなかった場合、by-design 対象に追加する
- [ ] 3.3 `README.md` の rule map で `Needs triage` を by-design 理由付き表示へ更新する
- [ ] 3.4 `tests/fixtures/rule-fixture-matrix.json` と `.md` を README と揃える
- [ ] 3.5 `docs/rule-fix-feasibility.md` と `docs/rule-coverage-dashboard.md` を必要に応じて更新する

## 4. リリース記録更新

- [ ] 4.1 `CHANGELOG.md` に `v0.12.21` を追加する
- [ ] 4.2 `Cargo.toml` を `0.12.21` に更新し、`Cargo.lock` を更新する
- [ ] 4.3 `openspec/changes/active-roadmap.md` に 0.12.x DONE と `v0.13.0` へ進む条件を反映する
- [ ] 4.4 KatanA feedback の release-blocking issue が 0 件であることを release evidence に残す

## 5. Quality Gates

- [ ] 5.1 `make fmt-check` を実行する
- [ ] 5.2 `make lint` を実行する
- [ ] 5.3 `make test` を実行する
- [ ] 5.4 `make ast-lint` を実行する
- [ ] 5.5 `make dogfood` を実行する
- [ ] 5.6 `make public-confidence` を実行する
- [ ] 5.7 `make release-check VERSION=v0.12.21` を実行する
