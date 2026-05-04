# Requirement: v0.18.4 release gate state determinism

### Requirement: release-verify SHALL output editor artifact state

`release-verify` は、対象 editor artifact について `published` または `deferred` を明示し、`v0.18.4` 判定時に参照できる形で残す。

### Requirement: release-check SHALL consume the same state definition

#### Scenario: release check follows verify state

- **WHEN** `release-verify` が `deferred` を出す。
- **THEN** `release-check` はその状態に矛盾しない判定をとり、同時に `release` を進行しない。

### Requirement: publish trace SHALL be reviewable

- **WHEN** release 判定時に editor artifact の状態を再確認する。
- **THEN** `release-runbook` と `release-verify` に同じ state 解釈が残っていること。
