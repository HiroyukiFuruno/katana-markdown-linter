# Tasks

## Definition of Ready

- [x] `v0-14-0-mcp-package-and-registry-publication` が完了していること
- [x] API-hosted LLM から直接 `kml` を呼ぶ具体ユースケースがあること
- [x] remote transport が必要で、library embedding では足りない理由 growth 記録されていること
- [x] auth、workspace access、write policy の最低条件が合意済みであること

## 1. Product Boundary

- [x] 1.1 text-only remote と workspace-backed remote の scope を分ける
- [x] 1.2 provider API から直接使う場合の制約を docs に記録する
- [x] 1.3 local stdio server と remote transport の違いを README / docs に記録する
- [x] 1.4 public hosted service を含めるか self-hosted のみにするか判断する

## 2. Transport Design

- [x] 2.1 remote MCP transport の実装方式を選ぶ
- [x] 2.2 `kml-mcp` feature 追加か `kml-mcp-remote` binary 分離かを決める
- [x] 2.3 tool capability metadata で remote 非対応操作を表現する
- [x] 2.4 request size limit、timeout、concurrency limit を定義する

## 3. Security

- [x] 3.1 auth model を定義する
- [x] 3.2 workspace-backed mode の tenant boundary を定義する
- [x] 3.3 write operation の explicit apply と audit log を実装する
- [x] 3.4 path traversal、symbolic path、ignored file policy を remote でも固定する
- [x] 3.5 anonymous write が不可能であることを test する

## 4. Implementation

- [x] 4.1 text-only remote tools を実装する
- [x] 4.2 workspace-backed read tools を実装する
- [x] 4.3 workspace-backed write tools は safety gate を満たす場合のみ実装する
- [x] 4.4 local stdio server と shared model を使う
- [x] 4.5 core library に MCP transport dependency が漏れないことを確認する

## 5. Verification And Docs

- [x] 5.1 remote transport integration test を追加する
- [x] 5.2 auth failure と size limit の test を追加する
- [x] 5.3 deployment docs を追加する
- [x] 5.4 README / docs / changelog / version metadata を `v0.15.0` に更新する

## Verification

- [x] remote transport integration test
- [x] auth and limit tests
- [x] `make mcp-stdio-smoke`
- [x] `make release-check VERSION=v0.15.0`
- [x] `git diff --check`

## Definition of Done

- [x] remote MCP transport の提供範囲が docs と capability metadata で一致していること
- [x] anonymous write が不可能であること
- [x] workspace-backed remote は local stdio と同等以上の path safety を持つこと
- [x] core crate が MCP-free のままであること
