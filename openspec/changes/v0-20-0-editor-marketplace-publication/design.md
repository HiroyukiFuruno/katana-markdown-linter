# Design

## 方針

`v0.20.0` は editor marketplace 公開専用の change とする。editor 機能の不足修正や dogfood のバグ修正は、この change へ混ぜない。`v0-19-0-editor-capability-completion` が完了し、final editor dogfood の release-blocking finding が 0 件であることを DoR にする。

## 公開前提

- `v0-19-0-editor-capability-completion` が完了している。
- final editor dogfood evidence が存在し、診断・整形・安全な修正・config changes を含む。
- VS Code publisher、extension name、token secret が確認済み。
- Zed extension registry PR が merged である。
- target version が GitHub Release、crates.io、npm、PyPI、Homebrew、VS Code Marketplace、Zed registry のいずれにも既存ではない。

## Release workflow

- editor marketplace publish は明示 input / env が有効な場合だけ実行する。
- VS Code publish は `VSCE_PAT` secret がない場合に fail fast する。
- Zed publish は upstream registry PR URL がない、または未mergeの場合に fail fast する。
- publish job は core release の状態と分離して記録し、partial publish の状態を release verification で説明する。

## Verification

- `release-target-check` は target version の既存公開状態を editor marketplaces まで確認する。
- `release-verify` は publication flag に応じて `published` / `deferred` / `failed` を説明する。
- VS Code Marketplace と Zed registry の両方で target version が確認できるまで、`v0.20.0` は完了扱いにしない。

## 非対象

- LSP config 解決。
- editor diagnostics / quick fix の挙動修正。
- final editor dogfood で見つかった release-blocking issue の修正。
- `v0.18.7` 事故版の再公開。

## Rollback / retry

- 既に外部 registry に公開された version は同じ意味で再利用しない。
- 片方の marketplace だけ公開された場合、状態を evidence に残し、同一 version で安全に retry できるかを registry ごとに確認する。
- 内容変更が必要な場合は、次 version へ進める。
