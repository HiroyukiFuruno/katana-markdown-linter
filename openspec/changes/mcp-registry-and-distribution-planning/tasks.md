# Tasks

## Definition of Ready

- [x] `kml-mcp` が stdio server として build / install できること
- [x] MCP Registry が metadata registry であり artifact hosting ではないことを確認済み
- [x] 現状では crates.io package を直接公式 Registry package type として扱えないことを確認済み
- [x] 現時点では公開を急がず計画のみ作る方針で合意済み

## 1. Distribution Research

- [ ] 1.1 MCPB package の仕様と client support を確認する
- [ ] 1.2 GHCR / Docker distribution で workspace mount と stdio transport が成立するか確認する
- [ ] 1.3 npm / PyPI wrapper が必要になる条件を整理する
- [ ] 1.4 `server.json` に必要な fields と ownership verification を整理する

## 2. Registry Plan

- [ ] 2.1 `server.json` draft を作成する
- [ ] 2.2 package type ごとの install UX を比較する
- [ ] 2.3 Registry aggregator に拾われるための metadata 表現を決める
- [ ] 2.4 publish 手順を release runbook に追加するか判断する

## 3. Public Readiness

- [ ] 3.1 公開前 security review checklist を作成する
- [ ] 3.2 workspace access policy の user-facing docs を再レビューする
- [ ] 3.3 check / fix coverage 表現が過剰に見えないか確認する
- [ ] 3.4 remote HTTP MCP transport を別 change に分けるか判断する

## Verification

- [x] `mcp-stdio-smoke` が release gate に含まれていること
- [ ] Registry / Hub 登録はこの change で実行していないこと
- [ ] docs に「現時点では公開 deferred」と理由が記録されていること

## Definition of Done

- [ ] MCP Registry / Hub 登録の実施条件が明確であること
- [ ] package type の第一候補が決まっていること
- [ ] 公開しない判断が将来の作業者に伝わること
