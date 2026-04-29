## ADDED Requirements

### Requirement: v0.12.21 closeout SHALL block v0.13.0 when release-blocking feedback remains

`v0.12.21` closeout は、release-blocking feedback が残る場合に `v0.13.0` へ進んではならない（SHALL NOT）。

#### Scenario: closeout 判定を行う

- **WHEN** developer が `v0.12.21` の release 前確認を行う
- **THEN** system は KatanA feedback sweep の release-blocking issue が 0 件であることを確認する
- **THEN** system は by-design 宣言対象が README と fixture matrix に反映されていることを確認する
- **THEN** system は未分類の high-risk finding を残さない

### Requirement: KatanA false positives and bad fixes SHALL block release until fixed

KatanA feedback sweep で見つかった `check` の誤検知と `fix` の誤修正は、release 前に kml 側で修正されなければならない（SHALL）。

#### Scenario: precision blocker を扱う

- **WHEN** KatanA feedback sweep で false-positive または bad-fix が見つかる
- **THEN** system は該当 pattern を kml repository の regression test に落とす
- **THEN** system は production code を修正し、test 都合だけの挙動変更をしない
- **THEN** system は該当 pattern が再発しないことを確認するまで release readiness を満たさない

### Requirement: v0.12.21 SHALL record follow-up issues separately from release blockers

`v0.12.21` は、後続対応でよい issue と release blocker を混同してはならない（SHALL NOT）。

#### Scenario: follow-up を記録する

- **WHEN** KatanA feedback sweep で non-blocking finding が見つかる
- **THEN** system は後続版で扱う理由を記録する
- **THEN** system は `v0.13.0` の配布計画に影響するものと影響しないものを分ける
- **THEN** system は follow-up を by-design 宣言と混同しない
