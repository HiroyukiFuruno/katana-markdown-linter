## Context

`MD003` は現状、setext 見出しを検出しているが `fix_info` を持たない。
`MD028` は空行で分断された blockquote を検出し、GFM Alert は既に例外として扱っている。

`MD028` の公式説明は 2 つの修正案を示す。
1 つ目は引用の間に本文を追加して別引用として明示する方法、2 つ目は空行に `>` を足して同じ引用として扱う方法である。
後者は自動化しやすいが、「同じ引用にする」という人間の判断を含む。

## Goals / Non-Goals

**Goals:**

- `MD003` の低リスクな safe-fix を実装する。
- `MD028` は安全条件を先に定義し、条件を満たせない場合は診断のみとして明文化する。
- どちらの rule も fixture matrix と README の表示を現在の実態に合わせる。
- `make` entrypoint で検証する。

**Non-Goals:**

- `MD013` の改行整形や `MD043` の見出し挿入など、人間の判断が強い修正は扱わない。
- `MD028` を広く自動修正して文意変更リスクを取ることはしない。
- unsafe fix mode の拡張は扱わない。

## Decisions

### D-1. `MD003` は setext から ATX への変換を主対象にする

`MD003` の現在の診断は setext marker 行を見て、診断位置を見出し本文行へ向けている。
実装では、見出し本文行と underline 行の 2 行を置き換える `fix_info` を作る。

- `=` underline は `# Heading` に変換する。
- `-` underline は `## Heading` に変換する。
- 前後の本文や空行は触らない。
- front matter と horizontal rule は既存の除外条件を維持する。

`style = atx_closed` などの closed ATX は、末尾 marker の付与が安全に定義できる場合のみ対象にする。
判断が複雑化する場合は `atx` 系の最小 safe subset から始める。

### D-2. `MD028` は policy-first にする

`MD028` は「空行に `>` を入れる」修正が自然に見えるが、別々の引用を 1 つの引用へ寄せる可能性がある。
そのため、実装前に以下を tasks で明示的に判定する。

- 同一段落の継続として扱える根拠があるか。
- GFM Alert 間の空行を壊さないか。
- 連続 blockquote の見た目を変えるだけでなく、意味も変えていないと言えるか。

根拠が弱い場合は `MD028` を `v0.12.21` の by-design 対象へ送る。

### D-3. 表示更新は実装結果と同期する

README の rule map は公開向けなので英語で更新する。
OpenSpec と tasks は日本語で記録する。
README または docs を触った場合は `make ast-lint` を必ず通す。

## Risks / Trade-offs

- [Risk] `MD003` の設定値を広く扱いすぎると誤修正につながる。
  - Mitigation: 最初は source range と変換結果が明確な subset に限定する。
- [Risk] `MD028` の自動修正が文意を変える。
  - Mitigation: 安全条件を満たせない場合は実装せず、`v0.12.21` の明示的な宣言対象にする。
- [Risk] `MD003` と `MD022` など周辺の heading rule の fix が同じ行で競合する。
  - Mitigation: 既存の overlap resolution と fixture で収束性を確認する。
