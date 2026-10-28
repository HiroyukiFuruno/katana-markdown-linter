## ADDED Requirements

### Requirement: release target check SHALL reject already published editor versions

release target check は、VS Code Marketplace または Zed extension registry に target version が既に存在する場合、release を停止すること（SHALL）。

#### Scenario: VS Code version already exists

- **WHEN** target version が VS Code Marketplace に既に存在する。
- **THEN** `release-target-check` は失敗する。
- **AND** system は同じ version の再 publish を試みない。

#### Scenario: Zed version already exists

- **WHEN** target version が Zed extension registry に既に存在する。
- **THEN** `release-target-check` は失敗する。
- **AND** system は同じ version の再 publish を試みない。

### Requirement: release verification SHALL prove published editor marketplace state

release verification は、publish flag が有効な editor marketplace について target version の published state を確認すること（SHALL）。

#### Scenario: both editor publications are enabled

- **WHEN** `PUBLISH_VSCODE_EXTENSION=true` と `PUBLISH_ZED_EXTENSION=true` で `release-verify` を実行する。
- **THEN** system は VS Code Marketplace で target version を確認する。
- **AND** system は Zed extension registry で target version または merged registry PR を確認する。
- **AND** どちらか一方でも確認できない場合は release verification を失敗させる。

### Requirement: release evidence SHALL record marketplace publication outcome

release evidence は、VS Code / Zed それぞれの `published` / `deferred` / `failed` を記録すること（SHALL）。

#### Scenario: publication is partially complete

- **WHEN** VS Code または Zed の一方だけが published になっている。
- **THEN** system は partial state を evidence に記録する。
- **AND** system は完了扱いにしない。
- **AND** retry または next version の判断材料を残す。
