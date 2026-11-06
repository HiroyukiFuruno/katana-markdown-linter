## Context

PR #142 moved shared parsing and edit logic behind their existing boundaries.
The migration is included in the patch release with the same public Markdown
and fix contracts.

## Goals / Non-Goals

- Preserve diagnostics, fix ordering, output, and release quality thresholds.
- Verify each enabled distribution from the same signed source.
- Keep editor Marketplace publication in the existing v0.21.0 change.

## Decisions

- Use the unchanged KAL runner for both repository checks and isolated
  boundary regressions.
- Synchronize package, wrapper, MCP, and editor metadata in a release PR
  based on the merged implementation. Let that merge trigger publication.

## Risks / Trade-offs

- A successful workflow can precede registry propagation. Verify public
  metadata, checksums, and install or launch behavior after publication.
