## ADDED Requirements

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
