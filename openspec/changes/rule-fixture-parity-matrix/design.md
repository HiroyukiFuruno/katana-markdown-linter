## Context

phase5 で upstream rule document parsing と drift tracking は導入済みである。
ただし、現在の parser は metadata / property drift を主目的としており、rule behavior examples を unit test fixture として使う contract は未定義である。

## Goals / Non-Goals

**Goals:**

- 公式 markdownlint rule document から test matrix を生成または補助生成できる
- ruleごとの check / fix / config / edge coverage を可視化できる
- 自動生成できないものを `manual_required` として残し、実装時の曖昧さをなくす
- fixture matrix を後続changeのDoRとして使える

**Non-Goals:**

- このchangeでは全ruleのcheck/fix実装を完了しない
- rumdl / mado の実装をコピーしない
- upstream markdownlint の全unit testをそのまま取り込まない

## Decisions

### 1. Matrix schema を固定する

各ruleは以下の項目を持つ。

- `rule_id`
- `aliases`
- `tags`
- `parameters`
- `fixable`
- `check_pass`
- `check_fail`
- `fix`
- `config_valid`
- `config_invalid`
- `edge`
- `manual_required`

### 2. 公式docを正、local fixtureを派生物にする

`DavidAnson/markdownlint` default branch の document を source of truth とする。
local fixture は生成物または補助編集物であり、drift check で stale を検出する。

### 3. Summary は人間向けと機械向けを両方出す

JSON は後続テスト実行に使う。
Markdown summary は作業者がrule別進捗を確認するために使う。

## Risks / Trade-offs

- 公式docのexample表現はruleごとに不均一なため、全てを完全自動化できない可能性がある
- default branch追従のため、upstream変更でfixture matrixが変わる
- matrix生成の粒度を細かくしすぎると、実装よりfixture整備が重くなる

## Migration Plan

1. fixture matrix schemaを追加する
2. upstream doc parserにbehavior/example extractionを追加する
3. rule別matrixを生成する
4. JSON / Markdown summaryを出力する
5. 生成結果をast-lintまたはignored testで検証する
