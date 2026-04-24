# Tasks

## DoR

- [x] `dogfood-cli-and-api-usage` と `upstream-compatibility-golden-gate` の scope を確認し、MCP change がそれらの実装を前提にしすぎていないことを確認する
- [x] 公式 MCP SDK docs と Rust SDK repository の現状を確認する
- [x] core crate が MCP 非依存であるべきという前提を design と照合する
- [x] file write を伴う operation の安全条件を検討対象に含める

## Evaluation

- [x] MCP use case を agent / editor / CI assistant の観点で整理する
- [x] core crate / CLI / MCP server の責務境界を図示または文書化する
- [x] MCP tools 候補を `check_text`、`check_files`、`fix_text`、`config_validate`、`rule_list`、`rule_get` で評価する
- [x] `fix_files` を提供する場合の opt-in、allowlist、dry-run diff 方針を評価する
- [x] MCP resources 候補を rule catalog、config summary、coverage dashboard で評価する
- [x] MCP prompts を初期 scope に入れるか判断する
- [x] Rust SDK 候補として official `rmcp` の採用可否を評価する
- [x] crate 構成候補を standalone binary、workspace crate、optional feature で比較する
- [x] binary 名候補を `kml-mcp` を第一候補として整理する
- [x] 採用 / 保留 / 不採用の recommendation を作成する
- [x] 採用する場合の次 change の DoR / DoD を作成する

## DoD

- [x] MCP integration が core crate に依存を逆流させない設計になっている
- [x] tools / resources / prompts の初期 scope が明確になっている
- [x] write-capable operation の safety policy が明文化されている
- [x] SDK / crate 構成 / binary 名の recommendation がある
- [x] 次に実装する場合の OpenSpec change name と DoR / DoD が提示されている
- [x] `openspec status --change mcp-integration-evaluation --json` で apply-ready である
