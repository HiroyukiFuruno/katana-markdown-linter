## Purpose

元の Markdown ソース位置を保持した document context を定義し、rule 実行、診断、修正範囲、性能確認を同じ前提で扱うための契約を定義する。

## Requirements

### Requirement: system SHALL build a source-preserving document context

システムは、Markdown 文書の元ソースを保持したまま rule execution で共有できる document context を構築しなければならない（SHALL）。

#### Scenario: context を構築する

- **WHEN** system が Markdown content と file path を受け取る
- **THEN** system は original content への参照を保持する
- **THEN** system は line offsets と line/column conversion helper を提供する
- **THEN** system は diagnostics と fixes を original source byte range から line/column に変換できる
- **THEN** system は empty input、末尾改行なし、CRLF、Unicode を正しく扱う

### Requirement: system SHALL expose structural indexes through document context

システムは、rule family が重複 scan を避けられるように構造 index を document context から参照できなければならない（SHALL）。

#### Scenario: structural index を利用する

- **WHEN** heading、reference、code fence、table 系 rule が document context を利用する
- **THEN** system は code block ranges、heading entries、reference entries、table block entries を共有できる
- **THEN** system は fenced code block 内の Markdown 構文を通常本文と誤認しない
- **THEN** system は source range を original content の byte range として保持する

### Requirement: rule evaluation SHALL support context-based execution without breaking legacy APIs

rule evaluation は、context-based execution をサポートしつつ既存 public API を壊してはならない（SHALL）。

#### Scenario: 既存 API から lint する

- **WHEN** caller が既存の `lint(content, &LintOptions)` または `fix(content, &LintOptions)` を呼び出す
- **THEN** system は source-compatible な API を維持する
- **THEN** system は内部で必要に応じて document context を構築できる
- **THEN** system は CLI JSON shape、text output ordering、exit code contract を維持する

#### Scenario: rule を context-based に移行する

- **WHEN** migrated rule が document context を利用して diagnostics または fixes を生成する
- **THEN** system は legacy rule path と同じ rule enablement と config resolution を適用する
- **THEN** system は upstream golden comparison と fixture matrix で unexplained behavior delta を出さない

### Requirement: AST support SHALL be optional and lazy

AST support は optional かつ lazy でなければならず、fix range の唯一の根拠になってはならない（SHALL）。

#### Scenario: AST を必要としない rule を実行する

- **WHEN** line-local rule または既存 scan で十分な rule が実行される
- **THEN** system は AST parser を必須にしない
- **THEN** system は AST construction cost を全 rule に強制しない

#### Scenario: AST と markdownlint semantics が乖離する

- **WHEN** AST parser の解釈が markdownlint-compatible behavior と異なる
- **THEN** system は markdownlint-compatible behavior を優先する
- **THEN** system は乖離を known delta または implementation note として記録する
- **THEN** system は AST-normalized text から unsafe な fix range を生成しない

### Requirement: context migration SHALL include representative rule families

context migration は、v0.5.0 の範囲で少なくとも代表 rule family を移行しなければならない（SHALL）。

#### Scenario: v0.5.0 migration を完了する

- **WHEN** v0.5.0 の source-preserving document context change が完了する
- **THEN** system は少なくとも heading family の 1 つを context-based evaluation に移行する
- **THEN** system は少なくとも reference、table、または code fence family の 1 つを context-based evaluation に移行する
- **THEN** system は移行済み rule の check と safe fix が original source range を使うことを test で固定する

### Requirement: context migration SHALL be performance-gated

context migration は、performance baseline と behavior gates を通して評価されなければならない（SHALL）。

#### Scenario: migration performance を検証する

- **WHEN** developer が `just perf-check` を実行する
- **THEN** system は context migration 後の report を baseline と比較する
- **THEN** system は unexplained regression を failure または documented blocker として扱う
- **THEN** system は large document API lint、large document API fix、CLI many-small-files path を確認対象に含める

#### Scenario: migration behavior を検証する

- **WHEN** developer が verification suite を実行する
- **THEN** system は upstream golden comparison を通す
- **THEN** system は rule fixture harness を通す
- **THEN** system は dogfood を通し、baseline diagnostics の意図しない増加を許さない

### Requirement: context-sensitive rule evaluation SHALL not rely on isolated line text alone

context-sensitive rule evaluation は、単一行文字列だけに依存してはならない（SHALL NOT）。

#### Scenario: code block 内の Markdown 風テキストを評価する

- **WHEN** rule が table、link、emphasis、command prompt、heading、list に見える行を評価する
- **THEN** system はその行が fenced code block、indented code block、inline code span、HTML block、table block のどれに属するか確認する
- **THEN** system は context 上除外すべき行を通常本文として診断しない

### Requirement: document context SHALL support both backtick and tilde fences

document context は backtick fence と tilde fence の両方を扱わなければならない（SHALL）。

#### Scenario: tilde fence を解析する

- **WHEN** Markdown content に `~~~mermaid` または `~~~` fenced code block が含まれる
- **THEN** system はその範囲を code block として記録する
- **THEN** system は同じ fence kind かつ opening fence 以上の長さの closing fence だけで block を閉じる
- **THEN** system は code block 内の pipe、URL、list marker、dollar prompt を通常本文として診断しない

### Requirement: document context SHALL expose source-preserving inline tokens

`DocumentContext` は、構文依存 rule が共有できる inline token を source range 付きで提供しなければならない（SHALL）。

#### Scenario: inline token を参照する

- **WHEN** rule が link、inline code、image、reference を評価する
- **THEN** system は元 source の byte range を持つ token を返す
- **THEN** system は token text と destination を元 source から参照する
- **THEN** system は AST 正規化済み文字列を fix range の唯一の根拠にしない

### Requirement: inline token parsing SHALL handle Markdown boundary cases

inline token parser は、Markdown の境界値を rule ごとの手書き判定より優先して扱わなければならない（SHALL）。

#### Scenario: 境界値を解析する

- **WHEN** content に nested bracket、link title、inline code、image、autolink、reference definition が含まれる
- **THEN** system は通常本文として評価する範囲と除外する範囲を区別する
- **THEN** system は unclosed marker を通常 link と誤認しない
- **THEN** system は CRLF と Unicode を含む source range を保持する

### Requirement: context-sensitive rules SHALL declare their context source

context-sensitive rule は、自身がどの context source を根拠に評価するかを分類しなければならない（SHALL）。

#### Scenario: rule migration を棚卸しする

- **WHEN** developer が context-sensitive rule を確認する
- **THEN** system は rule を `parser-backed`、`document-context-backed`、`line-local-by-spec`、`future-work` のいずれかに分類する
- **THEN** system は `future-work` の理由と次の解消条件を tasks に記録する

### Requirement: migrated rules SHALL reuse shared structural indexes

migrated rule は、構文除外判定を rule-local の重複 scan だけに依存してはならない（SHALL NOT）。

#### Scenario: migrated rule を実行する

- **WHEN** migrated rule が Markdown 風テキストを評価する
- **THEN** system は `DocumentContext` または shared parser token から context を取得する
- **THEN** system は fenced code block、inline code、HTML、table、reference definition を通常本文と混同しない
