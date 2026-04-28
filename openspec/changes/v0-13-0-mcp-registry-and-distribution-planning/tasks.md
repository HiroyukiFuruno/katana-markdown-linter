# Tasks

## Definition of Ready

- [x] `v0.12.8` で stable score が 90 点以上であること
- [x] `v0.12.8` の hard blocker が 0 件であること
- [x] `v0.12.8` をユーザーが安定版として受け入れていること
- [x] `v0.12.9` の public confidence score が 90 点以上であること
- [x] `v0.12.9` の release-blocking issue が 0 件であること
- [x] `kml-mcp` が stdio server として build / install できること
- [x] `make mcp-stdio-smoke` が存在し、stdio 経由の tool call を検証できること
- [x] file mutation は preview と explicit apply に分離済みであること
- [x] core crate に MCP dependency を追加しない方針が確認済みであること
- [x] この change では公開せず、計画と gate に限定する方針であること

## 1. Distribution Research

- [x] 1.1 MCPB package の仕様、client support、署名や更新方式を確認する
- [x] 1.2 OCI image / GHCR で stdio transport と workspace mount が成立するか確認する
- [x] 1.3 npm / PyPI wrapper が必要になる条件と ownership cost を整理する
- [x] 1.4 crates.io binary install と Registry package type の関係を文書化する
- [x] 1.5 package type ごとの install UX、security boundary、CI smoke test を比較する

## 2. Registry Metadata Plan

- [x] 2.1 `server.json` draft を作成する
- [x] 2.2 metadata に含める command、args、package reference、docs URL を決める
- [x] 2.3 ownership verification と publish credential の扱いを整理する
- [x] 2.4 Registry aggregator に拾われるための metadata 表現を確認する
- [x] 2.5 Registry 公開前に確認する manual checklist を作成する

## 3. Public Readiness

- [x] 3.1 workspace access policy の docs を再レビューする
- [x] 3.2 check / fix coverage 表現が過剰に見えないか確認する
- [x] 3.3 security review checklist を作成する
- [x] 3.4 remote MCP transport が必要な条件を `v0-15-0-remote-mcp-transport` に分離する
- [x] 3.5 package artifact 実装を `v0-14-0-mcp-package-and-registry-publication` に分離する

## 4. 品質評価スコア

Release 条件は `100/100`、Registry / Hub publish 実行 `0` とする。
score が `100` 未満、または gate が失敗した場合は、この `tasks.md` に追加 task を記録し、修正して同じ gate を再実行する。

| 項目 | 配点 | 現在 | 完了条件 |
| --- | ---: | ---: | --- |
| 配布方式調査 | 20 | 20 | MCPB、OCI、crates.io、npm、PyPI、NuGet の扱いを `docs/mcp-distribution-plan.md` に記録 |
| Registry metadata | 20 | 20 | `server.json` 草案、package reference、ownership verification を `docs/mcp-server.md` に記録 |
| safety gate | 20 | 20 | workspace root、preview/apply 分離、remote transport 非対象を docs に記録 |
| follow-up split | 15 | 15 | package publish は `v0.14.0`、remote transport は `v0.15.0` に分離 |
| publish deferral | 15 | 15 | この change では Registry / Hub 登録を実行していないことを記録 |
| 品質 gate | 10 | 10 | `mcp-stdio-smoke`、`ast-lint`、`dogfood`、`release-task-ledger-check`、`git diff --check` が成功 |
| 合計 | 100 | 100 | 100/100 で release-ready |

## Verification

- [x] `make mcp-stdio-smoke`
- [x] `make ast-lint`
- [x] `make dogfood`
- [x] `make release-task-ledger-check VERSION=v0.13.0`
- [x] Registry / Hub 登録をこの change で実行していないこと
- [x] docs に「現時点では公開 deferred」と理由が記録されていること
- [x] `git diff --check`

## Definition of Done

- [x] MCP Registry / Hub 登録の実施条件が明確であること
- [x] package type の第一候補と fallback 条件が決まっていること
- [x] `server.json` draft が後続 change で実装できる粒度になっていること
- [x] 公開しない判断が将来の作業者に伝わること
