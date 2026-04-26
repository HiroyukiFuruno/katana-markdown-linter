## ADDED Requirements

### Requirement: stable scoring SHALL include performance stability

stable scoring は、performance stability を評価しなければならない（SHALL）。

#### Scenario: performance stability を採点する

- **WHEN** developer が stable score を算出する
- **THEN** system は parser / context migration 後の benchmark を 20 点満点で採点する
- **THEN** system は link-heavy、inline-code-heavy、reference-heavy、parser index construction の結果を含める
- **THEN** system は説明不能な重大 regression を hard blocker として扱う
- **THEN** system は benchmark 数値だけを目的に rule semantics を弱めない
