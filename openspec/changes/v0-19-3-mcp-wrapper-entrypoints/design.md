# Design

## Context

現在の `katana-markdown-linter` は、以下の配布導線を持つ。

- `kml` CLI: crates.io、GitHub Release binary archive、Homebrew、npm wrapper、PyPI wrapper で利用できる。
- `kml-mcp`: `cargo install --features mcp --bin kml-mcp`、local stdio smoke、MCPB bundle で検証されている。
- `kml-mcp-remote`: `cargo install --features mcp-remote --bin kml-mcp-remote`、Streamable HTTP smoke で検証されている。

ただし npm / PyPI wrapper は `kml` binary だけを download / verify / execute する。LLM client の設定へ一時実行形式で登録するには、`kml-mcp` を `npx` / `bunx` / `uvx` から直接起動できる必要がある。

## Goals / Non-Goals

**Goals:**

- Rust toolchain なしで `kml-mcp` を `npx` / `bunx` / `uvx` から起動できる。
- `kml-mcp-remote` も同じ wrapper architecture で起動できる。
- 既存の `kml` CLI wrapper と cache contract を壊さない。
- release gate が package metadata だけでなく、実際の MCP protocol call まで検証する。
- LLM client 登録例を、実際に動く command / args で示す。

**Non-Goals:**

- public hosted MCP service の提供。
- MCP Registry / third-party Hub への新規 publish。
- npm / PyPI package に lint rule や formatter logic を実装すること。
- `kml-mcp` の workspace file policy や write policy の変更。
- `kml` CLI archive 名の変更。

## Decisions

### Decision 1: MCP 用 binary archive を executable prefix ごとに追加する

`kml` archive を MCP binary で膨らませず、以下の archive を追加する。

- `kml-mcp-vX.Y.Z-<target>.tar.gz` / `.zip`
- `kml-mcp-remote-vX.Y.Z-<target>.tar.gz` / `.zip`

各 archive は対応する executable、`LICENSE`、短い install note を含み、隣接する `.sha256` を持つ。既存の `kml-vX.Y.Z-<target>` は維持する。

**理由:** `kml` CLI と MCP server は利用シーン、feature set、smoke test が異なる。archive を分けることで wrapper installer は必要な binary だけを取得でき、既存 Homebrew / CLI wrapper の契約も保てる。

**Alternatives considered:**

- `kml` archive に MCP binaries も同梱する: 既存 archive の意味が広がり、CLI-only 利用者に不要な payload を配る。却下。
- MCPB だけを wrapper から使う: MCPB は local stdio package / Registry metadata 向けであり、npm / PyPI thin wrapper の target 別 binary cache と責務が違う。却下。

### Decision 2: npm / PyPI installer を binary role 対応にする

installer は `binary_role` を受け取り、以下を解決する。

| role | executable | archive prefix | required feature |
| --- | --- | --- | --- |
| `cli` | `kml` | `kml` | default |
| `mcp` | `kml-mcp` | `kml-mcp` | `mcp` |
| `mcp-remote` | `kml-mcp-remote` | `kml-mcp-remote` | `mcp-remote` |

cache path は `version / target / executable / bin/<executable>` のように executable ごとに分離する。これにより `kml` cache と `kml-mcp` cache が混ざらない。

**理由:** wrapper は lint logic を持たず、公式 GitHub Release artifact を検証して起動するだけに保つ。role 対応は installer の責務であり、各 launcher は role を指定するだけにする。

### Decision 3: npm は package-name dispatcher と direct bin の両方を提供する

npm package の `bin` は以下を提供する。

- `katana-markdown-linter`: package-name dispatcher
- `kml`: 既存 CLI launcher
- `kml-mcp`: MCP stdio launcher
- `kml-mcp-remote`: remote MCP launcher

`katana-markdown-linter` dispatcher は、先頭引数が `kml` / `kml-mcp` / `kml-mcp-remote` の場合に対応 launcher へ委譲し、それ以外は既存通り `kml` へ委譲する。

これにより次の形を公式にする。

```bash
npx --yes katana-markdown-linter@0.19.3 kml-mcp --workspace-root /absolute/path/to/workspace
bunx katana-markdown-linter@0.19.3 kml-mcp --workspace-root /absolute/path/to/workspace
```

direct bin を使う client では次の形も許可する。

```bash
npx --yes --package katana-markdown-linter@0.19.3 kml-mcp --workspace-root /absolute/path/to/workspace
```

**理由:** `npx <package> <args...>` と `bunx <package> <args...>` は package-name の executable を使う形が最も利用者に説明しやすい。direct bin も残すことで MCP client の command / args 生成に合わせやすくする。

### Decision 4: PyPI は console scripts を追加する

PyPI package は以下の console scripts を提供する。

- `kml`
- `kml-mcp`
- `kml-mcp-remote`

公式の one-off 実行は次の形にする。

```bash
uvx --from katana-markdown-linter==0.19.3 kml-mcp --workspace-root /absolute/path/to/workspace
```

**理由:** `uvx` は package 名と executable 名を分離できるため、Python 側は dispatcher より console script の明示が自然。

### Decision 5: MCP smoke は wrapper 経由の JSON-RPC まで検証する

`kml-mcp` は通常の `--version` command ではなく MCP stdio で動く。したがって wrapper smoke は、process 起動後に `initialize`、`tools/list`、`check_text` または `check_file` を JSON-RPC で呼ぶ。

`kml-mcp-remote` は bearer token 付き Streamable HTTP smoke を行い、workspace file tools が出ないことを確認する。

**理由:** 起動できるだけでは LLM が使える状態とは言えない。MCP protocol で tool が列挙され、実 tool call が通ることを release gate にする。

## Risks / Trade-offs

- **Risk:** release asset 数が増え、workflow と release verification が複雑になる。  
  **Mitigation:** binary packaging script を executable parameter 化し、asset list generation と verification を同じ source of truth へ寄せる。
- **Risk:** `npx` / `bunx` の package-name invocation が registry / runtime version により差分を持つ。  
  **Mitigation:** release gate で exact command を実行し、通らない形式は公式例に載せない。
- **Risk:** MCP server は stdio を占有するため、wrapper が stdout に余計な log を出すと protocol を壊す。  
  **Mitigation:** MCP launcher は stdout に wrapper log を出さない。download / checksum / error は stderr に限定する。
- **Risk:** remote MCP を手軽に起動できることで公開 endpoint と誤解される。  
  **Mitigation:** remote docs は self-hosted / bearer token / localhost default / no public hosted service を維持する。

## Migration Plan

1. packaging script を executable parameter 化し、`kml` の既存 archive と smoke が変わらないことを先に確認する。
2. `kml-mcp` / `kml-mcp-remote` archive build と smoke を追加する。
3. npm / PyPI wrapper installer を role 対応にし、既存 `kml` wrapper smoke を維持する。
4. `npx` / `bunx` / `uvx` 経由の MCP smoke を追加する。
5. release workflow / release verification / docs を更新する。

Rollback は wrapper entrypoint と MCP archive 追加分の revert で行う。既に `v0.19.3` を registry 公開した後に修正が必要になった場合、同じ version を再利用せず次 patch version へ進める。
