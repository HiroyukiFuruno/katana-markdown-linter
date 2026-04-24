# Design

## Goals

- Agent/editor が text buffer を MCP 経由で lint / fix preview / config validation / rule introspection できるようにする
- core crate を MCP SDK、transport、async runtime から独立させる
- write-capable workflow を prototype に混ぜず、安全方針が整うまで file operation を追加しない

## Non-Goals

- workspace path traversal、recursive directory check、file rewrite は実装しない
- KatanA 固有 adapter、KatanA 固有 config、KatanA workspace model は実装しない
- MCP resources / prompts は今回の completion 条件に含めない

## Architecture

```text
MCP client
  -> kml-mcp binary
      -> katana_markdown_linter public API
          -> core lint/fix/config/rule implementation
```

Dependency direction is one-way. The library exposes linting primitives and rule metadata; `kml-mcp` only serializes MCP requests/responses around those primitives.

## SDK And Runtime

- Use `rmcp` as the Rust MCP SDK.
- Pin the dependency through `Cargo.lock`.
- Add `rmcp` only behind the optional `mcp` Cargo feature.
- Add `tokio` only behind the optional `mcp` Cargo feature.
- Use stdio transport for the prototype.

## Tool Contract

| Tool | Workspace access | Behavior |
| --- | --- | --- |
| `check_text` | none | lint supplied Markdown content and return structured diagnostics |
| `fix_text` | none | return fixed content, applied fix count, and remaining diagnostics |
| `config_validate` | none | validate supplied JSON object against markdownlint-compatible config rules |
| `rule_list` | none | return rule metadata for all available rules |
| `rule_get` | none | return metadata for a single rule ID or an error |

## Safety

- No tool accepts a filesystem path in this prototype.
- No tool writes to the workspace.
- `fix_text` is safe because it returns content to the caller and leaves persistence to the caller.
- Future file tools MUST add explicit opt-in, workspace-root allowlist, and dry-run diff before write.

## Response Shape

Diagnostics are JSON-serializable and include:

- rule ID
- rule name
- message
- severity
- start line and column
- end line and column
- fix availability

`fix_text` returns the fixed content and remaining diagnostics after applying safe fixes so callers can decide whether to persist or display additional issues.

## Verification

- `cargo build --bin kml-mcp --features mcp --locked`
- `cargo test --features mcp --bin kml-mcp --locked`
- default `make check`
- source inspection confirms the core crate has no direct `rmcp` dependency usage
