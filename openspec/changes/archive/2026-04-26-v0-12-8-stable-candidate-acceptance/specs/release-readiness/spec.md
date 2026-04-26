## ADDED Requirements

### Requirement: stable release readiness SHALL be score-based

安定版 readiness は、数値化された stable score と hard blocker で評価されなければならない（SHALL）。

#### Scenario: stable score を評価する

- **WHEN** developer が `v0.12.8` の安定版判定を行う
- **THEN** system は 100 点満点の score を算出する
- **THEN** system は Precision correctness、Safe command behavior、Performance stability、Release reproducibility、Evidence quality の category score を記録する
- **THEN** system は hard blocker の有無を記録する
- **THEN** score が 90 点以上、hard blocker が 0 件の場合だけ stable candidate として扱う

### Requirement: stable release SHALL require user acceptance

安定版 release は、最終的なユーザー受け入れ判断を必要としなければならない（SHALL）。

#### Scenario: ユーザー受け入れを確認する

- **WHEN** system が stable score、hard blocker、known limitation、verification result を提示する
- **THEN** user は安定版として受け入れるか判断する
- **THEN** user が受け入れた場合だけ `v0.13.0` の DoR を満たす
- **THEN** user が受け入れない場合、追加の `v0.12.x` hardening scope を定義する

### Requirement: distribution work SHALL wait for stable acceptance

配布展開は、安定版受け入れが完了するまで進めてはならない（SHALL NOT）。

#### Scenario: v0.13.0 に進む

- **WHEN** developer が `v0.13.0` の配布計画に着手する
- **THEN** system は `v0.12.8` の stable score が 90 点以上であることを確認する
- **THEN** system は hard blocker が 0 件であることを確認する
- **THEN** system は user acceptance が記録されていることを確認する
