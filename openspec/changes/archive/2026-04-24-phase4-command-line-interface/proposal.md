## Why

この crate を単なるライブラリで終わらせず、markdownlint 互換の操作を実行できる CLI として提供したい。
phase4 は、`check`、`fix`、`.markdownlint.json` 作成 helper を使う entrypoint を定義する。

## What Changes

- `check` と `fix` の CLI command を追加する
- `.markdownlint.json` を作成する `init-config` helper を追加する
- markdownlint に近い config discovery と exit code の contract を定義する
- file / glob 入力に対して lint を実行できるようにする
- `rumdl` と `mado` を `check` / `fix` の UX 参考として確認し、CLI 仕様に反映する

## Impact

- ユーザーは cargo install 後に即座に Markdown lint を実行できる
- config file の作成が初回導入で迷わない
- library API と CLI の contract が分離される
