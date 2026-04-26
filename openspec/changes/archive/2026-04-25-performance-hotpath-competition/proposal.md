## Why

`kml` は benchmark harness と cross-tool comparison を持つようになったが、現状は report-first であり、hot path を継続的に削る具体的な profiling / optimization loop はまだ薄い。
`mado` や `rumdl` と張り合うこと自体が目的ではないが、同じ条件で大きく遅い状態は避けたい。

ms 単位の改善を狙うには、measurement、profiling、targeted optimization、regression visibility を分けて扱う必要がある。

## What Changes

- `kml` / `mado` / `rumdl` の common subset benchmark を再現しやすくする
- hot path を flamegraph / sampling / internal counters で特定する
- directory traversal、config loading、rule registry、line scanning、fix loop の最適化余地を順に検証する
- performance docs に current baseline と改善結果を残す
- CI hard gate にはせず、local regression visibility として維持する

## Impact

- 速さの議論が体感ではなく report に基づく
- optimization が API behavior を壊していないことを fixture で確認できる
- future release 前に performance regression を見つけやすくなる
