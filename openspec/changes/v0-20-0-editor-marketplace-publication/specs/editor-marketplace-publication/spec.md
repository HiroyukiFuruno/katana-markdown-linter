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

### Requirement: Zed marketplace publication SHALL require merged upstream registry PR

Zed extension registry 公開は、`zed-industries/extensions` の merged PR URL が確認できる場合にのみ完了扱いにすること（SHALL）。

#### Scenario: Zed publish is enabled with unmerged PR

- **WHEN** `publish_zed_extension` が有効である。
- **AND** `ZED_EXTENSION_PUBLICATION_PR_URL` が未指定、無効、または未merge PR を指している。
- **THEN** system は Zed publication verification を失敗させる。
- **AND** system は release completion を止める。

### Requirement: marketplace publication SHALL be explicit

editor marketplace publication は、明示 input または対応する local env が有効な場合だけ実行すること（SHALL）。

#### Scenario: core release runs without editor publish flags

- **WHEN** core release が editor publish flags なしで実行される。
- **THEN** system は VS Code / Zed publish を実行しない。
- **AND** release verification は editor publication を `deferred` として説明する。
