## Design

### Responsibility Split

- `check`: report diagnostics
- `fix`: apply safe lint-driven fixes
- `fmt`: apply deterministic formatting policy

`fmt` may use lint fixes internally, but it must be documented as a formatter
contract and tested for idempotence.

### Exit Code Policy

Formatter exit codes should be editor-friendly:

- success after formatting should exit `0`
- filesystem/config errors should exit `2`
- unresolved diagnostics should be reported according to the final policy, not
  inherited accidentally from `check --fix`

### Non-Goals

- Implementing formatter behavior before linter precision work is complete.
- Treating unsafe rewrites as formatter defaults.
