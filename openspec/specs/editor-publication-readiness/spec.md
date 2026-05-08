# editor-publication-readiness Specification

## Purpose
TBD - created by archiving change assess-editor-publication-readiness. Update Purpose after archive.
## Requirements
### Requirement: triage change SHALL not carry a release version

- この change は、versioned release 実装ではなく、editor publication readiness の調査・task 化 change として扱うこと（SHALL）。
- この change は、`v0.19.0` 系の editor 機能実装または `v0.20.0` Marketplace 公開を直接実装してはならない（MUST NOT）。

#### Scenario: discussion creates future versioned work

- **WHEN** 調査中に `v0.19.0` 系または `v0.20.0` の作業が見つかる。
- **THEN** system はこの change 内で実装済み扱いにしない。
- **AND** system は後続 OpenSpec change 作成タスクとして記録する。

### Requirement: readiness triage SHALL block marketplace publication while editor capability is incomplete

- VS Code / Zed が設定反映済みの診断、整形、安全な修正を提供できない場合、Marketplace 公開は No-Go とすること（SHALL）。
- `kml lsp` を起動できるだけの状態を公開可能な editor integration と見なしてはならない（MUST NOT）。

#### Scenario: current extension uses default lint options

- **WHEN** LSP の診断・修正が project config ではなく `LintOptions::default()` に依存している。
- **THEN** system は Marketplace 公開を保留する。
- **AND** system は config 反映、editor fix、最終 dogfood を後続タスクへ分離する。

### Requirement: follow-up work SHALL be split into explicit OpenSpec changes

- `v0.19.0` 系の editor 機能開発は、別 OpenSpec change として作成すること（SHALL）。
- `v0.19.0` 系の最終 editor dogfood は、別 OpenSpec change または明確な独立タスクとして扱うこと（SHALL）。
- `v0.20.0` Marketplace 公開は、editor 機能と dogfood が完了した後の別 OpenSpec change として作成すること（SHALL）。

#### Scenario: marketplace work is requested before editor readiness is complete

- **WHEN** Marketplace 公開の作業が、editor 機能開発や最終 dogfood より先に進められようとする。
- **THEN** system は公開作業を停止する。
- **AND** system は不足している前段 change を示す。

### Requirement: v0.18.7 skip policy SHALL remain explicit

- `v0.18.7` は再公開不可の事故版として永久欠番にすること（SHALL）。
- `v0.18.8` へ飛ばして継続してはならない（MUST NOT）。

#### Scenario: release target tries to continue v0.18.x after v0.18.7

- **WHEN** release target が `v0.18.7` または `v0.18.8` を指定する。
- **THEN** system はその release target を拒否する。
- **AND** system は `v0.18.7` が永久欠番であることを説明する。

