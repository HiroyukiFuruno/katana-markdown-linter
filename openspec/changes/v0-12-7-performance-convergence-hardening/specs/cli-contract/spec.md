## ADDED Requirements

### Requirement: core command workflows SHALL remain convergent after parser migration

parser migration 後も、core command workflow は収束しなければならない（SHALL）。

#### Scenario: core command workflow を再実行する

- **WHEN** system が `check`、`check --fix`、`fix`、`fmt`、`check`、`fmt` を同じ corpus に順番に実行する
- **THEN** `check` は入力を書き換えない
- **THEN** `check --fix` と `fix` は default-safe fix だけを適用する
- **THEN** `fmt` は formatter policy の範囲だけを変更する
- **THEN** 再実行で同じ変更を繰り返さない
- **THEN** stdout JSON shape と exit code contract は維持される
