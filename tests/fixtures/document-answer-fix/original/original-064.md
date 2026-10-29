# Nested emphasis and links

This page places _outer *nested strong* text_ beside [a linked _emphasis_ label](https://example.invalid/guide). The nested delimiters are intentional, and the link destination must not be confused with punctuation in the label. A style-only fix can change emphasis markers only when the resulting document keeps the same nesting, link target, and literal wording.

A later line includes the Japanese phrase [変更手順](#変更手順) and inline code `*not emphasis*`, preserving the split between parsed markup and literal code.

## 変更手順

この節は上のリンク先となり、入れ子の強調とコード内の文字列をそのまま保つ手順を説明します。
