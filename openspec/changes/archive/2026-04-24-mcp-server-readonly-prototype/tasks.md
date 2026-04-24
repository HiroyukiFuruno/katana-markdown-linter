# Tasks

## DoR

- [x] `docs/mcp-integration-evaluation.md` の recommendation が `kml-mcp` optional adapter 方針であることを確認する
- [x] `rmcp` の stdio server pattern と required feature を確認する
- [x] core public API が text lint、text fix、config validation、rule metadata を提供していることを確認する
- [x] prototype では file read/write tool を提供しないことを design と spec で確認する

## Implementation

- [x] `mcp` Cargo feature、optional dependency、`kml-mcp` binary target を追加する
- [x] `kml-mcp` stdio server skeleton を追加する
- [x] `check_text` tool と response schema を実装する
- [x] `fix_text` tool と remaining diagnostics response を実装する
- [x] `config_validate` tool と structured error response を実装する
- [x] `rule_list` tool を実装する
- [x] `rule_get` tool を実装する
- [x] MCP server unit tests を追加する
- [x] README と docs に optional / experimental MCP server usage を追加する
- [x] Makefile に MCP build/test target を追加する

## DoD

- [x] `cargo build --bin kml-mcp --features mcp --locked` が成功する
- [x] `cargo test --features mcp --bin kml-mcp --locked` が成功する
- [x] `make check` が成功する
- [x] default feature build が MCP SDK を要求しない
- [x] `kml-mcp` が file write tool を公開していないことを test または source inspection で確認する
- [x] `openspec status --change mcp-server-readonly-prototype --json` で apply-ready である
