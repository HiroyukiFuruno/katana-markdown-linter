# Tasks

## Definition Of Ready

- [ ] `safe-fix-coverage-continuous-expansion` has completed at least one batch.
- [ ] `check`, `fix`, and `fmt` responsibility split is accepted.
- [ ] Unsafe fix policy is not required for default formatter behavior.

## 1. Contract

- [ ] 1.1 Define formatter API and CLI semantics.
- [ ] 1.2 Define formatter exit code behavior.
- [ ] 1.3 Define stdin/stdout behavior for editor integration.

## 2. Implementation

- [ ] 2.1 Implement formatter policy entrypoint if current `fmt` alias is insufficient.
- [ ] 2.2 Add idempotence tests.
- [ ] 2.3 Add docs that compare `check`, `fix`, and `fmt`.

## Verification

- [ ] `cargo test --workspace --locked` passes.
- [ ] `make dogfood` passes.
- [ ] `git diff --check` passes.

## Definition Of Done

- [ ] Formatter behavior is deterministic and idempotent.
- [ ] Formatter contract is separate from lint fix contract.
- [ ] Editor-friendly stdout and exit code behavior is documented.
