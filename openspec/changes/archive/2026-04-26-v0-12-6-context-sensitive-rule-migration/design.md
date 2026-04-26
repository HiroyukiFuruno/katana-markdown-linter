# Design

## Migration Model

rule を次の分類で扱う。

| Category | Meaning |
| --- | --- |
| `parser-backed` | shared parser token を主要根拠にする |
| `document-context-backed` | heading / table / code block など `DocumentContext` index を主要根拠にする |
| `line-local-by-spec` | rule 仕様上、単一行判定で十分 |
| `future-work` | AST or parser 拡張が必要で、この change では移譲しない |

`future-work` は放置ではなく、理由と次の解消条件を tasks に残す。

## Target Rule Families

対象は context-sensitive rule 全体とする。
特に次を重点確認する。

- URL / link / reference: `MD034`、`MD051`、`MD052`、`MD053`、`MD054`、`MD059`
- HTML / inline content: `MD033`、`MD044`
- emphasis / strong / code span: `MD037`、`MD038`、`MD039`、`MD049`、`MD050`
- table: `MD055`、`MD056`、`MD060`
- list / command prompt / reversed link: `MD011`、`MD014`、`MD029`
- heading / code fence: `MD013`、`MD026`、`MD031`、`MD046`、`MD048`

## Test Shape

各 rule family は、rule-local test と document-level fixture の両方を持つ。

document-level fixture は以下を混ぜる。

- fenced code block
- inline code
- HTML block / inline HTML
- table
- nested list
- reference link
- autolink
- Markdown に見えるが通常本文ではない文字列

## Safety Policy

precision migration は fixed content を不用意に変えない。
fix がある rule は、migration 前後で fix range と replacement の意図が一致することを確認する。

## Performance Policy

migration は parser / context index の再利用を増やす方向で行う。
rule ごとの repeated scan が増える場合は、実装前に task へ理由を残す。
