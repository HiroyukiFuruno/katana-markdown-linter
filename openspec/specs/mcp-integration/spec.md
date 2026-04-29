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

### Requirement: MCP metadata tools SHALL accept explicit locale

MCP metadata tools は、client が明示した locale に基づいて metadata を localized できなければならない（SHALL）。

#### Scenario: localized rule list を取得する

- **WHEN** MCP client が `rule_list` に supported locale を渡す
- **THEN** system は lenient library resolver で locale を解決する
- **THEN** response は resolved locale と localized rule descriptions を含む
- **THEN** locale が省略された場合は English を返す
- **THEN** supported locale の description は English canonical description の単純コピーではない

#### Scenario: localized rule detail を取得する

- **WHEN** MCP client が `rule_get` に rule ID と supported locale を渡す
- **THEN** system は該当 rule の localized description を返す
- **THEN** unknown rule handling は existing behavior と同じ error contract を保つ
- **THEN** localized description は English canonical description の単純コピーではない

### Requirement: MCP rule documentation tool SHALL return localized Markdown

MCP `rule_doc_get` tool は、client が明示した locale に基づいて rule documentation Markdown を localized できなければならない（SHALL）。

#### Scenario: localized rule document を取得する

- **WHEN** MCP client が `rule_doc_get` に rule ID と supported locale を渡す
- **THEN** response は resolved locale と localized Markdown content を含む
- **THEN** content は rule ID、設定 key、example code block を保持する
- **THEN** content の prose は English document の単純コピーではない

#### Scenario: unsupported locale の rule document を取得する

- **WHEN** MCP client が `rule_doc_get` に unsupported locale を渡す
- **THEN** system は lenient library resolver で English に fallback する
- **THEN** response は `locale: "en"` と English Markdown content を返す

### Requirement: MCP diagnostic and config responses SHALL localize when requested

MCP check/config tools は、client が明示した locale に基づいて diagnostic と config validation error を localized できなければならない（SHALL）。

#### Scenario: check_text diagnostic を localized 表示する

- **WHEN** MCP client が `check_text` に Markdown content と Japanese locale を渡す
- **THEN** response diagnostics は Japanese message を含む
- **THEN** response diagnostics は stable message ID と parameters を保持する

#### Scenario: config_validate error を localized 表示する

- **WHEN** MCP client が `config_validate` に invalid config と Japanese locale を渡す
- **THEN** response errors は Japanese message を含む
- **THEN** response errors は stable kind、message ID、parameters、expected/actual/allowed metadata を含む

### Requirement: system SHALL not expose write-capable MCP file tools in the prototype

システムは、path allowlist と dry-run write policy が実装されるまで write-capable MCP file tools を公開してはならない（SHALL NOT）。

#### Scenario: prototype tool list を確認する

- **WHEN** developer が `kml-mcp` の公開 tool を確認する
- **THEN** system は `fix_files` を公開しない
- **THEN** system は workspace path を受け取って file を書き換える tool を公開しない

### Requirement: remote MCP transport SHALL be explicitly separated from stdio

The project SHALL distinguish remote MCP transport from the local stdio MCP server.

#### Scenario: documentation describes MCP support

- **GIVEN** local stdio MCP support is available
- **WHEN** documentation describes API-hosted LLM usage
- **THEN** the documentation SHALL state that provider API direct usage requires remote transport
- **AND** the documentation SHALL NOT present local stdio as a remote MCP connector

### Requirement: remote MCP server SHALL expose only safe capabilities by default

The remote MCP server SHALL default to non-mutating operations unless explicit
write safety requirements are implemented.

#### Scenario: remote capabilities are listed

- **GIVEN** a remote MCP client calls `tools/list`
- **WHEN** the server returns available tools
- **THEN** text-only diagnostics and metadata tools MAY be available
- **AND** workspace write tools SHALL be absent or marked unavailable unless auth, audit, and explicit apply are enabled

### Requirement: workspace-backed remote mode SHALL enforce authenticated workspace boundaries

Workspace-backed remote mode SHALL require authenticated access and server-side
workspace boundaries.

#### Scenario: remote file tool resolves a path

- **GIVEN** a remote session has an assigned workspace root
- **WHEN** a file tool receives a path
- **THEN** the path SHALL resolve under the assigned workspace root
- **AND** root escape attempts SHALL be rejected
- **AND** anonymous sessions SHALL NOT perform file writes

### Requirement: core library SHALL remain independent of MCP transport

The core `katana-markdown-linter` library SHALL remain independent of local or
remote MCP transport dependencies.

#### Scenario: library consumer builds without MCP features

- **GIVEN** a consumer depends on `katana-markdown-linter` as a library
- **WHEN** the consumer builds without MCP features
- **THEN** MCP transport dependencies SHALL NOT be required
- **AND** public lint / fix APIs SHALL remain available
