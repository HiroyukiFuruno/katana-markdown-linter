## 1. FixDetail 公開型追加（C-1）

- [x] 1.1 `src/types.rs` に `FixDetail { rule_id: String, range: Range, applied: bool }` を追加し `#[derive(Debug, Clone, PartialEq, Eq, Serialize)]` を付与する
- [x] 1.2 `FixResult` に `details: Vec<FixDetail>` フィールドを追加する（`#[serde(default)]` で後方互換を維持）
- [x] 1.3 `src/lib.rs` の公開 re-export に `FixDetail` を追加する

## 2. fix::apply の FixDetail 収集実装（C-1）

- [x] 2.1 `src/fix/mod.rs` の edits タプルに `rule_id: &str` と `fix_range: Range` を追加する
- [x] 2.2 accepted ループ内で `FixDetail { rule_id, range, applied: true }` を収集する
- [x] 2.3 スキップされた edits を `FixDetail { rule_id, range, applied: false }` として収集する
- [x] 2.4 `FixResult` を構築するときに `details` フィールドを詰める
- [x] 2.5 `FixResult::default()` の `details` フィールドが空 Vec になることを確認する（既存の `Default` derive で OK）

## 3. FixDetail テスト追加（C-1）

- [x] 3.1 `src/fix/mod.rs` の unit test に「safe fix 適用で applied=true の FixDetail が返る」テストを追加する
- [x] 3.2 「競合 fix でスキップされた edit が applied=false で返る」テストを追加する
- [x] 3.3 `tests/ast_linter.rs` の public API surface test を `FixDetail` を含む形に更新して integration test を兼ねる

## 4. MD051 fragment 精度テスト追加（C-2）

- [x] 4.1 `src/rules/markdown/rules/md051/fragments.rs` の `github_heading_slug` が emoji を除去することを unit test で確認する
- [x] 4.2 CJK 文字を含む見出しのフラグメントが正しく生成されることを unit test で確認する
- [x] 4.3 emoji 混在見出し（例: `# Hello 🎉 World`）のリンク参照が false positive / false negative にならないことを integration test で確認する
- [x] 4.4 emoji のみ見出し（例: `# 🎉`）が空フラグメントとして扱われ（不一致リンクでエラー）、片方だけ emoji を除去した場合の挙動を確認する
- [x] 4.5 既存の MD051 golden test が引き続き通ることを `make test` で確認する

## 5. Quality Gates

- [x] 5.1 `make test` を実行して全テストが通ることを確認する
- [x] 5.2 `make bench` はパフォーマンス変更なし（実装変更なし）のため省略
- [x] 5.3 `make ast-lint` を実行して内部品質スコアが維持されていることを確認する
- [x] 5.4 `CHANGELOG.md` に v0.12.17 エントリを追加する
- [x] 5.5 `Cargo.toml` のバージョンを `0.12.17` に更新する
