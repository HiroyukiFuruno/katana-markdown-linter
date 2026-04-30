# Document Answer Fix Regressions

## Target Version

`v0.16.2`

## Why

`v0.17.0` の配布拡張へ進む前に、実際の Markdown 文書に対する `kml check --fix` の正しさを patch release として固める。
従来の「診断件数」や「収束性」だけでは、fix 後の文書が期待形と完全一致しているかを検証できない。

## What Changes

- public GitHub repository から 200 件以上の Markdown sample を収集し、source URL、commit、license、選定理由を manifest に記録する
- project 内で 50 件の original Markdown sample を作成し、各 sample は単純な文字列の羅列ではない 200 文字以上の文書にする
- original sample は過去に検知・修正した bug pattern を単体で再現するだけでなく、複数の pattern を同じ文書内に組み合わせる
- check 対象ごとに `xxx_answer.md` を用意し、`kml check --fix` 後の出力が answer と byte-for-byte で一致することを検証する
- answer と 1 byte でも乖離した場合は bug として分類し、`v0.16.2` の patch 範囲で修正する
- 検証 harness を `make` target と release gate に接続し、同じ bug が再発しないようにする
- `v0.16.2` では新しい配布導線、package manager 対応、rule の大規模拡張は扱わない

## Capabilities

### New Capabilities

- `document-answer-fix-evaluation`: public GitHub sample と original sample を使い、fix 後文書を answer fixture と完全一致で検証する評価導線を扱う

### Modified Capabilities

- `release-readiness`: `v0.16.2` の release readiness に document answer fix evaluation を追加する

## Impact

- `tests/fixtures/**`
- `tests/**`
- `scripts/ci/**`
- `Makefile`
- `openspec/changes/active-roadmap.md`
- `CHANGELOG.md`
- `Cargo.toml` / `Cargo.lock`
- 検出された bug に対応する `src/**`
