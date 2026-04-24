## ADDED Requirements

### Requirement: system SHALL provide an optional MCP server binary

システムは、通常の library/CLI 利用者に MCP dependency cost を負わせずに MCP server を利用できる optional binary を提供しなければならない（SHALL）。

#### Scenario: MCP feature を無効にして build する

- **WHEN** developer が default feature set で build または test を実行する
- **THEN** system は MCP SDK を core crate の public API 実装に要求しない
- **THEN** system は `kml` CLI を MCP feature なしで利用できる

#### Scenario: MCP server を build する

- **WHEN** developer が `cargo build --bin kml-mcp --features mcp --locked` を実行する
- **THEN** system は `kml-mcp` binary を build する
- **THEN** system は stdio transport で MCP server として起動できる binary を提供する

### Requirement: system SHALL expose read-only text-first MCP tools

システムは、prototype scope として workspace file access を伴わない text-first MCP tools を公開しなければならない（SHALL）。

#### Scenario: text content を lint する

- **WHEN** MCP client が `check_text` tool に Markdown content を渡す
- **THEN** system は diagnostics を structured JSON として返す
- **THEN** diagnostics は rule ID、message、severity、range、fix availability を含む

#### Scenario: text content の fix preview を取得する

- **WHEN** MCP client が `fix_text` tool に Markdown content を渡す
- **THEN** system は fixed content と applied fix count を返す
- **THEN** system は workspace file を変更しない

#### Scenario: config を validate する

- **WHEN** MCP client が `config_validate` tool に JSON config object を渡す
- **THEN** system は config validity と structured errors を返す
- **THEN** system は unknown rule、unknown property、type mismatch、enum mismatch を区別できる error kind を返す

#### Scenario: rule catalog を参照する

- **WHEN** MCP client が `rule_list` または `rule_get` tool を呼び出す
- **THEN** system は rule ID、name、description、docs URL、fixable flag を含む metadata を返す

### Requirement: system SHALL not expose write-capable MCP file tools in the prototype

システムは、path allowlist と dry-run write policy が実装されるまで write-capable MCP file tools を公開してはならない（SHALL NOT）。

#### Scenario: prototype tool list を確認する

- **WHEN** developer が `kml-mcp` の公開 tool を確認する
- **THEN** system は `fix_files` を公開しない
- **THEN** system は workspace path を受け取って file を書き換える tool を公開しない
