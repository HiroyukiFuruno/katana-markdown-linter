## ADDED Requirements

### Requirement: context-sensitive rules SHALL declare their context source

context-sensitive rule は、自身がどの context source を根拠に評価するかを分類しなければならない（SHALL）。

#### Scenario: rule migration を棚卸しする

- **WHEN** developer が context-sensitive rule を確認する
- **THEN** system は rule を `parser-backed`、`document-context-backed`、`line-local-by-spec`、`future-work` のいずれかに分類する
- **THEN** system は `future-work` の理由と次の解消条件を tasks に記録する

### Requirement: migrated rules SHALL reuse shared structural indexes

migrated rule は、構文除外判定を rule-local の重複 scan だけに依存してはならない（SHALL NOT）。

#### Scenario: migrated rule を実行する

- **WHEN** migrated rule が Markdown 風テキストを評価する
- **THEN** system は `DocumentContext` または shared parser token から context を取得する
- **THEN** system は fenced code block、inline code、HTML、table、reference definition を通常本文と混同しない
