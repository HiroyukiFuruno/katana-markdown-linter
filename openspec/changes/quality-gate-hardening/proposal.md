## Why

rule fixture、rule implementation、CLI拡張が進むと、品質ゲートが弱いままではregressionを防げない。
KatanAの`lint` / `ast-lint`のように、通常のClippyだけでなくrepository固有の不変条件を検査するgateを強化する必要がある。

## What Changes

- `make lint` と `make ast-lint` の責務を明確化する
- ast-lintにrule fixture coverage、upstream drift、release tag、CLI traversalなどの不変条件を追加する
- CI required checksとMakefile targetの対応を整理する
- coverageをreportとblocking gateに分ける
- release前に必要なquality gatesをrunbook化する

## Impact

- rule/CLI/releaseのregressionが早期に検出される
- KatanAと同じ運用思想で品質ゲートを維持できる
- required checkとlocal make targetの対応が明確になる
