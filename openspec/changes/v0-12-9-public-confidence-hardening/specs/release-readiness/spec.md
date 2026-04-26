## ADDED Requirements

### Requirement: distribution expansion SHALL wait for v0.12.9 public confidence completion

配布展開は、`v0.12.9` の public confidence hardening が完了するまで進めてはならない（SHALL NOT）。

#### Scenario: v0.13.0 に進む

- **WHEN** developer が `v0.13.0` の配布計画に着手する
- **THEN** system は `v0.12.8` の stable score が 90 点以上であることを確認する
- **THEN** system は `v0.12.8` の hard blocker が 0 件であることを確認する
- **THEN** system は `v0.12.8` の user acceptance が記録されていることを確認する
- **THEN** system は `v0.12.9` の public confidence score が 90 点以上であることを確認する
- **THEN** system は `v0.12.9` の release-blocking issue が 0 件であることを確認する

### Requirement: public confidence score SHALL be recorded before release

`v0.12.9` release 前に、public confidence score が記録されなければならない（SHALL）。

#### Scenario: public confidence score を評価する

- **WHEN** developer が `v0.12.9` の release 前確認を行う
- **THEN** system は external corpus confidence、precision regression、command convergence、performance stability、release reproducibility を採点する
- **THEN** system は hard blocker の有無を記録する
- **THEN** system は known limitation と non-blocking follow-up を記録する
