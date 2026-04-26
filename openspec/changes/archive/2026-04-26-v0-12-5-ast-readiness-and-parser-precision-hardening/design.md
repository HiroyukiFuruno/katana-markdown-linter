# Design

## Position

`v0.12.5` は安定版そのものではない。
安定版を評価できる状態へ進むための parser / AST readiness change とする。

配布展開に関係する task は扱わない。
判断軸は精度、速度、check / fix / fmt の安定性に限定する。

## Parser Boundary

source-preserving parser は、元の Markdown source に対する byte range を保持する。
AST 正規化済み文字列を fix range の根拠にしない。

最初に扱う token:

- inline code span
- inline link
- image link
- collapsed / full reference link
- reference definition
- autolink

token は `DocumentContext` の lazy index として保持する。
public API には出さず、rule 実装の内部入力として使う。

## Rule Migration

優先 rule は link / inline code に強く依存するものに絞る。

- `MD034`: bare URL の除外判定を shared token に寄せる
- `MD051`: fragment link の抽出を shared link token に寄せる
- `MD052`: missing reference の判定を shared reference token に寄せる
- `MD054`: link style 判定を shared link/reference token に寄せる
- `MD059`: link text 判定を shared link token に寄せる

`MD053` は duplicate reference definition の rule なので、reference definition index の整備対象には含めるが、必要以上に behavior を変えない。

## Parser Readiness Inventory

`v0.12.5` では、rule を以下の観点で棚卸しする。

- `link`: `MD034`、`MD051`、`MD052`、`MD054`、`MD059` は shared inline token へ移譲する。
- `reference`: `MD053` は shared reference definition index と整合させる。
- `inline-code`: link 系 rule の除外根拠として shared inline code span を使う。
- `HTML`: `MD034` の HTML attribute 除外は既存の局所判定を残し、後続 AST 移譲候補とする。
- `code-block`: 既存 `DocumentContext` の code block range を継続利用する。
- `table`: `MD055`、`MD056` は既存 table index があり、`v0.12.5` では移譲済み扱いにする。
- `list`: `MD029` は直近修正済みだが、全体 AST 移譲は `v0.12.6` 以降へ送る。
- `heading`: `MD051` の fragment 検証は既存 heading index と shared link token を併用する。

移譲しない rule の理由は `already-context-safe`、`line-local-by-spec`、`future-ast-required`、`blocked` のいずれかで扱う。
`v0.12.5` での外部 parser dependency は不要と判断する。

## Dependency Policy

外部 parser dependency は既定では追加しない。
内部 parser で markdownlint-compatible behavior を満たせないことが実装中に判明した場合だけ、設計判断としてユーザー確認を行う。

## Performance Policy

AST / parser readiness は速度劣化を許容しない。
ただし CI は不安定な wall-clock threshold だけで release を止めない。

追加する測定観点:

- link-heavy document
- inline-code-heavy document
- reference-heavy document
- parser index construction

## Release Gate

`v0.12.5` は以下を満たす。

- rule-local test と document-level fixture の両方で精度改善を固定する
- `make perf-check` に parser 関連 case を追加する
- `check --fix`、`fix`、`fmt` の意味を変えない
- `v0.13.0` の配布 task を進めない
