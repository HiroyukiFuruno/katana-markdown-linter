# Tasks

## Definition of Ready

- [x] 0.1 `v0.18.4` の gate finalization が完了している

## v0.18.5 Hardening

- [x] 1.1 external registry / wrapper / Homebrew の停止条件を script 化する
- [x] 1.2 partial publish（例: GitHub Release 先行）のテストを追加する
- [x] 1.3 `release-check` と `release-verify` の判定キーを共通化する

## Definition of Done

- [x] 2.1 partial publish が成功扱いにならないことを再現手順付きで確認する
- [x] 2.2 停止条件と再試行条件が runbook と一致する
- [x] 2.3 `v0.18.5` 判定を DoR/DoD で説明可能

## 品質評価スコア

| 項目 | スコア |
| --- | --- |
| 実装精度 | 9 |
| テスト網羅性 | 9 |
| ドキュメント | 9 |
| **合計** | **27** |

## v0.18.6 への引き継ぎ事項

- [ ] editor-publish-gate / wrapper-publish-gate / homebrew-publish-gate の判定結果を state.json に集約する
- [ ] verify-release-published.sh の各 assertion ごとに独立したテストケースを整備する
