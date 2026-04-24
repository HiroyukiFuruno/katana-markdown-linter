## Context

この crate は library-first として分離され、CLI は後続 phase で強化された。現時点では release gate と AST lint は整っているが、`kml` 自身をこの repository の Markdown に日常利用する契約は未定義である。

dogfooding は単なる lint 実行ではなく、CLI の再帰走査、ignore、config discovery、JSON output、fix の体験を実利用で確認するための運用である。歴史的な OpenSpec archive まで自動修正すると意図しない churn が大きいため、既定の対象は現在の文書と仕様に寄せる。

## Goals / Non-Goals

**Goals:**

- `kml` を repository 自身に対して再帰実行できる make target を定義する
- check-only と fix の責務を分ける
- dogfood 結果と UX issue を残せる場所を定義する
- Rust 組み込み API の最小利用例を実行可能な形で追加する

**Non-Goals:**

- archived OpenSpec 文書を大量に整形し直すこと
- 互換性 oracle として upstream markdownlint を使うこと
- MCP server を実装すること
- CLI の大規模 redesign を同時に行うこと

## Decisions

- Dogfood target は `README.md`、`docs/**/*.md`、`openspec/specs/**/*.md`、active change の `proposal.md` / `design.md` / `tasks.md` を中心にする。`openspec/changes/archive/**` は既定除外とし、必要時だけ明示的に走査する。
- `make dogfood` は check-only とする。自動修正は `make dogfood-fix` のような明示 target に分け、差分確認を前提にする。
- CI 必須化はこの change の完了条件にしない。まず local dogfood と report を安定させ、次の quality gate change で CI 化を判断する。
- Public API examples は docs-only snippet ではなく、`examples/` か integration test で compile される形を優先する。
- UX issue は散文ではなく、再現 command、期待、実際、対応判断を持つ Markdown report として残す。

## Risks / Trade-offs

- dogfood 実行で既存 Markdown の違反が大量に出る可能性がある。
  Mitigation: 初回は report 化し、fix は安全な範囲に限定する。

- archived OpenSpec 文書を除外すると全 Markdown coverage ではなくなる。
  Mitigation: archive は履歴保全を優先し、別 target で確認できるようにする。

- examples が public API の変更に追従できない可能性がある。
  Mitigation: examples を compile 対象に含める。
