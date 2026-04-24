## ADDED Requirements

### Requirement: system SHALL provide an opt-in cross-tool CLI benchmark

システムは、`kml`、`mado`、`rumdl` を同一 corpus 上の外部 CLI process として比較する opt-in benchmark を提供しなければならない（SHALL）。

#### Scenario: cross-tool benchmark を実行する

- **WHEN** developer が cross-tool benchmark command を実行する
- **THEN** system は release build の `kml` binary を benchmark 対象に含める
- **THEN** system は利用可能な `mado` binary を benchmark 対象に含める
- **THEN** system は利用可能な `rumdl` binary を benchmark 対象に含める
- **THEN** system は各 tool を同じ corpus path に対する外部 CLI process として実行する
- **THEN** system は third-party tool が未導入の場合、その tool の case を skipped として report する

#### Scenario: benchmark report を生成する

- **WHEN** cross-tool benchmark が完了する
- **THEN** system は machine-readable JSON report を生成する
- **THEN** system は human-readable Markdown summary を生成する
- **THEN** report は tool name、tool version、case name、mode、command、timing method、median milliseconds、mean milliseconds、min milliseconds、max milliseconds、standard deviation milliseconds を含む
- **THEN** report は skipped case の reason を含む

### Requirement: system SHALL separate default and common-subset comparisons

システムは、tool default behavior の比較と shared rule subset の比較を分離しなければならない（SHALL）。

#### Scenario: default comparison を実行する

- **WHEN** developer が default comparison mode を実行する
- **THEN** system は各 tool の標準的な check command と default config behavior を使う
- **THEN** system は report に default comparison であることを明記する
- **THEN** system は default comparison を純粋な rule engine speed comparison として扱わない

#### Scenario: common-subset comparison を実行する

- **WHEN** developer が common-subset comparison mode を実行する
- **THEN** system は shared markdownlint-style rule subset を使う config を生成または選択する
- **THEN** system は各 tool に対応する config format を使う
- **THEN** system は report に有効化された rule IDs を含める
- **THEN** system は unsupported rule または option を detected limitation として report する

### Requirement: system SHALL benchmark check and fix workflows safely

システムは、check と fix の workflow を同じ source corpus から安全に benchmark しなければならない（SHALL）。

#### Scenario: diagnostics-heavy check を測定する

- **WHEN** benchmark case が diagnostics-heavy corpus を check する
- **THEN** system は expected violation exit code を benchmark failure として扱わない
- **THEN** system は unexpected tool error を case failure として report する
- **THEN** system は diagnostics output を timing result と混在させない

#### Scenario: clean check を測定する

- **WHEN** benchmark case が clean corpus を check する
- **THEN** system は zero exit code を期待する
- **THEN** system は non-zero exit code を case failure として report する

#### Scenario: fix workflow を測定する

- **WHEN** benchmark case が fix command を実行する
- **THEN** system は source corpus を temporary workspace に copy してから tool を実行する
- **THEN** system は source corpus を変更しない
- **THEN** system は tool が fix workflow を提供しない場合、その case を skipped として report する

### Requirement: system SHALL keep cross-tool benchmarks out of required CI gates

システムは、cross-tool benchmark を required CI gate に含めてはならない（MUST NOT）。

#### Scenario: local quality gate を実行する

- **WHEN** developer が `make check` または required CI workflow を実行する
- **THEN** system は `mado`、`rumdl`、または `hyperfine` の導入を要求しない
- **THEN** system は cross-tool benchmark を自動実行しない

#### Scenario: optional benchmark target を実行する

- **WHEN** developer が optional cross-tool benchmark target を実行する
- **THEN** system は不足している optional binaries を明確に report する
- **THEN** system は `kml` の benchmark case を必ず実行する
