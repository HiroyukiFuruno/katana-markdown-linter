# Tasks

## Definition Of Ready

- [ ] Release workflow is stable for the current crate.
- [ ] CLI contracts for check/fix/fmt are stable enough to document externally.
- [ ] Maintenance cost for each channel is understood.

## 1. Channel Evaluation

- [ ] 1.1 Evaluate GitHub Action wrapper.
- [ ] 1.2 Evaluate pre-commit integration.
- [ ] 1.3 Evaluate Homebrew distribution.
- [ ] 1.4 Evaluate npm and pip/uv wrappers.
- [ ] 1.5 Evaluate config schema publication.

## 2. Implementation

- [ ] 2.1 Implement the first selected official channel.
- [ ] 2.2 Add release verification for that channel.
- [ ] 2.3 Document installation and update policy.

## Verification

- [ ] Release preflight passes.
- [ ] New channel smoke test passes.
- [ ] `git diff --check` passes.

## Definition Of Done

- [ ] At least one non-Cargo integration channel is official and verified.
- [ ] Unsupported channels are explicitly deferred with reasons.
- [ ] Core crate remains independent from wrappers.
