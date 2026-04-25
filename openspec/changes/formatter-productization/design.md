## Design

### Responsibility Split

- `check`: report diagnostics
- `fix`: apply safe lint-driven fixes
- `fmt`: apply deterministic layout formatting policy

`fmt` may use lint fixes internally, but it must use a fixed formatter rule
subset and must be documented as a formatter contract. It must be tested for
idempotence.

### Formatter Scope

The `v0.10.0` formatter is a layout formatter. It SHALL normalize:

- CRLF and CR line endings to LF
- missing final newline
- repeated blank lines outside code fences
- missing blank lines around ATX headings, fenced code blocks, lists, and tables
- safe list indentation and list-marker spacing when the existing lint fix logic can prove the rewrite

The formatter SHALL NOT perform:

- paragraph reflow or hard wrapping
- heading style conversion
- emphasis or strong marker conversion
- URL/reference style conversion
- table pipe/alignment style conversion
- trailing-space removal

### Exit Code Policy

Formatter exit codes should be editor-friendly:

- success after formatting should exit `0`
- filesystem/config errors should exit `2`
- unresolved non-formatting lint diagnostics must not make `fmt` exit `1`
- `fmt --stdin` writes formatted Markdown to stdout and returns `0` on success

### API Policy

The library SHALL expose a formatter entrypoint separate from `fix` so embedders
can format content without running a CLI command.

The first formatter API SHOULD expose:

- input content
- formatter options with a default policy
- formatted content
- applied operation count

### Non-Goals

- Treating unsafe rewrites as formatter defaults.
- Making the formatter a full Prettier-compatible Markdown formatter.
- Making formatter behavior depend on every enabled lint rule.
