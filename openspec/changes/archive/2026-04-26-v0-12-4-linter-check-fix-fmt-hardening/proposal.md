# v0.12.4 Linter / Check / Fix / Fmt Hardening

## Why

`v0.12.3` で lint 精度、速度、単体テスト（UT）/結合テスト（IT）の補強は進んだが、次に MCP Registry や配布展開へ進む前に、製品の中心である `linter` / `check` / `fix` / `fmt` の信頼性をもう一段固める必要がある。

このリポジトリは Markdown linter であり、利用者が最初に信頼するのは次の動作である。

- `check` が正しく診断し、ファイルを書き換えないこと
- `fix` が安全な修正だけを適用し、残った違反を見失わないこと
- `fmt` が lint fix の別名ではなく、決定的で冪等な整形（formatting）として動くこと
- 速度改善が診断・修正・整形の意味を変えないこと

そのため `v0.12.4` は展開前の patch release として、中核コマンドの品質・速度・テスト証拠を優先する。

## What Changes

- `linter` / `check` / `fix` / `fmt` の既知課題を棚卸しし、`v0.12.4` 対象と後続対象を分離する。
- `check`、`check --fix`、`fix`、`fmt` の責務、終了コード（exit code）、標準出力（stdout）、JSON 出力、ファイル変更有無を明確化する。
- `fmt` を lint fix の別名ではなく、レイアウト整形（layout formatting）専用の契約として再固定する。
- rule 単位テスト、文書単位 fixture、CLI 結合テストを追加し、誤検知・検出漏れ・安全修正・整形の回帰を防ぐ。
- `fix` / `fmt` の収束性（convergence）と冪等性（idempotence）を検証する。
- `check` / `fix` / `fmt` の代表経路で速度を測定し、速度改善が正しさを崩していないことを確認する。
- `v0.12.4` release gate に中核コマンドの確認を追加する。

## Scope

- `check` の no-write contract と診断精度
- `fix` / `check --fix` の safe fix contract、衝突回避、残存診断
- `fmt` の整形範囲、stdin/stdout、冪等性
- CLI text / JSON output contract
- rule precision regression coverage
- dogfood、corpus、benchmark、CI での検証証拠
- `v0.12.4` version / changelog / release readiness

## Out of Scope

- MCP Registry / Hub への公開
- MCP package artifact の新規実装
- 遠隔 MCP 接続（remote MCP transport）
- unsafe fix の対象拡大
- 新しい大規模 rule family の追加
- editor / distribution integration の拡張
- 既存 JSON schema を壊す breaking change

## Impact

`v0.12.4` 完了後に `v0.13.0` 以降の配布・公開作業へ戻る。配布面を広げる前に、CLI と linter core の契約を固定することで、後続の利用経路でも同じ品質を維持できる。
