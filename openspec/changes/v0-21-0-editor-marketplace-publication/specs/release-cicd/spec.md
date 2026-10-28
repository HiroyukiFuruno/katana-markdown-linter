## MODIFIED Requirements

### Requirement: release workflow SHALL run quality gates before release creation

システムは、GitHub Release 作成前に品質ゲートを実行しなければならない（SHALL）。

#### Scenario: editor marketplace release gate を実行する

- **WHEN** release workflow が editor marketplace publish を有効にして実行される。
- **THEN** system は通常の release gate に加えて editor capability evidence と final editor dogfood evidence を確認する。
- **AND** system は `just editor-extension-check` と `just editor-publish-gate` を実行する。
- **AND** evidence または gate が不足する場合、GitHub Release 作成または marketplace publish を実行しない。

### Requirement: local and CI release gates SHALL remain aligned

local と CI の release gate は、意図しない乖離を起こしてはならない（SHALL NOT）。

#### Scenario: editor publication gate を変更する

- **WHEN** developer が editor marketplace publication の workflow step を変更する。
- **THEN** system は対応する Justfile recipe、release script、AST lint、または release verification を更新する。
- **AND** local `just release-check` と CI release workflow が同じ editor publication 前提を検証する。
