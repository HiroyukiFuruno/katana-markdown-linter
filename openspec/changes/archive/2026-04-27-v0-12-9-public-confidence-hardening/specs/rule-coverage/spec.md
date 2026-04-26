## ADDED Requirements

### Requirement: external corpus findings SHALL be classified before release expansion

外部 corpus で見つかった finding は、配布展開へ進む前に分類されなければならない（SHALL）。

#### Scenario: external finding を分類する

- **WHEN** developer が KatanA corpus または curated public confidence fixture の diagnostic を確認する
- **THEN** system は finding を `true-positive`、`false-positive`、`false-negative`、`unsafe-fix-risk`、`fmt-policy-gap`、`perf-regression` のいずれかに分類する
- **THEN** system は release-blocking issue と non-blocking follow-up を分ける
- **THEN** system は未分類の high-risk finding が残る場合、release を進めない

### Requirement: external precision fixes SHALL become regression tests

外部 corpus で見つかった精度修正は、回帰テストとして固定されなければならない（SHALL）。

#### Scenario: external precision issue を修正する

- **WHEN** developer が external corpus 由来の false-positive または false-negative を修正する
- **THEN** system は rule-local test を追加する
- **THEN** system は document-level fixture または curated public confidence fixture に再発条件を追加する
- **THEN** system は unrelated diagnostics が増えていないことを確認する
