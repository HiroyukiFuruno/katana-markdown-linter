## ADDED Requirements

### Requirement: CI SHALL verify supported operating systems before release

システムは、release 前の CI で supported operating systems を検証しなければならない（SHALL）。

#### Scenario: Windows を含む CI matrix を実行する

- **WHEN** pull request または main push が CI 対象ファイルを変更する
- **THEN** system は Ubuntu、macOS、Windows の job を実行する
- **THEN** system は Windows 上で Rust workspace の build と test を検証する
- **THEN** system は Windows 固有の path、shell、binary suffix 差分で失敗した場合に release を止める

### Requirement: release workflow SHALL keep publish execution single-runner

システムは、crates.io publish を複数 OS job から実行してはならない（SHALL NOT）。

#### Scenario: release workflow を実行する

- **WHEN** release workflow が GitHub Release または crates.io publish を実行する
- **THEN** system は publish step を単一 runner で実行する
- **THEN** system は OS compatibility verification を release workflow の publish job ではなく、release 前 CI / preflight の責務として扱う

### Requirement: CI cache strategy SHALL be explicit and observable

システムは、CI cache strategy を明示し、cache hit/miss と安全境界を確認できなければならない（SHALL）。

#### Scenario: cache を利用する

- **WHEN** CI、release-preflight、release workflow が Rust build を実行する
- **THEN** system は cache key が OS、lockfile、toolchain、feature set の差分を安全に扱うことを保証する
- **THEN** system は stale artifact によって test が誤って通る状態を避ける
- **THEN** system は workflow log から cache hit/miss を確認できる

### Requirement: local and CI release gates SHALL remain aligned

local と CI の release gate は、意図しない乖離を起こしてはならない（SHALL NOT）。

#### Scenario: release gate を変更する

- **WHEN** developer が workflow の release-critical step を変更する
- **THEN** system は対応する Makefile target または AST lint の更新を要求する
- **THEN** system は local `make release-check` と CI release gate の差分を可視化する
