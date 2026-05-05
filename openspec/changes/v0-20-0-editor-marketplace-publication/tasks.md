# Tasks

## 目標

- `v0-19-0-editor-capability-completion` 完了後に、VS Code / Zed Marketplace 公開だけを扱う。
- 公開前提、公開実行、公開後 verification を同じ release evidence に紐づける。
- editor 機能不足や dogfood 未完了を、この change 内で隠して公開しない。

## 0. Definition of Ready

- [ ] 0.1 `v0-19-0-editor-capability-completion` が完了していることを確認する
- [ ] 0.2 final editor dogfood evidence が diagnostics / formatting / safe fixes / config changes を含むことを確認する
- [ ] 0.3 release-blocking finding が 0 件であることを確認する
- [ ] 0.4 VS Code publisher / package name / token secret を確認する
- [ ] 0.5 Zed upstream registry PR の作成・merge 手順を確認する

## 1. Release gate

- [ ] 1.1 `release-target-check` が VS Code Marketplace の既存 version を検出することを固定する
- [ ] 1.2 `release-target-check` が Zed extension registry の既存 version を検出することを固定する
- [ ] 1.3 editor capability evidence がない場合に publish gate が fail-fast するようにする
- [ ] 1.4 `just release-check` と release workflow の editor publication gate を一致させる

## 2. VS Code publication

- [ ] 2.1 `VSCE_PAT` secret の存在確認を release workflow に入れる
- [ ] 2.2 `vsce` publish command を official path で実行する
- [ ] 2.3 VS Code Marketplace で target version が公開されたことを verify する
- [ ] 2.4 token 不備・publisher 不備・package metadata 不備の failure message を runbook に反映する

## 3. Zed publication

- [ ] 3.1 `zed-industries/extensions` への PR 手順を runbook に固定する
- [ ] 3.2 `ZED_EXTENSION_PUBLICATION_PR_URL` の URL format と repo を検証する
- [ ] 3.3 PR が merged であることを `gh pr view` で確認する
- [ ] 3.4 Zed registry 側の published state を release verification に記録する

## 4. Release verification

- [ ] 4.1 `release-verify` が editor publication flags に応じて `published` / `deferred` / `failed` を記録する
- [ ] 4.2 VS Code / Zed の片方だけ公開された partial state を失敗として扱う
- [ ] 4.3 evidence に target version、registry URL、verification command、確認日時を残す
- [ ] 4.4 公開失敗時の retry / next version 判断を runbook に記録する

## 5. 検証

- [ ] 5.1 `just release-check`
- [ ] 5.2 `PUBLISH_VSCODE_EXTENSION=true PUBLISH_ZED_EXTENSION=true ZED_EXTENSION_PUBLICATION_PR_URL=<url> just VERSION=v0.20.0 release-verify`
- [ ] 5.3 `just ast-lint`
- [ ] 5.4 `scripts/openspec validate v0-20-0-editor-marketplace-publication --strict`

## Definition of Done

- [ ] D1 editor capability evidence と final dogfood evidence が公開前提として検証されている
- [ ] D2 VS Code Marketplace と Zed extension registry の公開状態が release verification で確認できる
- [ ] D3 partial publish が成功扱いにならない
- [ ] D4 公開手順と失敗時対応が runbook と release evidence に残っている
