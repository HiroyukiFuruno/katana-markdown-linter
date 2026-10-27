## Context

The published release and clean master checkout start at v0.19.5. The next patch is v0.19.6. Existing release recipes provide the validation and publication contract.

## Goals / Non-Goals

Refresh GitHub Actions and Rust dependencies without changing public linting behavior. Do not publish editor marketplaces, change compatibility gates, edit sibling repositories, or create worktrees.

## Decisions

Use upstream stable action releases and immutable commit revisions where appropriate. Preserve install-action tool selection inputs. Upgrade rmcp to 3.8.0 and zed_extension_api to 0.7.0 (the current extension API; the extension remains unpublished in marketplaces) and refresh both Cargo lockfiles; evaluate any remaining older transitive dependency before deciding whether it can be upgraded.

## Risks / Trade-offs

Dependency changes can affect MCP transport and code generation. The full existing release-check, coverage threshold, MCP smoke tests, packaging checks, and PR CI remain mandatory.

## Migration Plan

Validate locally, create a signed Draft PR, collect Codex and self review, address findings, promote to Ready, pass required checks, merge, run the canonical release recipe and release-verify, then clean the merged local release branch. Remote branches are retained unless separately authorized.
