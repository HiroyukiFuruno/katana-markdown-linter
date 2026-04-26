# Design

## Transport Boundary

remote MCP transport は local stdio server と同じ binary に無理に混ぜない。
実装候補は次のどちらかにする。

1. `kml-mcp` に remote transport feature を追加する
2. `kml-mcp-remote` binary を分ける

core crate は引き続き MCP を知らない。

## Product Modes

remote MCP は 2 つの mode を分けて扱う。

### Text-Only Remote

`check_text`、`fix_text`、`config_validate`、`rule_list`、`rule_get` のように、
request body 内の text だけを扱う。

この mode は workspace mount を必要としないため、hosted service にしやすい。
ただし repository file を直接読むとは説明しない。

### Workspace-Backed Remote

remote server 側に workspace が存在し、file / directory tools を実行する。

この mode は auth、tenant boundary、workspace provisioning、write policy が必要になる。
default は read-only とし、write は explicit apply と audit log を必須にする。

## Auth And Access

remote transport は anonymous write を許可しない。

- text-only read operations は rate limit と request size limit を持つ
- workspace-backed operations は authenticated session を必須にする
- file write は explicit apply と audit log を必須にする
- workspace root は server-side configuration として固定する

## Compatibility With Local Tools

tool name と response schema は local stdio server とできるだけ揃える。
ただし、remote で安全に提供できない fields や operations は capability metadata で明示的に非対応にする。

## Deployment Options

初期実装は self-hosted を優先する。
public hosted service は運用責務が大きいため、この change では採用判断だけに留める。

deployment docs には以下を含める。

- TLS 終端
- auth 設定
- request size limit
- workspace mount / provisioning
- log と audit policy
- remote で提供しない tool の一覧

## Failure Policy

remote transport が安全条件を満たせない場合は、text-only remote だけを出すか、実装を延期する。
local stdio server の Registry 公開を remote support の代替として扱わない。
