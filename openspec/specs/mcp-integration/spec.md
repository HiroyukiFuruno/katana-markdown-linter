## Purpose

MCP integration defines how `kml` can be exposed to agents without coupling the core linting crate to MCP.

## Requirements

### Requirement: MCP integration SHALL preserve core crate independence

MCP integration は、core linting crate を MCP protocol や consuming application から独立させなければならない（SHALL）。

#### Scenario: core crate boundary を確認する

- **WHEN** developer が MCP integration design を確認する
- **THEN** system は core crate が MCP SDK に依存しない構成を示す
- **THEN** system は MCP server を adapter layer として定義する
- **THEN** system は KatanA 固有の責務を linter 側に持ち込まない

### Requirement: MCP evaluation SHALL define tool and resource candidates

MCP evaluation は、公開候補となる tools と resources を明確に定義しなければならない（SHALL）。

#### Scenario: tool candidate を確認する

- **WHEN** developer が MCP tool candidate を確認する
- **THEN** system は check、fix、config validation、rule introspection の候補を列挙する
- **THEN** system は read-only tool と write-capable tool を区別する

#### Scenario: resource candidate を確認する

- **WHEN** developer が MCP resource candidate を確認する
- **THEN** system は rule catalog、config summary、coverage dashboard などの read-only resource 候補を列挙する
- **THEN** system は prompt を初期必須対象に含めるかを明示する

### Requirement: MCP evaluation SHALL define workspace write safety policy

MCP evaluation は、workspace write を伴う operation の安全方針を定義しなければならない（SHALL）。

#### Scenario: file fix を評価する

- **WHEN** MCP 経由で file fix を提供するか検討する
- **THEN** system は explicit opt-in、path allowlist、dry-run diff の必要性を評価する
- **THEN** system は write operation を read-only operation と同じ扱いにしない

### Requirement: MCP evaluation SHALL produce an implementation recommendation

MCP evaluation は、実装に進むかどうかを判断できる recommendation を出力しなければならない（SHALL）。

#### Scenario: recommendation を確認する

- **WHEN** evaluation が完了する
- **THEN** system は採用 / 保留 / 不採用の判断を示す
- **THEN** system は採用する場合の crate 構成、binary 名、SDK 候補、DoR、DoD を提示する
- **THEN** system は保留または不採用の場合の理由を提示する
