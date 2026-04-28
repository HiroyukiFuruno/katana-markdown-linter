## 0. 前提確認

- [ ] 0.1 `v0.12.18` が release 済みで、`Cargo.toml` と tag が `0.12.18` を指していることを確認する
- [ ] 0.2 `MD003` と `MD028` の現状を README、fixture matrix、rule-local tests で確認する
- [ ] 0.3 `make help` と Makefile の品質 gate を確認し、自己流コマンドを使わない

## 1. MD003 safe-fix 実装

- [ ] 1.1 `src/rules/markdown/rules/heading_style.rs` に setext 見出しの置換範囲を返す helper を追加する
- [ ] 1.2 `=` underline を `# Heading`、`-` underline を `## Heading` に変換する `fix_info` を追加する
- [ ] 1.3 front matter、horizontal rule、code block 内は既存通り修正対象外にする
- [ ] 1.4 `MD003` を safe-fix allowlist に追加する
- [ ] 1.5 `MD003` の unit test と fixture matrix の fix ケースを追加する

## 2. MD028 fix 方針決定

- [ ] 2.1 `MD028` の公式修正案を確認し、自動修正が文意を変えない条件を書き出す
- [ ] 2.2 safe subset が成立する場合だけ `blockquote.rs` に `fix_info` を追加する
- [ ] 2.3 safe subset が成立しない場合は実装せず、`v0.12.21` の by-design 対象に送る
- [ ] 2.4 GFM Alert 間の空行が修正対象にならないことを unit test で固定する
- [ ] 2.5 fixture matrix と README の `MD028` 行を、実装結果または by-design 送りに合わせて更新する

## 3. リリース記録更新

- [ ] 3.1 `CHANGELOG.md` に `v0.12.19` を追加する
- [ ] 3.2 `Cargo.toml` を `0.12.19` に更新し、`Cargo.lock` を更新する
- [ ] 3.3 `openspec/changes/active-roadmap.md` に `v0.12.19` の完了条件を反映する

## 4. Quality Gates

- [ ] 4.1 `cargo test -p katana-markdown-linter --lib heading_style` を実行する
- [ ] 4.2 `cargo test -p katana-markdown-linter --lib blockquote` を実行する
- [ ] 4.3 `make fmt-check` を実行する
- [ ] 4.4 `make lint` を実行する
- [ ] 4.5 `make test` を実行する
- [ ] 4.6 `make ast-lint` を実行する
- [ ] 4.7 `make dogfood` を実行する
- [ ] 4.8 `make release-check VERSION=v0.12.19` を実行する
