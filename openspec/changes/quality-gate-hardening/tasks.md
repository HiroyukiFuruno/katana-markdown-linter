## Definition of Ready

- [ ] `rule-fixture-parity-matrix` のmatrix schemaが存在していること
- [ ] `rule-check-fix-completion` のfixture harnessが存在していること
- [ ] `cli-parity-upgrade` のCLI contractが定義済みであること
- [ ] KatanAのlint / ast-lint運用を参考にし、このrepo固有のgateだけを採用すること
- [ ] required checkを変更する場合はbranch protectionも同時に更新すること

## 1. Makefile Gate Contract

- [ ] 1.1 `make lint` の責務をClippy zero warningとして固定する
- [ ] 1.2 `make ast-lint` の責務をrepository固有不変条件として固定する
- [ ] 1.3 `make check` に含めるgateを明示する
- [ ] 1.4 `make coverage` をreport mode / blocking modeに分ける

## 2. AST Lint Expansion

- [ ] 2.1 fixture matrix coverage gateを追加する
- [ ] 2.2 upstream unknown drift gateを追加する
- [ ] 2.3 release signed tag workflow gateを維持する
- [ ] 2.4 CLI traversal / gitignore behavior gateを維持する
- [ ] 2.5 public API / rule catalog破壊検出gateを追加する

## 3. CI and Branch Protection

- [ ] 3.1 GitHub Actions required checksとMakefile targetの対応を文書化する
- [ ] 3.2 required checksを変更する場合はbranch protectionを更新する
- [ ] 3.3 PRとdirect pushの挙動をrunbookに記録する

## 4. Release Readiness

- [ ] 4.1 release前に実行するgateをMakefileに集約する
- [ ] 4.2 release workflowとlocal release gateの差分を文書化する
- [ ] 4.3 failed gateの復旧手順をrunbookに追加する

## Definition of Done

- [ ] `make lint`, `make ast-lint`, `make check`, `make release-check` の責務が明確であること
- [ ] AST lintがfixture/upstream/CLI/releaseの主要regressionを検出できること
- [ ] CI required checksとbranch protectionが文書化されていること
