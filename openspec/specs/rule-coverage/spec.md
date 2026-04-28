## Purpose

Rule coverage exposes markdownlint-compatible rule metadata, checking, and fix behavior.

## Requirements

### Requirement: system SHALL expose every official markdownlint rule in the rule catalog

システムは、公式 markdownlint documentation にある active rule を rule catalog として公開し、deprecated / removed rule は lifecycle state として区別しなければならない（SHALL）。

#### Scenario: rule catalog を参照する

- **WHEN** developer が利用可能な rule 一覧を取得する
- **THEN** system は official docs にある active rule ID と説明を返す
- **THEN** system は各 rule の metadata を区別できる
- **THEN** system は deprecated / removed rule を active rule と混同しない

### Requirement: system SHALL check documents according to official markdownlint rule behavior

システムは、公式 markdownlint の rule behavior に準じて Markdown 文書を check しなければならない（SHALL）。

#### Scenario: 文書を check する

- **WHEN** system が Markdown 文書を解析する
- **THEN** system は rule ごとの違反を返す
- **THEN** system は rule の設定に従って enable / disable を判定する
- **THEN** system は fixture matrix の pass / fail 期待値と一致する判定を返す
- **THEN** system は境界値・イレギュラー条件に対しても仕様準拠の判定を返す

### Requirement: system SHALL support automatic fix only for rules that define fix behavior

システムは、upstream implementation または official documentation から安全な fix behavior を確認できる rule に対してのみ自動修正を適用しなければならない（SHALL）。

#### Scenario: fix を実行する

- **WHEN** system が document fix を要求される
- **THEN** system は fixable rule の修正だけを適用する
- **THEN** system は fix 非対応 rule を無理に変更しない
- **THEN** system は fix 非対応 rule の理由を metadata として保持する

### Requirement: MD003 safe fix SHALL preserve heading meaning and source ranges

`MD003` の safe-fix は、見出しレベルと本文を保持し、対象行だけを置き換えなければならない（SHALL）。

#### Scenario: setext H1 を ATX H1 に変換する

- **WHEN** system が `Heading\n=======\n` を `MD003` の fix 対象として処理する
- **THEN** system は `# Heading\n` へ変換する
- **THEN** system は見出し本文を変更しない
- **THEN** system は underline 行を残さない

#### Scenario: setext H2 を ATX H2 に変換する

- **WHEN** system が `Heading\n-------\n` を `MD003` の fix 対象として処理する
- **THEN** system は `## Heading\n` へ変換する
- **THEN** system は見出しレベルを保持する

#### Scenario: front matter と horizontal rule を修正しない

- **WHEN** system が front matter delimiter または独立した horizontal rule を含む Markdown を処理する
- **THEN** system はそれらを `MD003` safe-fix 対象にしない
- **THEN** system は既存の誤検知回避を維持する

### Requirement: MD028 fix policy SHALL be decided before enabling automatic fixes

`MD028` は、文意を変えない安全条件が定義できる場合にだけ自動修正を提供しなければならない（SHALL）。

#### Scenario: safe subset を実装できる場合

- **WHEN** developer が `MD028` の safe subset を定義する
- **THEN** system はその subset を fixture と unit test で固定する
- **THEN** system は GFM Alert 間の空行を修正対象にしない
- **THEN** system は `MD028` を safe-fix allowlist に追加する

#### Scenario: safe subset を定義できない場合

- **WHEN** developer が `MD028` の自動修正に人間の意図が必要だと判断する
- **THEN** system は `MD028` を `Diagnostic only` のまま維持する
- **THEN** system は `v0.12.21` の by-design 宣言対象に `MD028` を追加する
- **THEN** system は README と fixture matrix に理由を反映する

### Requirement: system SHALL generate a rule fixture parity matrix from official markdownlint documentation

システムは、公式 markdownlint rule document から rule ごとの fixture parity matrix を生成しなければならない（SHALL）。

#### Scenario: fixture matrix を生成する

