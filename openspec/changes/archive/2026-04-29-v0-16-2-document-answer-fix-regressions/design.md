# Document Answer Fix Regressions Design

## Context

`v0.16.1` までの品質確認は、rule fixture、public confidence、dogfood、収束性テストで診断と再実行安定性を確認してきた。
ただし `kml check --fix` 後の文書が、期待する完成形と完全一致しているかを大量文書で見る導線はまだ弱い。

`v0.16.2` では `v0.17.0` の配布拡張へ進む前に、実際の Markdown 文書を使って fix の正しさを確認する。
評価は「入力文書」「期待する answer 文書」「fix 後出力」の 3 点を固定し、answer と出力が byte-for-byte で一致しなければ bug として扱う。

## Goals / Non-Goals

**Goals:**

- public GitHub repository 由来の Markdown sample を 200 件以上集める
- original Markdown sample を 50 件作る
- original sample は 200 文字以上で、単純な文字列の羅列ではない実文書にする
- original sample は過去に検知・修正した bug pattern を複数組み合わせる
- 各 check 対象に `xxx_answer.md` を用意し、fix 後出力と完全一致で比較する
- 乖離を bug として分類し、`v0.16.2` の patch 範囲で修正する

**Non-Goals:**

- `v0.17.0` の Homebrew、standalone binary、npm、PyPI 配布対応
- 新しい lint rule の大規模追加
- answer を現在の実装出力から自動生成して正解扱いすること
- public GitHub sample の license を無視して第三者文書を無条件に repository へ取り込むこと

## Decisions

### D-1: corpus は public 200 件以上 + original 50 件に分ける

public GitHub sample は現実の文書分布を見るために使う。
original sample は過去 bug の複合再現に使う。

public sample には source repository、commit SHA、path、license、取得日、選定理由を manifest として残す。
repository に内容を取り込む場合は、license が fixture 再配布に使えることを確認する。

実装では license allowlist を `MIT` に固定し、公開済みの
`HiroyukiFuruno/katana-markdown-linter` repository 内 Markdown を public
corpus として使う。既存 repository の license boundary 内に閉じ、第三者文書の
再配布判断をこの patch release に混ぜない。

### D-2: original sample は historical bug mix として設計する

original sample は 1 rule 1 file の単体 fixture ではなく、過去の bug pattern を複数組み合わせる。
優先して混ぜる pattern は次の通り。

- `MD007` の nested list bad-fix
- `MD029` の nested unordered-list interruption
- `MD034` の bare URL / `ftp://` / `mailto:` と inline code / inline link の境界
- `MD046` の indented code block と fenced code block の混在
- `MD051` の emoji / CJK / Unicode heading anchor
- `MD052` の collapsed reference fix
- `MD056` の table column padding
- nested link text、reference link、inline code span、table、HTML の混在
- CRLF、final newline、Unicode byte offset、multi-byte character の境界

各 original sample は 200 文字以上の意味が通る Markdown 文書にする。
見出し、本文、list、link、code、table のような文書構造を持たせ、ランダム文字列や同一語の反復で文字数を満たすことは禁止する。

### D-3: answer は手で確認できる期待形として管理する

各入力 `xxx.md` に対して `xxx_answer.md` を置く。
answer は「現在の `kml` 出力をコピーしただけ」のファイルにしない。
人間が期待形として読める形にし、fix の対象外にすべき構造はそのまま残す。

### D-4: harness は完全一致だけを合格にする

検証 harness は次の順に実行する。

1. 入力 sample を一時ディレクトリへコピーする
2. `kml check --fix` を実行する
3. fix 後の `xxx.md` と `xxx_answer.md` を byte-for-byte で比較する
4. 差分がある場合は sample 名、rule、差分、分類を report に出す

比較は whitespace や改行の差も許容しない。
Markdown の見た目が同じでも byte が違えば failure とする。

### D-5: bugfix は検出された乖離だけに絞る

`v0.16.2` は patch release として扱う。
評価で見つかった乖離を修正するが、配布導線や package manager 対応は入れない。
rule 追加が必要に見える場合も、既存 rule / fix の bugfix として説明できる範囲に限定する。

## Risks / Trade-offs

- Risk: public GitHub sample の license が fixture 取り込みに適さない
  → license allowlist と manifest を必須にし、取り込めない文書は URL 参照だけにするか対象外にする
- Risk: answer 作成が現在実装の追認になる
  → answer は手で期待形を確認し、差分 review を tasks に残す
- Risk: 250 件以上の corpus で release check が遅くなる
  → 通常 test は deterministic subset、release gate は全件、必要なら target cache を使う
- Risk: 複合 sample が大きすぎて原因切り分けが難しい
  → report に rule 推定、差分、入力 path を出し、bug 修正時は狭い regression test も追加する

## Migration Plan

1. `v0.16.2` の corpus manifest schema を決める
2. public GitHub sample を 200 件以上収集し、license と source を記録する
3. historical bug pattern を分類し、original sample 50 件を作る
4. 各 sample に `xxx_answer.md` を用意する
5. answer 完全一致 harness を追加する
6. harness で検出した bug を修正する
7. `make release-check VERSION=v0.16.2` に接続する

## Open Questions

- Resolved: `v0.16.2` では `MIT` license allowlist と
  `HiroyukiFuruno/katana-markdown-linter` repository 内 Markdown に限定する。
