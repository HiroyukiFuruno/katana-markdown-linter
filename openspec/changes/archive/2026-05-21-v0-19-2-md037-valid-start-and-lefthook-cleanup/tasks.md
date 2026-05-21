## 1. MD037 valid_start ロジックの再整備

- [x] 1.1 `src/rules/markdown/rules/spaces_in_emphasis.rs` の `valid_start` を「直前: 行頭/空白/`([{"'`、または直前句読点 `.!?,;:` かつ直後 non-空白」へ拡張する
- [x] 1.2 通常強調の閉じ記号に見える強調記号を opener 候補から除外する判定を追加する
- [x] 1.3 必要に応じて `matching_end_marker` ほか補助ロジックの整合を取り、開始候補と終了候補のペアリングが連結を起こさないことを確認する

## 2. 回帰テストの増補

- [x] 2.1 `tests/emphasis_regressions.rs` に「句読点直後のスペース付き強調 `Hello.* spaced *.` を MD037 が検出し fix する」回帰テストを追加する
- [x] 2.2 `tests/emphasis_regressions.rs` の既存「同一行内の独立強調範囲を連結しない」テストと、同種・同長 marker の通常強調直後にあるスペース付き強調のテストが拡張ロジック下でも pass することを確認する
- [x] 2.3 `cargo test --test rule_fixture_harness -- --nocapture` で MD037 / MD049 / MD050 を含む fixture 全件 pass を確認する。差分が出た場合は意図確認のうえで fixture を更新するか判定ロジックを再調整する

## 3. lefthook pre-push 構成の整理

- [x] 3.1 `lefthook.yml` から `run: sh -c 'just JOBS=2 check' -- {files}` を一旦 `run: just JOBS=2 check` へ戻し、`files: git diff --name-only HEAD master` + `glob` の組み合わせで Markdown のみ変更時に command が skip されることを `lefthook run pre-push --file <path>` で確認する
- [x] 3.2 skip または Rust/TOML/lock 対象判定が発火しない場合のみ `sh -c '...' -- {files}` 形式へ戻し、近接行に意図コメント（`# {files} は lefthook の glob skip を発火させるためにのみ参照する`）を追加する

## 4. 受け入れ確認

- [x] 4.1 `cargo test --test emphasis_regressions -- --nocapture` が pass する
- [x] 4.2 `just check` が pass する
- [x] 4.3 `lefthook run pre-push --file <path>` がローカルで意図通り（Rust/TOML/lock 変更時は check 実行、Markdown のみは skip）動作する
- [x] 4.4 CHANGELOG.md に v0.19.2 セクションを追加し、本 change の対応内容を記載する
- [x] 4.5 `scripts/openspec validate v0-19-2-md037-valid-start-and-lefthook-cleanup --strict` を実行し artifact 整合性を確認する
