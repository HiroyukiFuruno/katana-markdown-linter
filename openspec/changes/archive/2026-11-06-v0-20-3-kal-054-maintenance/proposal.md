## Why

KAL 0.5.4 detects source and function length violations that the previous
version missed. Issue #139 delivers the migration and keeps the existing
quality thresholds. The patch release makes those changes available through
the regular distribution channels.

## What Changes

- Release the KAL 0.5.4 migration, shared rule and fix boundaries, and the
  Node.js 26 npm publishing configuration from PR #142.
- Preserve the release gate and verify versioned distribution artifacts.

## Capabilities

### Modified Capabilities

- `release-readiness`: Protect source and function length checks with
  regressions through the standard repository gate.

## Impact

Development tooling, shared Markdown internals, release automation, and
distribution metadata for v0.20.3.
