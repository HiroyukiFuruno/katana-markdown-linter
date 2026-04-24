# Tasks

## DoR

- [x] active OpenSpec change がこの change のみであることを確認する
- [x] public owned rule vector APIs を破壊しないことを design に明記する
- [x] rule behavior、CLI output、exit code を変更しないことを確認する
- [x] performance baseline refresh を同じ change に含めることを確認する

## Implementation

- [x] `MarkdownRule` に cached registry で必要な thread-safety bound を追加する
- [x] official rule metadata registry の cached borrowed accessor を追加する
- [x] user-configurable rule metadata registry の cached borrowed accessor を追加する
- [x] `evaluate_all` の cached dispatch を測定し、regression 回避のため既存 dispatch を維持する
- [x] `RuleCatalog::build` を cached source から clone する形に変更する
- [x] `implemented_rules` を cached official metadata registry に切り替える
- [x] `options_from_config` を cached user-configurable metadata registry に切り替える
- [x] `MarkdownLintConfig` に cached-rule validation path を追加する
- [x] CLI config validation を cached-rule validation path に切り替える
- [x] MCP/config benchmark の config validation を cached-rule validation path に切り替える
- [x] registry/cache behavior preservation tests を追加する
- [x] `tests/fixtures/perf-baseline.json` を更新する
- [x] OpenSpec main spec に delta を同期する

## DoD

- [x] `make perf-check` が成功し median comparison を出力する
- [x] `make check` が成功する
- [x] `make mcp-build` が成功する
- [x] `openspec status --change performance-rule-registry-cache --json` で apply-ready である
- [x] active OpenSpec change が archive 可能な状態である
