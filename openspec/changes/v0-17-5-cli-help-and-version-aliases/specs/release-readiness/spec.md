### Requirement: v0.17.5 release readiness SHALL restore CLI help entrypoints

`v0.17.5` の release readiness は、CLI help entrypoint が lint 対象探索に流れないことを release blocker として扱わなければならない（SHALL）。

#### Scenario: global help is requested

- **WHEN** developer prepares `v0.17.5`
- **THEN** system runs `kml help`
- **AND** system runs `kml --help`
- **AND** system runs `kml -h`
- **AND** each command exits with code `0`
- **AND** each command prints global usage
- **AND** system does not run Markdown file discovery for those commands

#### Scenario: command help is requested

- **WHEN** developer prepares `v0.17.5`
- **THEN** system runs `kml check --help`
- **AND** system runs `kml check -h`
- **AND** each command exits with code `0`
- **AND** each command prints command usage

### Requirement: v0.17.5 release readiness SHALL support version aliases

`v0.17.5` の release readiness は、version 表示の短縮 alias を release blocker として扱わなければならない（SHALL）。

#### Scenario: version alias is requested

- **WHEN** developer prepares `v0.17.5`
- **THEN** system runs `kml version`
- **AND** system runs `kml --version`
- **AND** system runs `kml -V`
- **AND** system runs `kml -v`
- **AND** each command exits with code `0`
- **AND** each command prints `0.17.5`

### Requirement: v0.17.5 release readiness SHALL update Homebrew tap

`v0.17.5` の release readiness は、Homebrew tap が release artifact と同じ version を指すことを release blocker として扱わなければならない（SHALL）。

#### Scenario: release workflow updates Homebrew tap

- **WHEN** developer publishes `v0.17.5`
- **THEN** release workflow uses `HOMEBREW_KATANA_GIT_TOKEN`
- **AND** release workflow updates `Formula/kml.rb` in `HiroyukiFuruno/homebrew-katana`
- **AND** release workflow adds `Formula/kml@0.17.5.rb` in `HiroyukiFuruno/homebrew-katana`
- **AND** versioned formula `Formula/kml@0.17.5.rb` is `keg_only :versioned_formula`
- **AND** release workflow does not fall back to `github.token` for the tap update

#### Scenario: post-release verification checks actual tap content

- **WHEN** developer runs `make release-verify VERSION=v0.17.5`
- **THEN** system renders the expected Homebrew formula from GitHub Release assets
- **AND** system reads `Formula/kml.rb` from `HiroyukiFuruno/homebrew-katana`
- **AND** system reads `Formula/kml@0.17.5.rb` from `HiroyukiFuruno/homebrew-katana`
- **AND** verification fails if either tap file differs from the generated formula

### Requirement: v0.17.5 release readiness SHALL backfill Homebrew versioned formulae

`v0.17.5` の release readiness は、npm / PyPI に合わせて `v0.17.1` 以降の公開済み Homebrew formula を登録しなければならない（SHALL）。

#### Scenario: historical Homebrew formulae are backfilled

- **WHEN** developer prepares `v0.17.5`
- **THEN** `homebrew-katana` contains `Formula/kml@0.17.1.rb`
- **AND** `homebrew-katana` contains `Formula/kml@0.17.3.rb`
- **AND** `homebrew-katana` contains `Formula/kml@0.17.4.rb`
- **AND** each versioned formula is `keg_only :versioned_formula`
- **AND** `homebrew-katana` does not add `Formula/kml@0.17.2.rb`
