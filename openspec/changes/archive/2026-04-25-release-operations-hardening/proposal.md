## Why

`v0.3.0` release では最終的に GitHub Release と crates.io publish まで成功したが、途中で release workflow が失敗し、未公開 tag の付け直しが必要になった。
原因は local `make release-check` と GitHub Actions の Clippy 対象が一致していなかったこと、tag push と publish dispatch の役割が運用上わかりにくかったこと。

release は失敗時の復旧手順を誤ると tag / GitHub Release / crates.io version の整合性を壊すため、次回以降は local preflight、tag 作成、GitHub Release、crates.io publish、retry 判断を明確に分離する。

## What Changes

- local `make release-check` と CI release workflow の gate 差分を検出できるようにする
- GitHub 上で `Verified` と表示される tagger/signing identity を release flow に固定する
- tag 作成後の workflow failure に対する復旧手順を文書化し、Makefile から確認できるようにする
- 「GitHub Release だけ作る」「crates.io まで publish する」を明確に分ける
- release 済み version の tag 付け直しや crates.io republish が不可能なケースを fail-fast する
- changelog / release notes / package metadata / tag target の整合チェックを release 前後に固定する

## Impact

- release 前に GitHub runner と同じ対象を local で検証できる
- GitHub 上で `Unverified` な release tag を作らない
- release 失敗時に tag を消してよい条件と消してはいけない条件が明確になる
- GitHub Release と crates.io の publish 状態を機械的に確認できる
- 次回 release で同じ種類の手戻りが発生しにくくなる
