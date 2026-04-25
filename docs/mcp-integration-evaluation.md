# MCP Integration Evaluation

## Recommendation

Adopt MCP integration in a follow-up change, starting with a read-only and
text-only prototype named `mcp-server-readonly-prototype`.

The binary should be named `kml-mcp`. The core library must remain MCP-free.
The MCP server should be an adapter that calls the public library API.

## References

- Official SDK list: <https://modelcontextprotocol.io/docs/sdk>
- Official Rust SDK repository: <https://github.com/modelcontextprotocol/rust-sdk>
- `rmcp` crate docs: <https://docs.rs/rmcp>

The official SDK list identifies Rust support through
`modelcontextprotocol/rust-sdk`. The Rust SDK is exposed as `rmcp`.

## Responsibility Boundary

~~~text
core crate
  owns lint, fix, config, rule catalog, and normalized diagnostics

kml CLI
  owns terminal UX, recursive file traversal, exit codes, and stdout/stderr

kml-mcp server
  owns MCP tools/resources, request validation, workspace access policy,
  and serialization of core API results
~~~

The dependency direction must be:

~~~text
kml CLI    -> core crate
kml-mcp    -> core crate
core crate -> no MCP dependency
~~~

## Use Cases

| User | Value | Priority |
| --- | --- | --- |
| Agent | Ask for Markdown diagnostics without shelling out | High |
| Editor | Get rule metadata and validate buffers | Medium |
| CI assistant | Explain lint failures and suggest fixes | Medium |

## Initial Tool Scope

| Tool | Kind | Recommendation |
| --- | --- | --- |
| `check_text` | read-only | Include first |
| `fix_text` | no workspace write | Include first |
| `config_validate` | read-only | Include first |
| `rule_list` | read-only | Include first |
| `rule_get` | read-only | Include first |
| `check_files` | read-only file access | Include after path allowlist is implemented |
| `fix_files` | write-capable | Defer |

`fix_text` returns fixed content and fix metadata. It must not write files.

## Resource Scope

| Resource | Recommendation |
| --- | --- |
| `kml://rules` | Include first |
| `kml://config/summary` | Include first |
| `kml://coverage/dashboard` | Include after dashboard generation is stable |

Prompts are not part of the initial scope. They should be reconsidered only
after tools and resources are stable.

## Write Safety Policy

Default mode is read-only.

Write-capable operations require:

- explicit server configuration opt-in
- path allowlist rooted at the workspace
- dry-run diff response before write
- no writes outside the allowlist
- structured response that reports changed paths and applied fixes

`fix_files` should not be implemented until `kml` has a non-writing fix preview
path that can be reused by MCP.

## SDK And Packaging

Preferred implementation:

- add optional Cargo feature `mcp`
- add binary target `kml-mcp`
- set `required-features = ["mcp"]` for the binary
- depend on `rmcp` and async runtime only behind the `mcp` feature

This keeps normal library users and CLI users from paying MCP dependency cost.

## Next Change

Change name: `mcp-server-readonly-prototype`

### DoR

- `dogfood-cli-and-api-usage` is complete
- `upstream-compatibility-golden-gate` is complete
- core API can return diagnostics and fixed text without CLI coupling
- `rmcp` version and feature set are pinned in `Cargo.toml`
- workspace path allowlist policy is documented before file tools are exposed

### DoD

- `kml-mcp` builds only with feature `mcp`
- core crate has no MCP dependency
- MCP server exposes `check_text`, `fix_text`, `config_validate`, `rule_list`, and `rule_get`
- file write tools are absent
- server responses are JSON-serializable and include diagnostics with rule ID and range
- README documents that MCP support is optional and experimental
