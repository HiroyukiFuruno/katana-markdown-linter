# Design

## Dependency On v0.13.0

この change は `v0-13-0-mcp-registry-and-distribution-planning` の完了を前提にする。
package type、`server.json` 方針、security checklist が未確定なら開始しない。

## Artifact Strategy

実装は `v0.13.0` で選ばれた package type を正とする。

- MCPB が選ばれた場合は、desktop client が `kml-mcp` binary を stdio server として起動できる package を作る
- OCI image が選ばれた場合は、GHCR などの image registry に置ける artifact と workspace mount の説明を作る
- npm / PyPI wrapper が選ばれた場合は、wrapper が Rust binary install を隠しすぎないことを smoke test で確認する

複数 artifact を同時に出す場合でも、Registry metadata の primary install path は 1 つに固定する。

## Registry Metadata

`server.json` は repository 内に保持し、release 時に検証する。

metadata は少なくとも以下を表す。

- server id と表示名
- package type と package reference
- `kml-mcp` command と `--workspace-root` の指定方法
- documentation URL
- license と source repository
- destructive write が explicit apply のみであること
- remote MCP support を提供していないこと

## Release And Publication Flow

1. package artifact を build する
2. artifact smoke test で `tools/list` と file check を実行する
3. `server.json` schema と install command を検証する
4. release runbook に従って tag / release / package publication を行う
5. Registry / Hub publish を実行する
6. 公開後に Registry listing、install path、documentation URL を確認する

Registry publish は release artifact が確認できた後に行う。

## Safety

公開後も workspace write policy は変えない。

- workspace root 外の path は拒否する
- symbolic path は policy に従って扱う
- file fix は preview と explicit apply のみ
- directory-wide mutation は提供しない

## Rollback Policy

Registry metadata に誤りがある場合は metadata を修正する。
既に公開した immutable artifact に問題がある場合は、同じ version を上書きせず次の patch version で修正する。
