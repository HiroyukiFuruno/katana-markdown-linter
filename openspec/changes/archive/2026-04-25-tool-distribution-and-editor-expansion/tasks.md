# Tasks

## Definition Of Ready

- [x] Release workflow is stable for the current crate.
- [x] CLI contracts for check/fix/fmt are stable enough to document externally.
- [x] Maintenance cost for each channel is understood.

## 1. Channel Evaluation

- [x] 1.1 Evaluate GitHub Action wrapper.
- [x] 1.2 Evaluate pre-commit integration.
- [x] 1.3 Evaluate Homebrew distribution.
- [x] 1.4 Evaluate npm and pip/uv wrappers.
- [x] 1.5 Evaluate config schema publication.

## 2. Implementation

- [x] 2.1 Implement the first selected official channel.
- [x] 2.2 Add release verification for that channel.
- [x] 2.3 Document installation and update policy.

## Verification

- [x] Release preflight passes.
- [x] New channel smoke test passes.
- [x] `git diff --check` passes.

## Definition Of Done

- [x] At least one non-Cargo integration channel is official and verified.
- [x] Unsupported channels are explicitly deferred with reasons.
- [x] Core crate remains independent from wrappers.
