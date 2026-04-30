## MODIFIED Requirements

### Requirement: npm wrapper publication SHALL use trusted publishing

npm wrapper publication は、通常 release path で repository secret token に依存してはならない（SHALL NOT）。

#### Scenario: npm wrapper publish job runs

- **WHEN** release workflow publishes the npm wrapper
- **THEN** job uses GitHub Actions OIDC trusted publishing
- **AND** job does not require `NPM_TOKEN` or `NODE_AUTH_TOKEN`
- **AND** job fails before publish when trusted publishing is not available

### Requirement: PyPI wrapper publication SHALL keep environment-scoped OIDC

PyPI wrapper publication は、`pypi` environment と trusted publisher の組み合わせで実行されなければならない（SHALL）。

#### Scenario: PyPI wrapper publish job runs

- **WHEN** release workflow publishes the PyPI wrapper
- **THEN** job runs under the `pypi` GitHub environment
- **AND** job uses `pypa/gh-action-pypi-publish`
- **AND** job does not require username, password, or API token secrets

### Requirement: wrapper publish gate SHALL match workflow behavior

wrapper publish gate は、workflow の実際の publish 条件と同じ前提を検証しなければならない（SHALL）。

#### Scenario: local release check evaluates wrapper publication

- **WHEN** developer runs the release gate with wrapper publication enabled
- **THEN** system reports that trusted publishing is required
- **AND** local gate does not pretend to publish registry packages
- **AND** workflow-only behavior is documented as a post-merge release action
