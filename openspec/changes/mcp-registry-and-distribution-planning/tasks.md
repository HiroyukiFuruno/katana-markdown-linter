# Tasks

## Definition of Ready

- [x] `kml-mcp` が stdio server として build / install できること
- [x] `make mcp-stdio-smoke` が存在し、stdio 経由の tool call を検証できること
- [x] file mutation は preview と explicit apply に分離済みであること
- [x] core crate に MCP dependency を追加しない方針が確認済みであること
- [x] この change では公開せず、計画と gate に限定する方針であること

## 1. Distribution Research

- [ ] 1.1 MCPB package の仕様、client support、署名や更新方式を確認する
- [ ] 1.2 OCI image / GHCR で stdio transport と workspace mount が成立するか確認する
- [ ] 1.3 npm / PyPI wrapper が必要になる条件と ownership cost を整理する
- [ ] 1.4 crates.io binary install と Registry package type の関係を文書化する
- [ ] 1.5 package type ごとの install UX、security boundary、CI smoke test を比較する

## 2. Registry Metadata Plan

- [ ] 2.1 `server.json` draft を作成する
- [ ] 2.2 metadata に含める command、args、package reference、docs URL を決める
- [ ] 2.3 ownership verification と publish credential の扱いを整理する
- [ ] 2.4 Registry aggregator に拾われるための metadata 表現を確認する
- [ ] 2.5 Registry 公開前に確認する manual checklist を作成する

## 3. Public Readiness

- [ ] 3.1 workspace access policy の docs を再レビューする
- [ ] 3.2 check / fix coverage 表現が過剰に見えないか確認する
- [ ] 3.3 security review checklist を作成する
- [ ] 3.4 remote MCP transport が必要な条件を `v0-15-0-remote-mcp-transport` に分離する
- [ ] 3.5 package artifact 実装を `v0-14-0-mcp-package-and-registry-publication` に分離する

## Verification

- [ ] `make mcp-stdio-smoke`
- [ ] `make ast-lint`
- [ ] Registry / Hub 登録をこの change で実行していないこと
- [ ] docs に「現時点では公開 deferred」と理由が記録されていること
- [ ] `git diff --check`

## Definition of Done

- [ ] MCP Registry / Hub 登録の実施条件が明確であること
- [ ] package type の第一候補と fallback 条件が決まっていること
- [ ] `server.json` draft が後続 change で実装できる粒度になっていること
- [ ] 公開しない判断が将来の作業者に伝わること
