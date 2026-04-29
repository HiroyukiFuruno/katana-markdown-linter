## Why

KatanA 本体が 10 言語を持つ一方で、kml の rule metadata と rule document は English / Japanese だけに留まっている。
v0.16.0 で editor / MCP 利用口が広がったため、v0.16.1 では外部から呼び出す口（API）で返す説明文とルール本文 Markdown の i18n 欠落を閉じる。

## What Changes

- KatanA 本体と同じ `en`, `ja`, `zh-CN`, `zh-TW`, `ko`, `pt`, `fr`, `de`, `es`, `it` を supported locale として扱う。
- Rule description は全 supported locale に対して自然な翻訳を返す。
- Rule document Markdown は全 supported locale に対して自然な翻訳を返す。
- English の単純コピーは翻訳欠落として扱い、品質 gate で検出する。
- Library API、CLI rule introspection、local / remote MCP tool は同じ locale 解決と fallback policy を共有する。
- Unsupported locale は既存方針を維持し、CLI explicit 指定は hard error、library / MCP は English fallback とする。

## Capabilities

### New Capabilities

- None.

### Modified Capabilities

- `i18n-api`: supported locale set、localized description coverage、translation quality gate を 10 言語へ拡張する。
- `cli`: `kml rule` / `kml rule <id>` の locale 対応範囲を 10 言語へ拡張する。
- `mcp-integration`: `rule_list` / `rule_get` / `rule_doc_get` が 10 言語の localized content を返す契約を持つ。
- `rule-doc-drift`: localized rule document Markdown の存在と翻訳欠落を検査対象に含める。

## Impact

- `src/i18n.rs` の `Locale`、resolver、catalog、translation coverage gate。
- `src/catalog.rs` / `src/types.rs` の localized metadata API。
- `src/upstream/document.rs` と `upstream_docs/<locale>/md*.md`。
- `src/bin/kml_mcp/**` の rule metadata / document response。
- CLI / MCP / AST lint tests。
- README と MCP documentation の locale support description。
