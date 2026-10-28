## ADDED Requirements

### Requirement: marketplace publication SHALL require completed editor capability evidence

editor marketplace publication は、`v0-19-0-editor-capability-completion` の完了 evidence がない状態で実行してはならない（MUST NOT）。

#### Scenario: editor capability evidence is missing

- **WHEN** release workflow または local release command が editor marketplace publish を有効にする。
- **AND** final editor dogfood evidence が存在しない。
- **THEN** system は publish を停止する。
- **AND** system は不足している evidence をエラーに含める。

### Requirement: VS Code marketplace publication SHALL require VSCE_PAT

VS Code Marketplace 公開は、repository secret `VSCE_PAT` が存在する場合にのみ実行すること（SHALL）。

#### Scenario: VS Code publish is enabled without token

- **WHEN** `publish_vscode_extension` が有効である。
- **AND** `VSCE_PAT` が存在しない。
- **THEN** system は VS Code publish job を実行前に失敗させる。
- **AND** system は token 名を含む明確な修正手順を表示する。

#### Scenario: VS Code publish attempts to use GitHub OIDC

- **WHEN** VS Code Marketplace publish が有効である。
- **AND** system が `ACTIONS_ID_TOKEN_REQUEST_TOKEN` または `ACTIONS_ID_TOKEN_REQUEST_URL` を Marketplace credential として扱おうとする。
- **THEN** system はその実装を拒否する。
- **AND** system は `VSCE_PAT` と `vsce` publish path を要求する。

### Requirement: Zed marketplace publication SHALL require merged upstream registry PR

Zed extension registry 公開は、`zed-industries/extensions` の merged PR URL が確認できる場合にのみ完了扱いにすること（SHALL）。

#### Scenario: Zed publish is enabled with unmerged PR

- **WHEN** `publish_zed_extension` が有効である。
- **AND** `ZED_EXTENSION_PUBLICATION_PR_URL` が未指定、無効、または未merge PR を指している。
- **THEN** system は Zed publication verification を失敗させる。
- **AND** system は release completion を止める。

#### Scenario: Zed publish attempts to require a repository secret

- **WHEN** Zed extension publication が有効である。
- **AND** system が `ZED_EXTENSION_TOKEN` などの独自 repository secret を要求する。
- **THEN** system はその secret 追加を採用しない。
- **AND** system は `ZED_EXTENSION_PUBLICATION_PR_URL` の merged PR verification を公開完了条件にする。

### Requirement: marketplace publication SHALL be explicit

editor marketplace publication は、明示 input または対応する local env が有効な場合だけ実行すること（SHALL）。

#### Scenario: core release runs without editor publish flags

- **WHEN** core release が editor publish flags なしで実行される。
- **THEN** system は VS Code / Zed publish を実行しない。
- **AND** release verification は editor publication を `deferred` として説明する。

### Requirement: editor publication variables SHALL be fixed

editor publication に使う workflow input / env / secret は、OpenSpec design の CI/CD variable contract に定義された名前だけを使うこと（SHALL）。

#### Scenario: implementation needs an additional publication variable

- **WHEN** 実装中に追加の workflow input、environment variable、または repository secret が必要に見える。
- **THEN** developer は自己判断で追加しない。
- **AND** developer は必要な理由、代替案、既存 contract で足りない点を報告する。