- **WHEN** developer が upstream rule document を入力する
- **THEN** system は rule id、aliases、tags、parameters、fixability を抽出する
- **THEN** system は check / fix / config / edge の fixture coverage を rule ごとに出力する
- **THEN** system は自動抽出できない記載を `manual_required` として可視化する

### Requirement: system SHALL report stale or incomplete rule fixtures

システムは、公式 markdownlint document と local fixture matrix の乖離を報告しなければならない（SHALL）。

#### Scenario: fixture drift を検出する

- **WHEN** upstream rule document が変更される
- **THEN** system は local fixture matrix と比較する
- **THEN** system は missing fixture、stale fixture、manual_required の数を報告する
- **THEN** system は未知の fixture drift を品質ゲートで検出できる

### Requirement: system SHALL publish a rule coverage dashboard

システムは、rule ごとの coverage と compatibility 状態を一覧できる dashboard を公開しなければならない（SHALL）。

#### Scenario: dashboard を生成する

- **WHEN** developer が coverage dashboard generation を実行する
- **THEN** system は rule ID ごとの check、fix、config、edge、golden comparison、known delta の状態を出力する
- **THEN** system は missing coverage と failing golden comparison を区別して表示する
- **THEN** system は dashboard を Markdown または JSON の再利用可能な形式で生成する

### Requirement: rule coverage gate SHALL fail on unknown golden deltas

rule coverage gate は、許可されていない upstream golden delta を失敗として扱わなければならない（SHALL）。

#### Scenario: unknown delta を検出する

- **WHEN** golden comparison が known delta にない差分を検出する
- **THEN** system は gate を failure にする
- **THEN** system は該当 rule、fixture、expected、actual を report する

### Requirement: system SHALL maintain file-level false-positive regression coverage

システムは、単体 rule test だけでなく、実文書に近い複数行 Markdown fixture で誤検知回帰を防がなければならない（SHALL）。

#### Scenario: mixed Markdown fixture を検証する

- **WHEN** developer が regression tests を実行する
- **THEN** system は HTML block、inline code、fenced code block、math、table、nested list、reference link を含む fixture を検証する
- **THEN** system は code block 内の Markdown 風テキストを通常本文として診断しない
- **THEN** system は HTML attribute 内 URL や inline code 内 URL を bare URL として診断しない

### Requirement: context-sensitive rules SHALL be tracked by risk category

システムは、context-sensitive rule の残課題を risk category 付きで追跡しなければならない（SHALL）。

#### Scenario: rule risk を棚卸しする

- **WHEN** developer が issue inventory を更新する
- **THEN** system は rule ごとに `bug`、`test-gap`、`design-debt`、`ci-gap` のいずれかで分類する
- **THEN** system は今回対応対象と後続 change 対象を区別する
- **THEN** system は user-reported false positive が chat history のみに残らないよう tasks に記録する

### Requirement: false-positive fixes SHALL include rule-local and document-level tests

誤検知修正は、rule-local test と document-level test の両方で固定されなければならない（SHALL）。

#### Scenario: 誤検知を修正する

- **WHEN** developer が false positive を修正する
- **THEN** system は該当 rule の最小再現 test を追加する
- **THEN** system は mixed document fixture に再発条件を追加する
- **THEN** system は unrelated diagnostics が増えていないことを確認する

### Requirement: precision fixes SHALL cover false positives and false negatives

精度修正（precision fix）は誤検知と検出漏れの両方を扱わなければならない（SHALL）。

#### Scenario: precision issue を修正する

- **WHEN** developer が rule behavior を変更する
- **THEN** system は issue を `bug`、`test-gap`、`design-debt`、`perf-risk` のいずれかで分類する
- **THEN** system は最小再現のルール単位テスト（rule-local test）を追加する
- **THEN** system は必要に応じて文書単位fixture（document-level fixture）に複数構文が混ざった再発条件を追加する

### Requirement: context-sensitive rule tests SHALL include structural exclusion cases

context-sensitive rule test は、通常本文ではない構造の除外ケースを含まなければならない（SHALL）。

#### Scenario: structural exclusion を検証する

