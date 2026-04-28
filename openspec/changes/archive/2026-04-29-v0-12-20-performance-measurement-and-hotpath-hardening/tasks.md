## 0. 前提確認

- [x] 0.1 `v0.12.19` が release 済みであることを確認する
- [x] 0.2 `v0.12.19` で `MD003` / `MD028` がどの状態になったかを README と fixture matrix で確認する
- [x] 0.3 Makefile の performance 関連 target を確認する

## 1. 性能計測

- [x] 1.1 `make bench` を実行し、`target/perf-report.json` を生成する
- [x] 1.2 `make perf-check` を実行し、baseline 比較を記録する
- [x] 1.3 `make perf-check-strict` を実行し、ratio が許容範囲にあるか確認する
- [x] 1.4 `make public-confidence` を実行し、check / fix / fmt timing と収束性を確認する
- [x] 1.5 必要に応じて `make bench-cross-tools-default`、`make bench-cross-tools-common`、`make bench-cross-tools-fix` を実行し、optional tool の skipped を含めて記録する

## 2. 改善対象の分類

- [x] 2.1 悪化した benchmark case がある場合、`api_lint`、`api_fix`、`api_rule_catalog`、`context`、`cli`、`config` のどれかに分類する
- [x] 2.2 説明不能な退行がない場合は evidence update のみにする判断を行う（今回は 2.3 を適用）
- [x] 2.3 説明不能な退行がある場合、最小の hot path 改善案を design に追記してから実装する

## 3. 性能改善または baseline 更新

- [x] 3.1 改善が必要な場合のみ、正しさを変えない内部最適化を実装する
- [x] 3.2 改善後に `make test` を実行する
- [x] 3.3 改善後に `make public-confidence` を実行する
- [x] 3.4 改善または benchmark shape の意図的変更がある場合だけ `make perf-refresh-baseline` を実行する
- [x] 3.5 `docs/performance.md` に必要な snapshot と解釈を追記する

## 4. リリース記録更新

- [x] 4.1 `CHANGELOG.md` に `v0.12.20` を追加する
- [x] 4.2 `Cargo.toml` を `0.12.20` に更新し、`Cargo.lock` を更新する
- [x] 4.3 `openspec/changes/active-roadmap.md` に `v0.12.20` の完了条件を反映する
- [x] 4.4 `v0.12.21` に引き継ぐ KatanA feedback sweep 観点を tasks に残す

## 5. Quality Gates

- [x] 5.1 `make fmt-check` を実行する
- [x] 5.2 `make lint` を実行する
- [x] 5.3 `make test` を実行する
- [x] 5.4 `make ast-lint` を実行する
- [x] 5.5 `make dogfood` を実行する
- [x] 5.6 `make perf-check-strict` を実行する
- [x] 5.7 `make release-check VERSION=v0.12.20` を実行する
