## Purpose

自動修正の適用結果を呼び出し側が追跡できるように、`FixResult` と `FixDetail` が公開する詳細情報の契約を定義する。

## Requirements

### Requirement: FixResult SHALL expose per-fix detail information

`fix_with_results` および `fix_with_results_including_unsafe` が返す `FixResult` SHALL include
適用またはスキップされた各 fix の詳細情報を `details: Vec<FixDetail>` フィールドとして含むこと。

`FixDetail` は、以下のフィールドを持つ公開型として定義されなければならない（SHALL）。

- `rule_id: String` — 修正を生成したルール ID（例: `"MD034"`）
- `range: Range` — 修正が対象とする行・列の範囲（既存の `Range` 型を再利用）
- `applied: bool` — 実際に適用されたか（`true`）、スキップされたか（`false`）

#### Scenario: safe fix が適用されたときに FixDetail が返る

- **WHEN** `fix_with_results` を呼び出し、fixable な診断が 1 件以上ある文書を渡す
- **THEN** `FixResult.details` には applied=true の `FixDetail` が少なくとも 1 件含まれる
- **THEN** 各 `FixDetail.rule_id` は対応する `LintResult.rule_id` と一致する
- **THEN** 各 `FixDetail.range` は対応する `Fix.range` と一致する

#### Scenario: 重複する fix がある場合は後勝ち（先にソートされた edit が優先）でスキップ

- **WHEN** 同一範囲に複数の fix が競合する文書を渡す
- **THEN** 適用された fix の `FixDetail.applied` は `true`
- **THEN** スキップされた fix の `FixDetail.applied` は `false`
- **THEN** `FixResult.applied_fixes` は `details.iter().filter(|d| d.applied).count()` と一致する

#### Scenario: unsafe fix を除外したとき unsafe な FixDetail は含まれない

- **WHEN** `fix_with_results` を呼び出す（include_unsafe = false）
- **THEN** `FixResult.details` には `FixSafety::Unsafe` な fix の detail は含まれない

#### Scenario: FixDetail は Serialize を実装する

- **WHEN** `FixDetail` 値を JSON シリアライズする
- **THEN** `rule_id`、`range`、`applied` フィールドが JSON オブジェクトとして出力される
