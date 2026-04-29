# Tasks

## 0. Definition of Ready

- [x] 0.1 `v0.17.0` の配布拡張をこの release に混ぜないことを `active-roadmap.md` に記録する
- [x] 0.2 public GitHub sample の license allowlist と取得方針を確定する
- [x] 0.3 corpus manifest の項目を確定する
- [x] 0.4 `xxx.md` と `xxx_answer.md` の配置規約を確定する
- [x] 0.5 過去 bug pattern の組み合わせ一覧を既存 tests / archived OpenSpec から作る

## 1. Corpus 設計と収集

- [x] 1.1 public GitHub sample を 200 件以上収集する
- [x] 1.2 public sample ごとに source repository、commit SHA、path、license、取得日、選定理由を manifest に記録する
- [x] 1.3 license 証跡が不十分な public sample を fixture content から除外する
- [x] 1.4 original sample を 50 件作成する
- [x] 1.5 各 original sample を 200 文字以上の意味が通る Markdown 文書にする
- [x] 1.6 各 original sample に 2 種類以上の historical bug pattern を組み合わせる
- [x] 1.7 original sample manifest に含めた historical bug pattern を記録する

## 2. Answer Fixture

- [x] 2.1 各 check 対象に対応する `xxx_answer.md` を作成する
- [x] 2.2 answer fixture が現在実装の出力コピーだけになっていないことを review する
- [x] 2.3 answer fixture と入力 fixture の対応漏れを検出する検査を追加する
- [x] 2.4 answer fixture の改行、末尾改行、Unicode を byte-for-byte 比較前提で固定する
- [x] 2.5 answer fixture 単体の残診断と再 fix 差分を検出する検査を追加する
- [x] 2.6 `MD013` は自動 fix 対象外の noise として document answer fix evaluation では無効化する

## 3. Evaluation Harness

- [x] 3.1 入力 sample を一時ディレクトリにコピーして `kml check --fix` を実行する harness を追加する
- [x] 3.2 fix 後の `xxx.md` と `xxx_answer.md` を byte-for-byte で比較する
- [x] 3.3 乖離時に sample 名、source、推定 rule、diff、分類を report へ出す
- [x] 3.4 `make document-answer-fix` 相当の entrypoint を Makefile に追加する
- [x] 3.5 `make release-check VERSION=v0.16.2` に document answer fix evaluation を組み込む
- [x] 3.6 answer fixture 単体の診断数と再 fix 差分数を report summary へ出す
- [x] 3.7 `cargo run` の終了コード wrapper を避けるため、document answer fix evaluation はビルド済み `kml` を直接実行する

## 4. Bug Detection and Fix

- [x] 4.1 harness の初回実行で検出された mismatch を bug candidate として分類する
- [x] 4.2 `default: true` 再評価で検出した public answer 差分 3 件を default fix 期待値として更新する
- [x] 4.3 `MD013` が Unicode 境界で panic する bug を regression test 付きで修正する
- [x] 4.4 answer 側の誤り、残診断、再 fix 差分が 0 件であることを確認する
- [x] 4.5 修正後に harness が全件 pass することを確認する

## 5. Release Metadata

- [x] 5.1 `Cargo.toml` / `Cargo.lock` を `0.16.2` に更新する
- [x] 5.2 `CHANGELOG.md` に `v0.16.2` の bugfix 内容を追加する
- [x] 5.3 `openspec/changes/active-roadmap.md` に `v0.16.2` の位置づけと `v0.17.0` の凍結継続を記録する
- [x] 5.4 対象 OpenSpec change の品質評価スコアを実装結果に合わせて更新する

## 6. Verification

- [x] 6.1 `make fmt-check`
- [x] 6.2 `make lint`
- [x] 6.3 `make ast-lint`
- [x] 6.4 `cargo test --workspace --locked`
- [x] 6.5 `make dogfood`
- [x] 6.6 `make document-answer-fix`
- [x] 6.7 `git diff --check`
- [x] 6.8 `make release-check VERSION=v0.16.2`
- [x] 6.9 `make release-task-ledger-check VERSION=v0.16.2`
- [x] 6.10 `cargo test --test document_answer_fix_answer_validation --locked`
- [x] 6.11 `cargo test handles_unicode_at_line_length_boundary_without_panicking --locked`

## Definition of Done

- [x] 7.1 public GitHub sample が 200 件以上あり、source / commit / path / license / 選定理由が manifest に記録されている
- [x] 7.2 original sample が 50 件あり、各 sample が 200 文字以上かつ historical bug pattern を複数組み合わせている
- [x] 7.3 全 check 対象に `xxx_answer.md` が存在する
- [x] 7.4 `kml check --fix` 後の出力が全 answer fixture と byte-for-byte で一致する
- [x] 7.5 検出された bug が修正済み、または answer 誤りとして理由付きで整理済みである
- [x] 7.6 `v0.17.0` の配布拡張が `v0.16.2` に混入していない
- [x] 7.7 release gate が全て成功している
- [x] 7.8 answer fixture 単体の残診断と再 fix 差分が 0 件である
- [x] 7.9 `MD013` の Unicode 境界 panic が回帰テストで固定されている

## 品質評価スコア

| 項目 | 最大 | 現在 | 根拠 |
| --- | ---: | ---: | --- |
| corpus | 25 | 25 | 200 public samples と 50 original samples を manifest に固定した。 |
| answer fixture | 20 | 20 | 全 250 input に `xxx_answer.md` を対応させ、`default: true` + `MD013: false` 前提の review note と answer 単体診断を固定した。 |
| harness | 20 | 20 | `scripts/ci/document-answer-fix.py`、answer validator、ビルド済み `kml` を使う `make document-answer-fix` を追加した。 |
| bugfix | 15 | 15 | default 再評価で public answer 3 件を更新し、`MD013` Unicode 境界 panic を修正した。 |
| release metadata | 10 | 10 | version、CHANGELOG、MCPB/server metadata、roadmap を `v0.16.2` に更新した。 |
| verification | 10 | 10 | 6.1 から 6.9 までの verification と release task ledger check が成功した。 |
| 合計 | 100 | 100 | 全タスク完了。 |
