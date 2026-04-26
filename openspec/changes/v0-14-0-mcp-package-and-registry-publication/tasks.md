# Tasks

## Definition of Ready

- [ ] `v0-13-0-mcp-registry-and-distribution-planning` が完了していること
- [ ] package type の第一候補と fallback 条件が決まっていること
- [ ] `server.json` draft が存在すること
- [ ] security review checklist が存在すること
- [ ] Registry / Hub 公開に必要な credential と ownership 確認方法が決まっていること

## 1. Package Artifact

- [ ] 1.1 選定済み package type 用の manifest / build script を追加する
- [ ] 1.2 `kml-mcp` binary が `mcp` feature 付きで artifact に含まれることを固定する
- [ ] 1.3 artifact から `tools/list` を実行する smoke test を追加する
- [ ] 1.4 artifact から workspace-safe `check_file` を実行する smoke test を追加する
- [ ] 1.5 artifact install path と local development install path の違いを docs に記録する

## 2. Registry Metadata

- [ ] 2.1 `server.json` を repository に追加する
- [ ] 2.2 `server.json` の schema validation を追加する
- [ ] 2.3 metadata が primary install path、source repository、docs URL を指すことを検証する
- [ ] 2.4 metadata が remote MCP support を過剰に主張しないことを確認する
- [ ] 2.5 Registry aggregator に必要な tags / categories があれば追加する

## 3. Release Runbook

- [ ] 3.1 package artifact build 手順を release runbook に追加する
- [ ] 3.2 Registry / Hub publish 手順を release runbook に追加する
- [ ] 3.3 公開後 verification を release runbook に追加する
- [ ] 3.4 immutable artifact の修正時は patch version を使う方針を明記する
- [ ] 3.5 failed publish の retry 判断を既存 release policy と揃える

## 4. Publication

- [ ] 4.1 release gate で package artifact smoke test を通す
- [ ] 4.2 security checklist を完了する
- [ ] 4.3 Registry / Hub publish を実行する
- [ ] 4.4 公開 listing から install と documentation URL を確認する
- [ ] 4.5 README / docs / changelog / version metadata を `v0.14.0` に更新する

## Verification

- [ ] `make mcp-stdio-smoke`
- [ ] package artifact smoke test
- [ ] `server.json` schema validation
- [ ] `make release-check VERSION=v0.14.0`
- [ ] Registry / Hub listing verification
- [ ] `git diff --check`

## Definition of Done

- [ ] `kml-mcp` の primary package artifact が公開済みであること
- [ ] Registry / Hub metadata が package artifact と docs を正しく指していること
- [ ] release runbook が package publication と Registry publication を説明していること
- [ ] remote MCP transport が別 change のままであること
