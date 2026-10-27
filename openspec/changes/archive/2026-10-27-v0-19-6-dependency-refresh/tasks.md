# v0.19.6 dependency refresh

## 1. Definition of Ready

- [x] 1.1 User authorized dependency updates, patch version increment, and publication.
- [x] 1.2 v0.19.5 is the current published release; master started clean with zero stashes.

## 2. Implementation and verification

- [x] 2.1 Verify latest GitHub Actions revisions and apply compatible updates.
- [x] 2.2 Update root and Zed Rust dependencies and assess remaining outdated packages.
- [x] 2.3 Synchronize release metadata and examples to v0.19.6.
- [x] 2.4 Pass full `just VERSION=v0.19.6 release-check` and diff validation.
- [x] 2.5 Archive implementation artifacts after verification.
- [x] 2.6 Run `release-task-ledger-check` after recording verified quality evidence.

## Release DoD

Draft PR, Codex review and self review, findings fixed with reply/resolve, zero open P0/P1, Ready, required checks, merge, canonical release, release-verify, and branch hygiene are tracked in the session hook ledger and publication evidence. These remain mandatory after implementation archive.

## 品質評価スコア

| 項目 | 最大 | 現在 |
| --- | --- | --- |
| 依存関係の確認 | 20 | 20 |
| ローカル品質検査 | 60 | 60 |
| 差分とメタデータ整合性 | 20 | 20 |
| 合計 | 100 | 100 |

## Verified evidence

- GitHub API verified upstream action tags and commits on 2026-10-27.
- Root and Zed `cargo outdated` both report all dependencies up to date.
- `just zed-extension-check` passed with zed_extension_api 0.7.0.
- Workflow guard expectations retain exact SHA checks and were synchronized to the new revisions; all 25 ast-lint tests passed.
- Independent diff review found one P2 documentation synchronization issue, fixed in the three affected public docs; no P0/P1 findings.
- Full release-check log: `/tmp/kml-v0.19.6-release-check.log` (PASS, exit 0; coverage 802 <= 881; MCP stdio/remote, packaging, wrappers, editors, cargo publish dry-run and installation all passed).
