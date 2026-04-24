## ADDED Requirements

### Requirement: system SHALL publish a rule coverage dashboard

システムは、rule ごとの coverage と compatibility 状態を一覧できる dashboard を公開しなければならない（SHALL）。

#### Scenario: dashboard を生成する

- **WHEN** developer が coverage dashboard generation を実行する
- **THEN** system は rule ID ごとの check、fix、config、edge、golden comparison、known delta の状態を出力する
- **THEN** system は missing coverage と failing golden comparison を区別して表示する
- **THEN** system は dashboard を Markdown または JSON の再利用可能な形式で生成する

### Requirement: rule coverage gate SHALL fail on unknown golden deltas

rule coverage gate は、許可されていない upstream golden delta を失敗として扱わなければならない（SHALL）。

#### Scenario: unknown delta を検出する

- **WHEN** golden comparison が known delta にない差分を検出する
- **THEN** system は gate を failure にする
- **THEN** system は該当 rule、fixture、expected、actual を report する
