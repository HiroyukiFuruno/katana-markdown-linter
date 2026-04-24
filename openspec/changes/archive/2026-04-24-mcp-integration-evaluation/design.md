## Context

MCP は agent が外部 tool / resource / prompt を呼び出すための protocol であり、公式 SDK 一覧では Rust SDK も提供されている。2026-04-24 時点で公式 docs は Rust SDK を Tier 2 として扱い、`modelcontextprotocol/rust-sdk` は `rmcp` crate を中心に server / client 実装を提供している。

`katana-markdown-linter` の中心価値は Markdown linting engine であり、MCP は transport と integration surface である。したがって core crate は MCP を知らず、MCP server は core API を呼び出す薄い adapter として設計する。

References:

- https://modelcontextprotocol.io/docs/sdk
- https://github.com/modelcontextprotocol/rust-sdk

## Goals / Non-Goals

**Goals:**

- MCP 化する価値がある use case と優先順位を定義する
- core crate / CLI / MCP server の責務境界を定義する
- check / fix / config / rule introspection を MCP tool として公開する候補を整理する
- workspace write を伴う fix operation の安全条件を定義する
- 採用する SDK または実装方針の判断基準を作る

**Non-Goals:**

- この change で本番 MCP server を実装すること
- KatanA 固有の adapter を linter 側に持ち込むこと
- MCP 経由で任意 shell command を実行できるようにすること
- CLI の既存 contract を MCP 都合で変更すること

## Decisions

- Core crate は MCP 非依存を維持する。MCP server は別 binary、別 crate、または optional feature として検討する。
- MCP tools の第一候補は `check_text`、`check_files`、`fix_text`、`config_validate`、`rule_list`、`rule_get` とする。`fix_files` は workspace write を伴うため、最初の採用候補から分離する。
- MCP resources は rule catalog、current config summary、coverage dashboard の read-only surface を候補にする。
- MCP prompts は初期段階では必須にしない。prompt は client 側と責務が重なりやすいため、tools/resources が安定した後に再評価する。
- Rust SDK は公式 `modelcontextprotocol/rust-sdk` / `rmcp` を第一候補として調査する。ただし Tier 2 であること、release cadence、API stability、dependency cost を採用判断に含める。
- Security policy は read-only default とし、write operation は explicit opt-in、path allowlist、dry-run diff を必須候補にする。

## Risks / Trade-offs

- MCP SDK の API が変化し、maintenance cost が増える可能性がある。
  Mitigation: MCP integration を core crate から分離し、optional dependency にする。

- MCP 経由の fix が workspace を意図せず変更する可能性がある。
  Mitigation: 初期は text-based fix と dry-run を優先し、file write は明示 opt-in にする。

- MCP server が CLI と同じ機能を重複実装する可能性がある。
  Mitigation: CLI と MCP はどちらも core library API を呼び出す adapter として扱う。
