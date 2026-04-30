# release-readiness Specification

## ADDED Requirements

### Requirement: Release readiness SHALL include document answer fix evaluation

Release readiness SHALL include document-level answer fixture evaluation before publishing `v0.16.2`.

#### Scenario: release check runs document answer evaluation

- **WHEN** developer runs `make release-check VERSION=v0.16.2`
- **THEN** system runs the document answer fix evaluation
- **AND** release check fails if any fixed output differs from its answer fixture
- **AND** release check fails if the public corpus has fewer than 200 valid public GitHub samples
- **AND** release check fails if the original corpus has fewer than 50 valid original samples

### Requirement: Release readiness SHALL limit v0.16.2 scope to bugfixes

Release readiness SHALL keep `v0.16.2` focused on bugfixes found by document answer evaluation.

#### Scenario: scope is checked

- **WHEN** `v0.16.2` release preparation is reviewed
- **THEN** system records detected document answer mismatches and their fixes
- **AND** system does not include `v0.17.0` distribution expansion work in the same release
- **AND** system keeps Homebrew, standalone binary expansion, npm wrapper, and PyPI wrapper work deferred to `v0.17.0`
