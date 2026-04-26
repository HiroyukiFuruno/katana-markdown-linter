## ADDED Requirements

### Requirement: context-sensitive rule evaluation SHALL not rely on isolated line text alone

context-sensitive rule evaluation は、単一行文字列だけに依存してはならない（SHALL NOT）。

#### Scenario: code block 内の Markdown 風テキストを評価する

- **WHEN** rule が table、link、emphasis、command prompt、heading、list に見える行を評価する
- **THEN** system はその行が fenced code block、indented code block、inline code span、HTML block、table block のどれに属するか確認する
- **THEN** system は context 上除外すべき行を通常本文として診断しない

### Requirement: document context SHALL support both backtick and tilde fences

document context は backtick fence と tilde fence の両方を扱わなければならない（SHALL）。

#### Scenario: tilde fence を解析する

- **WHEN** Markdown content に `~~~mermaid` または `~~~` fenced code block が含まれる
- **THEN** system はその範囲を code block として記録する
- **THEN** system は同じ fence kind かつ opening fence 以上の長さの closing fence だけで block を閉じる
- **THEN** system は code block 内の pipe、URL、list marker、dollar prompt を通常本文として診断しない
