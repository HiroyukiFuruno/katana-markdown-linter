# Tasks

## Definition of Ready

- [ ] `v0-14-0-mcp-package-and-registry-publication` が完了していること
- [ ] API-hosted LLM から直接 `kml` を呼ぶ具体ユースケースがあること
- [ ] remote transport が必要で、library embedding では足りない理由が記録されていること
- [ ] auth、workspace access、write policy の最低条件が合意済みであること

## 1. Product Boundary

- [ ] 1.1 text-only remote と workspace-backed remote の scope を分ける
- [ ] 1.2 provider API から直接使う場合の制約を docs に記録する
- [ ] 1.3 local stdio server と remote transport の違いを README / docs に記録する
- [ ] 1.4 public hosted service を含めるか self-hosted のみにするか判断する

## 2. Transport Design

- [ ] 2.1 remote MCP transport の実装方式を選ぶ
- [ ] 2.2 `kml-mcp` feature 追加か `kml-mcp-remote` binary 分離かを決める
- [ ] 2.3 tool capability metadata で remote 非対応操作を表現する
- [ ] 2.4 request size limit、timeout、concurrency limit を定義する

## 3. Security

- [ ] 3.1 auth model を定義する
- [ ] 3.2 workspace-backed mode の tenant boundary を定義する
- [ ] 3.3 write operation の explicit apply と audit log を実装する
- [ ] 3.4 path traversal、symbolic path、ignored file policy を remote でも固定する
- [ ] 3.5 anonymous write が不可能であることを test する

## 4. Implementation

- [ ] 4.1 text-only remote tools を実装する
- [ ] 4.2 workspace-backed read tools を実装する
- [ ] 4.3 workspace-backed write tools は safety gate を満たす場合のみ実装する
- [ ] 4.4 local stdio server と shared model を使う
- [ ] 4.5 core library に MCP transport dependency が漏れないことを確認する

## 5. Verification And Docs

- [ ] 5.1 remote transport integration test を追加する
- [ ] 5.2 auth failure と size limit の test を追加する
- [ ] 5.3 deployment docs を追加する
- [ ] 5.4 README / docs / changelog / version metadata を `v0.15.0` に更新する

## Verification

- [ ] remote transport integration test
- [ ] auth and limit tests
- [ ] `make mcp-stdio-smoke`
- [ ] `make release-check VERSION=v0.15.0`
- [ ] `git diff --check`

## Definition of Done

- [ ] remote MCP transport の提供範囲が docs と capability metadata で一致していること
- [ ] anonymous write が不可能であること
- [ ] workspace-backed remote は local stdio と同等以上の path safety を持つこと
- [ ] core crate が MCP-free のままであること
