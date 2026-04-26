# Project Rules

- `README.md` and public Markdown under `docs/` must be written in English.
- Do not add Japanese prose or Japanese status labels to user-facing documentation.
- When updating `README.md` or docs language/status tables, run `make ast-lint`
  before reporting completion.
- Use `scripts/openspec` instead of calling a bare `openspec` command from
  agent instructions or local workflow docs.
