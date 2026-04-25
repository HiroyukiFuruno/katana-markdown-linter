## Definition of Ready

- [x] `mcp-server-readonly-prototype` が archive 済みであること
- [x] `docs/mcp-server.md` の current contract を確認済みであること
- [x] core crate に MCP dependency を追加しない方針が確認済みであること
- [x] file mutation は explicit apply のみ許可する方針が確認済みであること
- [x] `release-operations-hardening` が完了済み、または MCP feature の publish/install 手順が release flow と衝突しないこと

## 1. Tool Contract

- [x] 1.1 current text tools の input/output schema を文書化する
- [x] 1.2 `check_file` schema を追加する
- [x] 1.3 `check_directory` schema を追加する
- [x] 1.4 `fix_file_preview` schema を追加する
- [x] 1.5 `fix_file_apply` schema を追加する

## 2. Workspace Safety

- [x] 2.1 workspace root resolution を実装する
- [x] 2.2 root 外 path を拒否する test を追加する
- [x] 2.3 symbolic path policy を実装し test で固定する
- [x] 2.4 ignore behavior を CLI と揃える
- [x] 2.5 non-UTF-8 / binary file handling を定義する

## 3. File And Directory Tools

- [x] 3.1 `check_file` を実装する
- [x] 3.2 `check_directory` を実装する
- [x] 3.3 `fix_file_preview` を実装する
- [x] 3.4 `fix_file_apply` を実装する
- [x] 3.5 apply 後 re-check を実装する

## 4. Docs And Examples

- [x] 4.1 Codex client 設定例を追加する
- [x] 4.2 ClaudeCode client 設定例を追加する
- [x] 4.3 Antigravity client 設定例を追加する
- [x] 4.4 `cargo install katana-markdown-linter --features mcp` の可否を検証し docs に反映する
- [x] 4.5 MCP feature を含む install smoke test が必要か release plan と照合する

## Verification

- [x] `cargo test --features mcp --bin kml-mcp --locked` が成功する
- [x] `cargo build --bin kml-mcp --features mcp --locked` が成功する
- [x] `cargo install --path . --locked --features mcp --bin kml-mcp --root <temp>` が成功する、または不可理由を docs に記録する
- [x] `make mcp-stdio-smoke` が MCP stdio 経由で `tools/list` と file tool call を検証する
- [x] MCP file tools が temp workspace を root 外へ出ないことを test する
- [x] `make check` が成功する
- [x] `git diff --check` が成功する

## Definition of Done

- [x] MCP server が file/directory check を workspace-safe に実行できること
- [x] file fix は preview と explicit apply に分かれていること
- [x] docs が native Rust stdio server としての起動方法を説明していること
