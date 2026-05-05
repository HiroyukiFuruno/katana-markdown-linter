# v0.18.5 Release Verification Hardening

## 対象バージョン

- `v0.18.5`

## 目的

`v0.18.5` では、外部配布系を含む部分公開事故を失敗扱いにし、release の分岐判定を機械的に確定する。

## 変更内容

- `release-check` / `release-verify` の分岐差分を固定する
- external registry / wrapper / Homebrew / marketplace publish 停止条件を検証対象へ追加する
- GitHub Release など partial publish を `fail-fast` で扱う
- `release` の再開条件を明文化する

## 非対応範囲

- 実際の marketplace account 操作は含めない
- エンジン本体変更は含めない
