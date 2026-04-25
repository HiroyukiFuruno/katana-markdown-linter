## Why

`v0.8.0` は linter 精度と safe fix 拡充を default safe behavior の範囲で進める。
一方で、mado や rumdl のように広い fix 対応を目指す場合、意味・構造・表示が変わる可能性がある fix candidate を将来的に扱う必要がある。

safe / unsafe はプログラムが完全に判定できる真偽値ではない。
kml としては「default mode で保証する fix policy」と「ユーザーが危険性を理解して opt-in した fix policy」を分離する。

CLI で unsafe fix を使う場合は、誤実行を避けるため interactive confirmation を要求する。
この change は `v0.9.0` の計画であり、`v0.8.0` の linter 精度 / safe fix expansion には含めない。

## What Changes

- fix candidate に safety metadata を持たせる
- default `kml fix` / `kml check --fix` は safe fix のみ適用する
- `--unsafe` 指定時のみ unsafe fix candidate を適用対象に含める
- CLI で `--unsafe` を指定した場合、TTY では `[Y/n]` confirmation を要求する
- non-interactive 実行では `--unsafe` 単独を fail させ、明示的な automation opt-in を要求する
- JSON output と dashboard で safe / unsafe / manual-required を区別して可視化する

## User Decisions

- `v0.8.0` は linter 精度と safe fix 拡充を優先し、unsafe fix mode は含めない
- unsafe fix mode は `v0.9.0` の計画として扱う
- default は safe に寄せる
- CLI から unsafe fix を利用する場合は `[Y/n]` confirmation を求める

## Impact

- default CLI の安全性を維持したまま fix 対応範囲を広げられる
- unsafe fix を automation で使う場合も、明示的な opt-in が必要になる
- rule fixture matrix と coverage dashboard が fixable count だけでなく safety policy を示せる
- library consumer は safety metadata を見て独自 UI や confirmation policy を実装できる
