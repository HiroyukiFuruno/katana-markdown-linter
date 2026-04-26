# Stable Candidate Acceptance

## Target Version

`v0.12.8`

## Why

安定版と言えるかどうかは、雰囲気では判断できない。
`v0.12.5` から `v0.12.7` で精度、速度、収束性を固めた後、`v0.12.8` では数値化した score と hard blocker で安定版候補を評価する。

最終判断は自動化しない。
score と evidence を提示した上で、ユーザーが安定版として受け入れるかを判断する。

## What Changes

- 安定版スコア（stable score）を 100 点満点で定義する
- score が高くても即不合格にする hard blocker を定義する
- `v0.12.8` の最終工程にユーザー受け入れ判断を追加する
- `v0.13.0` の配布展開は、ユーザー受け入れ後にだけ DoR を満たす
- score evidence を release tasks に残す

## Impact

- 「安定版と言ってよい状態」が再現可能になる
- 次の agent が主観で release / distribution を進めなくなる
- `v0.13.0` に進む条件が明確になる

## Non-Goals

- この change では MCP Registry / Hub 公開を行わない
- この change では package artifact 実装を行わない
- この change では scoring のためだけに gate を弱めない
- この change ではユーザー受け入れ判断を agent が代行しない
