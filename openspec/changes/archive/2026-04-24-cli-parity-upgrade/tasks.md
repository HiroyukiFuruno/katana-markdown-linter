## Definition of Ready

- [x] phase4 のCLI tasksが完了していること
- [x] `rule-check-fix-completion` のcheck/fix behaviorが安定していること
- [x] rumdl / mado はUX参考であり完全互換を目標にしないこと
- [x] existing `check` / `fix` / `init-config` の互換を維持すること
- [x] stdout/stderr/exit code contractを変更する場合はREADMEとtestsを同時更新すること

## 1. Command Contract

- [x] 1.1 `check --fix` を追加する
- [x] 1.2 `fmt` を追加する
- [x] 1.3 `rule` と `rule MD013` を追加する
- [x] 1.4 `config file` / `config get` / `config --output json` を追加する
- [x] 1.5 `version` を追加する

## 2. Input and Filtering

- [x] 2.1 `--stdin` を追加する
- [x] 2.2 `--include` を追加する
- [x] 2.3 `--exclude` を追加する
- [x] 2.4 `--respect-gitignore` / `--no-ignore` の挙動を固定する
- [x] 2.5 `--force-exclude` を追加する

## 3. Output and Reporting

- [x] 3.1 `--output json` を追加し、`--format json` をaliasにする
- [x] 3.2 `--statistics` を追加する
- [x] 3.3 `--quiet` を追加する
- [x] 3.4 `--verbose` を追加する
- [x] 3.5 `--diff` を追加する

## 4. Tests and Docs

- [x] 4.1 CLI parse testsを追加する
- [x] 4.2 stdout/stderr snapshot testsを追加する
- [x] 4.3 exit code testsを追加する
- [x] 4.4 README usageを更新する
- [x] 4.5 rumdl / mado 参考点をdesignに残し、実装コピーしていないことを確認する

## Definition of Done

- [x] `kml check`, `kml check --fix`, `kml fix`, `kml fmt` の役割がREADMEとtestsで固定されていること
- [x] JSON/statistics/rule/config/stdinのCLI contractがtestsで固定されていること
- [x] include/exclude/gitignoreの挙動がtestsで固定されていること