- **WHEN** rule が URL、link、table、heading、list、command prompt、emphasis に見える行を評価する
- **THEN** system は inline code、fenced code block、HTML block、table block、reference definition 内の期待値を検証する
- **THEN** system は unrelated diagnostics が増えていないことを確認する
- **THEN** system は fixture 追加だけで済むものと実装修正（implementation fix）が必要なものを区別する

### Requirement: linter hardening SHALL prioritize existing rule correctness before distribution expansion

システムは、配布経路の拡大より前に既存 rule の正しさを優先して固めなければならない（SHALL）。

#### Scenario: core rule issue を棚卸しする

- **WHEN** developer が `v0.12.4` の実装に着手する
- **THEN** system は既存 rule の誤検知、検出漏れ、fix 事故、test gap を分類する
- **THEN** system は patch release で扱う対象と後続 change に送る対象を区別する
- **THEN** system は fixture 追加だけで済むものと実装修正が必要なものを区別する

### Requirement: safe fix behavior SHALL converge and remain idempotent

safe fix behavior は収束し、再実行時に不要な差分を増やしてはならない（SHALL）。

#### Scenario: safe fix を再実行する

- **WHEN** system が `check --fix` または `fix` を同じ content に複数回実行する
- **THEN** system は初回で適用できる default-safe fix を適用する
- **THEN** system は再実行時に同じ修正を繰り返し適用しない
- **THEN** system は残存違反を diagnostics として報告し続ける
- **THEN** system は unsafe fix を default-safe fix と混同しない

### Requirement: check, fix, and fmt corpus SHALL include mixed Markdown structures

`check`、`fix`、`fmt` の corpus は、複数構文が混ざる Markdown 文書を含まなければならない（SHALL）。

#### Scenario: mixed corpus を検証する

- **WHEN** developer が corpus tests を実行する
- **THEN** system は fenced code block、inline code、HTML block、table、nested list、reference link を含む fixture を検証する
- **THEN** system は check diagnostics、fixed content、formatted content を別々に検証する
- **THEN** system は unrelated diagnostics や不要な formatting diff が増えていないことを確認する

### Requirement: external corpus findings SHALL be classified before release expansion

外部 corpus で見つかった finding は、配布展開へ進む前に分類されなければならない（SHALL）。

#### Scenario: external finding を分類する

- **WHEN** developer が KatanA corpus または curated public confidence fixture の diagnostic を確認する
- **THEN** system は finding を `true-positive`、`false-positive`、`false-negative`、`unsafe-fix-risk`、`fmt-policy-gap`、`perf-regression` のいずれかに分類する
- **THEN** system は release-blocking issue と non-blocking follow-up を分ける
- **THEN** system は未分類の high-risk finding が残る場合、release を進めない

### Requirement: external precision fixes SHALL become regression tests

外部 corpus で見つかった精度修正は、回帰テストとして固定されなければならない（SHALL）。

#### Scenario: external precision issue を修正する

- **WHEN** developer が external corpus 由来の false-positive または false-negative を修正する
- **THEN** system は rule-local test を追加する
- **THEN** system は document-level fixture または curated public confidence fixture に再発条件を追加する
- **THEN** system は unrelated diagnostics が増えていないことを確認する

### Requirement: link-family precision SHALL use shared parser evidence

link 系 rule の精度改善は、共有 parser または `DocumentContext` の token index に基づかなければならない（SHALL）。

#### Scenario: link 系 rule を評価する

- **WHEN** `MD034`、`MD051`、`MD052`、`MD054`、`MD059` が document を評価する
- **THEN** system は inline code、image、reference definition、autolink を通常本文と混同しない
- **THEN** system は rule ごとの独立した簡易 parser を増やさない
- **THEN** system は誤検知と検出漏れを rule-local test と document-level fixture の両方で固定する

#### Scenario: MD051 が emoji・CJK 混在見出しのフラグメントを正しく生成する

