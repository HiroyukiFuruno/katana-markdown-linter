## Why

GitHub Actions and Rust dependencies have newer upstream releases. Refresh them while preserving existing behavior and publish patch v0.19.6.

## What Changes

- Update workflow actions to verified upstream revisions.
- Update direct and transitive Rust dependencies in the root and Zed packages.
- Synchronize all release metadata and installation examples to v0.19.6.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `release-cicd`: record verified dependency refresh evidence for v0.19.6 while preserving existing release gates.

## Impact

Workflow configuration, Cargo manifests and lockfiles, package version metadata, release notes, and installation examples. No sibling repository edits or new worktrees.
