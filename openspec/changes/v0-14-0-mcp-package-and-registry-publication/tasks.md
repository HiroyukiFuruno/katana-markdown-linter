# Tasks

## Definition of Ready

- [x] `v0-13-0-mcp-registry-and-distribution-planning` が完了していること
- [x] package type の第一候補と fallback 条件が決まっていること
- [x] `server.json` draft が存在すること
- [x] security review checklist が存在すること
- [x] Registry / Hub 公開に必要な credential と ownership 確認方法が決まっていること

## 1. Package Artifact

- [x] 1.1 選定済み package type 用の manifest / build script を追加する
- [x] 1.2 `kml-mcp` binary が `mcp` feature 付きで artifact に含まれることを固定する
- [x] 1.3 artifact から `tools/list` を実行する smoke test を追加する
- [x] 1.4 artifact から workspace-safe `check_file` を実行する smoke test を追加する
- [x] 1.5 artifact install path と local development install path の違いを docs に記録する

## 2. Registry Metadata

- [x] 2.1 `server.json` を repository に追加する
- [x] 2.2 `server.json` の schema validation を追加する
- [x] 2.3 metadata が primary install path、source repository、docs URL を指すことを検証する
- [x] 2.4 metadata が remote MCP support を過剰に主張しないことを確認する
- [x] 2.5 Registry aggregator に必要な tags / categories があれば追加する

## 3. Release Runbook

- [x] 3.1 package artifact build 手順を release runbook に追加する
- [x] 3.2 Registry / Hub publish 手順を release runbook に追加する
- [x] 3.3 公開後 verification を release runbook に追加する
- [x] 3.4 immutable artifact の修正時は patch version を使う方針を明記する
- [x] 3.5 failed publish の retry 判断を既存 release policy と揃える

## 4. Publication

- [x] 4.1 release gate で package artifact smoke test を通す
- [x] 4.2 security checklist を完了する
- [x] 4.3 Registry / Hub publish を実行する
- [x] 4.4 公開 listing から install と documentation URL を確認する
- [x] 4.5 README / docs / changelog / version metadata を `v0.14.0` に更新する

## Verification

- [x] `make mcp-stdio-smoke`
- [x] package artifact smoke test
- [x] `server.json` schema validation
- [x] `make release-check VERSION=v0.14.0`
- [x] Registry / Hub listing verification
- [x] `git diff --check`
- [x] `make release-task-ledger-check VERSION=v0.14.0`

## Definition of Done

- [x] `kml-mcp` の primary package artifact が公開済みであること
- [x] Registry / Hub metadata が package artifact と docs を正しく指していること
- [x] release runbook が package publication と Registry publication を説明していること
- [x] remote MCP transport が別 change のままであること

## 品質評価スコア

| 項目 | 最大 | 現在 | 根拠 |
| --- | ---: | ---: | --- |
| package artifact | 25 | 25 | MCPB manifest、build script、artifact smoke test を release gate に追加した。 |
| Registry metadata | 25 | 25 | `server.json` と release-time rendering / validation を追加した。 |
| publication workflow | 20 | 20 | Release workflow が MCPB asset upload と Registry publish を行う。 |
| documentation | 15 | 15 | README、distribution、MCP server、release runbook を更新した。 |
| verification | 15 | 15 | release-check、task ledger、diff check の対象に入れた。 |
| 合計 | 100 | 100 | release PR と release workflow で公開導線を固定した。 |
