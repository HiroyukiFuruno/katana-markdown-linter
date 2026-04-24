## Why

`kml` CLI は library-first の補助として始まったが、release後は単体CLIとしても利用される。
rumdl / mado のようなRust製Markdown linterを参考に、check/fixだけでなく、開発者が日常的に使えるCLI UXへ近づける必要がある。

## What Changes

- `check --fix` と `fmt` を追加し、fix用途をCLI上で自然にする
- `--diff`、`--statistics`、`--quiet`、`--verbose`、`--output json` を追加する
- `rule` / `rule MD013` でrule一覧と詳細を表示する
- `config` subcommandで読み込まれたconfigの確認を可能にする
- `--stdin` による標準入力check/fixを追加する
- include/exclude/gitignore関連optionを整理する

## Impact

- CLI単体利用時の体験が向上する
- CI / editor / pre-commit連携がしやすくなる
- library APIを壊さずにCLIを拡張できる
