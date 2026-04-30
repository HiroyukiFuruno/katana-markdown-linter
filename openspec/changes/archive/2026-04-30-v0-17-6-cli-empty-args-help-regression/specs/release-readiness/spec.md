## ADDED Requirements

### Requirement: v0.17.6 release readiness SHALL treat empty CLI arguments as help

`v0.17.6` の release readiness は、引数なし `kml` が lint 対象探索へ流れないことを release blocker として扱わなければならない（SHALL）。

#### Scenario: empty CLI args are requested

- **WHEN** developer prepares `v0.17.6`
- **THEN** system runs `kml` without arguments in an empty working directory
- **AND** command exits with code `0`
- **AND** command prints global usage
- **AND** command does not run Markdown file discovery

#### Scenario: existing help and version aliases keep working

- **WHEN** developer prepares `v0.17.6`
- **THEN** system runs `kml help`
- **AND** system runs `kml --help`
- **AND** system runs `kml -h`
- **AND** system runs `kml version`
- **AND** system runs `kml --version`
- **AND** system runs `kml -V`
- **AND** system runs `kml -v`
- **AND** each command exits with code `0`

#### Scenario: Japanese help is requested

- **WHEN** developer prepares `v0.17.6`
- **THEN** system runs `kml --locale ja help`
- **AND** system runs `kml check --help --locale ja`
- **AND** each command exits with code `0`
- **AND** each command prints Japanese usage text
- **AND** each help text explains that `--locale` changes diagnostic and help text language

### Requirement: v0.17.6 release readiness SHALL accept official markdownlint config aliases

`v0.17.6` の release readiness は、公式 markdownlint alias と `integer|integer[]` 型の設定値を config error にしないことを release blocker として扱わなければならない（SHALL）。

#### Scenario: repository-style config uses official aliases

- **WHEN** developer prepares `v0.17.6`
- **THEN** system runs `kml check` with a config containing `first-line-h1`, `first-line-heading`, `no-duplicate-heading`, and `no-inline-html`
- **AND** the config contains `MD022.lines_above` and `MD022.lines_below` as integer or integer array values
- **AND** command does not report `unknown markdownlint rule`
- **AND** command does not report `invalid rule property value`

### Requirement: v0.17.6 release readiness SHALL stop before linting when config is invalid

`v0.17.6` の release readiness は、config error を lint 診断と混ぜず、既定で lint 実行前に停止することを release blocker として扱わなければならない（SHALL）。

#### Scenario: invalid config is used without override

- **WHEN** developer runs `kml check` with invalid config
- **THEN** command exits with code `2`
- **AND** command reports the config error
- **AND** command advises fixing the config or rerunning with `--ignore-config-errors`
- **AND** command does not report file lint diagnostics

#### Scenario: invalid config is explicitly ignored

- **WHEN** developer runs `kml check --ignore-config-errors` with invalid config
- **THEN** command reports the config error
- **AND** command ignores invalid config entries
- **AND** command continues to report file lint diagnostics
