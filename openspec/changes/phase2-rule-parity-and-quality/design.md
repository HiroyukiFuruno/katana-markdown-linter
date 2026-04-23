## Context

公式 markdownlint は rule ID、description、tags、config schema、fix 可否を含む明確な contract を持っている。
この phase では、既存実装だけに閉じず、公式 documentation と upstream implementation を照らし合わせて rule coverage を引き上げる。

## Goals / Non-Goals

**Goals:**

- 公式 docs にある rule をカタログ化し、check 実装の抜けをなくす
- upstream implementation または official documentation から安全な fix behavior を確認できる rule については自動修正の contract を持たせる
- `.markdownlint.json` の default / override / validation を扱う helper を整備する
- rule ごとの品質を unit / integration テストで守る

**Non-Goals:**

- CLI 体験の最終確定
- crates.io release 作業
- editor integration
- 利用側アプリケーション固有の diagnostic adapter

## Decisions

### 1. rule catalog は upstream docs と implementation を source of truth にする

ルール一覧、ID、説明、config property の基準は、手作業の抜粋ではなく official markdownlint documentation と upstream implementation を source of truth にする。
これにより、後続の差分確認がしやすくなる。

### 2. fix は rule 単位で明示的に分ける

check と fix を同じ実装に混ぜるのではなく、fix 可否と unsupported reason を rule metadata に持たせる。
一括修正の対象を安全に判定するためである。

### 3. config helper は生成と検証を分離する

`.markdownlint.json` を作る helper と、既存 config を読む helper を分ける。
生成ロジックと validation ロジックを一緒にすると、後で CLI から呼ぶときに扱いづらくなる。

## Risks / Trade-offs

- 公式 docs との同期コストがある
- fix の実装範囲が広いので、fixability は `fixable` / `not_fixable` / `unknown_needs_review` の状態で管理する
- config schema の吸収が甘いと、CLI 側の UX に負債が残る

## Confirmed Decisions

以下はユーザー確認済みの決定事項である。

- phase2 の完了条件は全 active rule の check 実装必須とする
- 実装開始時点で未対応 rule を可視化し、実装対象一覧として固定する
- official source と fixability 判断は phase5 の default branch 追従 contract と矛盾しない形で、upstream docs と upstream implementation を参照する
- `.markdownlint.json` helper の default は upstream default を基準にする
- 特定アプリケーション専用 preset はこの crate の責務に含めない

## Migration Plan

1. rule catalog と metadata を upstream docs / implementation ベースで確定し、各 rule の coverage status を記録する
2. check と fix の評価経路を分離する
3. config helper を追加する
4. rule 単位のテストを増やして品質を固定する