- **WHEN** 見出しテキストに emoji または CJK 文字が含まれる Markdown を評価する
- **THEN** system は GitHub と同じフラグメント生成ルール（小文字化・スペース→ハイフン・emoji 除去・Unicode 字母保持）を適用する
- **THEN** system は emoji のみで構成された見出しを空フラグメント（検出対象外）として扱う
- **THEN** system は CJK 文字を含む見出しに対して false positive を出さない
- **THEN** system は誤ったフラグメントを参照するリンクに対して false negative を出さない

#### Scenario: MD056 が列数不足のテーブル行を空セルで補完する

- **WHEN** テーブル行のセル数が header のセル数より少ない（`row.cells.len() < expected_columns`）Markdown を fix モードで処理する
- **THEN** system は不足分だけ空セルを補完し、行のパイプスタイル（leading/trailing）を保持する
- **THEN** system は補完後の行が header と同じ列数になるように修正する
- **THEN** system は `row.safe_to_fix=false`（escaped pipe や inline code を含む行）は fix 対象外として診断のみ返す
- **THEN** system は列数過多の行（`row.cells.len() > expected_columns`）に対しては fix を生成せず診断のみ返す（データ消失防止）

### Requirement: parser migration SHALL document non-migrated rules

parser migration は、移譲しない rule の理由を明文化しなければならない（SHALL）。

#### Scenario: rule を移譲しない

- **WHEN** rule が `v0.12.5` で shared parser へ移譲されない
- **THEN** system は理由を `already-context-safe`、`line-local-by-spec`、`future-ast-required`、`blocked` のいずれかで記録する
- **THEN** system は future work が必要なものを後続 version の tasks に残す

### Requirement: context-sensitive migration SHALL cover rule families

context-sensitive migration は、単一 rule ではなく rule family 単位で coverage を固定しなければならない（SHALL）。

#### Scenario: rule family を検証する

- **WHEN** developer が migration tests を実行する
- **THEN** system は link/reference、inline content、block structure、table/list/heading/fence の fixture を検証する
- **THEN** system は false positive と false negative の両方を検証する
- **THEN** system は unrelated diagnostics が増えていないことを確認する

### Requirement: safe fixes SHALL preserve source-range intent after migration

safe fix を持つ rule は、migration 後も source range の意図を維持しなければならない（SHALL）。

#### Scenario: migrated fix を適用する

- **WHEN** system が migrated rule の fix を適用する
- **THEN** system は original source range に基づいて replacement を適用する
- **THEN** system は overlapping fix と adjacent fix の競合を検出または安全に解決する
- **THEN** system は unsafe fix を default-safe fix と混同しない

### Requirement: migrated rule corpus SHALL prove convergence

migrated rule corpus は、検出精度だけでなく fix / fmt の収束性を検証しなければならない（SHALL）。

#### Scenario: migrated corpus を検証する

- **WHEN** developer が migrated corpus tests を実行する
- **THEN** system は check diagnostics、fixed content、formatted content を別々に検証する
- **THEN** system は repeated fix で不要差分が増えないことを確認する
- **THEN** system は repeated fmt で不要差分が増えないことを確認する

### Requirement: stable scoring SHALL include precision correctness

stable scoring は、rule precision を最大配点の評価対象にしなければならない（SHALL）。

#### Scenario: precision correctness を採点する

- **WHEN** developer が stable score を算出する
- **THEN** system は誤検知、検出漏れ、構文除外、fixture matrix、upstream golden の結果を 40 点満点で採点する
- **THEN** system は未分類の高優先度誤検知または検出漏れを hard blocker として扱う
- **THEN** system は accepted limitation を future work として記録する

### Requirement: stable scoring SHALL include safe command behavior

stable scoring は、safe command behavior を評価しなければならない（SHALL）。

#### Scenario: safe command behavior を採点する

- **WHEN** developer が stable score を算出する
- **THEN** system は `check` no-write、safe fix、fix/fmt idempotence、collision safety を 20 点満点で採点する
- **THEN** system は unsafe fix が default-safe fix に混ざる場合を hard blocker として扱う
- **THEN** system は repeated fix/fmt で不要差分が増える場合を hard blocker として扱う
