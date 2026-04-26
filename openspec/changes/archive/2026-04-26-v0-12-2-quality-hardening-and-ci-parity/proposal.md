## Why

`v0.12.1` では code block、HTML、link、math、nested list、table まわりの誤検知を修正したが、ユーザー指摘で発覚したものが多く、事前検出の仕組みがまだ弱い。

また、`kml` は Rust binary として Windows でも動作する前提だが、現在の CI matrix は macOS / Ubuntu のみで、Windows runner 上の build、test、CLI smoke が release 前に保証されていない。

CI cache も通常 CI は手書きの `actions/cache`、release / preflight は `Swatinem/rust-cache` に分かれており、cache key、target cache、lockfile 変更時の挙動、OS 別の再利用性が明確ではない。

`v0.12.2` は機能追加ではなく、品質不安を解消する patch release として扱う。

## What Changes

- CI matrix に Windows を追加するべきかを実行可能性込みで検証し、追加する
- Windows runner で使う shell、GNU make、path、`.exe` suffix、Bash script 呼び出しを明確にする
- 通常 CI / release preflight / release workflow の cache strategy を比較し、重複や非効率を整理する
- `Swatinem/rust-cache` への統一、または手書き cache を残す理由を明文化する
- 誤検知が出やすい rule と context を棚卸しし、ファイル単位の回帰 fixture を拡張する
- 単一行判定に依存している rule を洗い出し、`DocumentContext` へ移行する優先順位を決める
- `v0.12.2` 以降に回す課題は「未対応理由」と「次の change」を明記して残す

## Impact

- Windows user が release 前に CI で保護される
- CI 実行時間と cache hit/miss の説明責任が上がる
- 誤検知修正が個別対応で終わらず、再発防止用 fixture と rule migration backlog に残る
- `v0.13.0` の MCP Registry planning に進む前に、現在の linter 品質を安定させられる

## Non-Goals

- `v0.13.0` の MCP Registry / Hub 公開
- remote MCP transport
- 全 rule の一括 AST 化
- formatter の新機能追加
- release workflow を OS matrix 化して複数 OS から crates.io publish すること
