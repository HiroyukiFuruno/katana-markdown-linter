# Public Confidence Hardening

## Target Version

`v0.12.9`

## Why

`v0.12.8` は stable score 100 点、technical hard blocker 0 件、release verification 完了まで到達した。

一方で、MCP Registry や配布導線の拡大に進むには、内部 fixture だけでなく、実運用に近い Markdown 文書群で `check` / `fix` / `fmt` の精度、速度、収束性をもう一段確認したい。

`v0.12.9` は、公開範囲を広げる前の最終 confidence gate として扱う。

## What Changes

- KatanA の `docs/` と `assets/**/*.md` を候補に、実文書 dogfood corpus を定義する
- KatanA corpus を必須 CI に直接依存させず、許可された範囲で curated fixture または任意 external dogfood として扱う
- 既存 performance corpus と KatanA 系 corpus の役割を分ける
- `check` / `fix` / `fmt` の実運用寄り収束テストを拡張する
- external corpus で見つかった誤検知、検出漏れ、性能劣化、formatter 差分を分類して regression test に落とす
- `v0.13.0` の配布計画へ進む条件に、`v0.12.9` の public confidence gate 完了を追加する

## Impact

- 内部品質だけでなく、外部品質への自信を数値と evidence で説明できる
- 配布導線を広げる前に、実文書での release-blocking issue を潰せる
- KatanA 側の実文書を直接壊さず、再現可能な形で confidence evidence を残せる

## Non-Goals

- この change では MCP Registry、MCP package、remote MCP を進めない
- この change では新しい user-facing feature を追加しない
- この change では KatanA repository の文書を直接変更しない
- この change では private repository の全文を無条件に fixture として取り込まない
