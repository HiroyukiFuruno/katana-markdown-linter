# v0.19.0 Editor Publication Gate

## 背景

- v0.18.7 は公開事故で npm が再リリース不能になり、`v0.18.x` の意味のある進行が止まっています。
- 現在は `v0.18.x` を「実害がある報告ベースの bugfix（check/fix/format）」で進める方針を継続したいです。
- `v0.19.0` の判断前に、kml 自身に対する dogfood (`just dogfood`) を release-readiness に固定したいです。
- `v0.19.0` は、VS Code / Zed の Marketplace 公開実装が実行へ入る場合のみ進行します。

## 方針

1. `v0.18.7` は、同一版番号の再リリースができない事故版として扱い、次の有効リリースは `v0.18.8` から始める。
2. `v0.18.x` はバージョン bump の条件を満たす限り「報告ベースの linter バグfix（check/fix/format）」で積む。
3. `v0.19.0` は、Marketplace 公開実装が実効的に入る場合のみ申請する。
4. 失敗しにくい公開判定を事前に固定する。具体的には、以下を release gate に明文化する。
   - 既存チャネル（npm / PyPI / crates.io / GitHub Release / Homebrew）で対象 version が既存ならリリース失敗させる。
   - marketplace 公開前提条件（account / publisher / package / verification）が未設定なら公開を停止する。
   - published / deferred のどちらでも、release-verify が実状態を説明できる。
5. `just dogfood` で `README.md docs openspec` を毎回実行し、自己リポジトリの新規 warning 増加を release blocker とする。
6. この change の proposal / design / tasks / spec は、再公開不可、`v0.19.0` 判定、dogfood 前提の3軸を同一条件で扱うことを DoR / DoD の成立条件に含める。
