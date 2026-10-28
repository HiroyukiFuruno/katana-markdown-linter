## Why

kml の共有構文解析には CommonMark/GFM の有効構文を誤認する箇所がある。仕様への継続追従を回帰検査へ落とし、既存ルールの精度を改善して次の minor 版 v0.20.0 を公開する。

## What Changes

- CommonMark 0.31.2 と GFM 0.29-gfm、および GitHub 固有拡張の対応範囲を記録する。
- エスケープ済み表区切り、フェンスの境界、タブ区切り ATX 見出し、複数行コードスパンを共有解析で改善する。
- Open Issue を照合し、GitHub Actions の使用 Action と Rust 依存関係を最新互換版へ更新・検証する。
- v0.20.0 を Draft PR、レビュー対応、CI、merge、公開確認、ローカル整理まで進める。

## Capabilities

### New Capabilities

- `markdown-spec-tracking`: 仕様の一次資料、対応範囲、回帰検査の追跡。

### Modified Capabilities

既存公開 API は保持する。共有構文解析の精度改善は新しい検証契約へ追加する。

## Impact

共有 DocumentContext、inline scanner、回帰テスト、英語の公開資料、CI Action、依存 lockfile、全配布版メタデータ。別リポジトリの直接編集は含まない。
