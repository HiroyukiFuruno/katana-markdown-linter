## MODIFIED Requirements

### Requirement: release gates SHALL validate npm package page artifacts before publish

release gate は、npm publish 前に registry page に表示される package artifact を検証しなければならない（SHALL）。

#### Scenario: npm wrapper publish is enabled

- **WHEN** release workflow is dispatched with npm wrapper publication enabled
- **THEN** system runs `npm pack --dry-run --json` for `wrappers/npm`
- **AND** system verifies the packed file list contains `README.md`, `package.json`, `bin/kml.js`, and `lib/installer.js`
- **AND** system verifies npm package metadata includes non-empty `description`, `keywords`, `repository`, `homepage`, `bugs`, `license`, and `bin`
- **AND** system stops before publish if README or required metadata is missing

### Requirement: npm wrapper retry SHALL use trusted publishing without token fallback

npm wrapper retry は、trusted publishing で実行し、長期 token fallback に戻してはならない（SHALL NOT）。

#### Scenario: npm publish is retried after trusted publisher setup

- **WHEN** developer retries npm wrapper publication for `vX.Y.Z`
- **THEN** workflow uses GitHub Actions OIDC trusted publishing
- **AND** workflow does not require `NPM_TOKEN` or `NODE_AUTH_TOKEN`
- **AND** workflow records a clear failure when npm rejects the trusted publisher context
