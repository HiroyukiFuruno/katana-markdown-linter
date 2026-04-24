## Context

現状のCLIは `check` / `fix` / `init-config`、`--config`、`--format json`、`--file`、ディレクトリ再帰探索を持つ。
rumdl は `check --fix`、`fmt`、`--diff`、`--statistics`、`rule`、`config`、`--stdin`、include/exclude/gitignoreなどのUXが充実している。
mado は高速なRust製Markdown linterとしてCLI操作と性能観点の参考になる。

## Goals / Non-Goals

**Goals:**

- `kml check` を日常的なCI lint入口にする
- `kml check --fix` と `kml fmt` を自然なfix入口にする
- JSON出力、statistics、rule detail、config introspectionを提供する
- stdin/stdoutでeditor integrationしやすくする
- include/exclude/gitignoreの挙動を明示する

**Non-Goals:**

- rumdl / mado の完全互換CLIにしない
- config formatをTOMLへ変更しない
- library APIにCLI固有概念を漏らさない

## Decisions

### 1. Existing commands は互換維持する

既存の `kml fix` は残す。
`kml check --fix` と `kml fmt` は追加の入口として提供する。

### 2. Output option は `--output` を正とし、`--format` は互換aliasにする

rumdlに寄せて `--output json` を追加する。
既存 `--format json` は後方互換として維持する。

### 3. stdin は明示optionにする

`--stdin` がある場合はfilesystem探索を行わず、stdin内容を仮想pathまたは`<stdin>`としてlintする。
fix結果はstdoutへ出す。

### 4. include/exclude は明示挙動を持つ

directory scanはgitignoreを尊重する。
explicit fileは原則includeするが、`--force-exclude` がある場合はexcludeを適用する。

## Risks / Trade-offs

- CLI拡張でparse_argsが複雑になるため、将来的にclap導入を検討する余地がある
- stdout/stderrの契約を誤るとeditor integrationが壊れる
- `fmt` と `fix` のexit code差を曖昧にするとCI利用で混乱する

## Migration Plan

1. CLI command/option contractをREADMEとtestsで固定する
2. `check --fix` と `fmt` を追加する
3. output/statistics/rule/config/stdinを追加する
4. include/exclude/gitignoreを追加する
5. CLI snapshot testsを追加する
