## MODIFIED Requirements

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

## ADDED Requirements

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
