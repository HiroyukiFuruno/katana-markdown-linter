# v0.17.6 CLI entrypoint and config compatibility regression design

## 方針

内部用の `Cli::default()` は既存テストや workflow helper が `Check` として使っているため変更しない。
外部 entrypoint の `parse_args` だけで空引数を `Command::Help(None)` に早期解決する。

## 設計

- `parse_args` の先頭で `args.is_empty()` を判定する
- 空引数の場合は `Command::Help(None)` を返す
- 既存の `requests_help` / `requests_version` の優先順位は維持する
- help 分岐でも `--locale` / `--local` / `-l` を保持し、日本語 locale では日本語 help を返す
- CLI binary テストは空の作業ディレクトリで `kml` を実行し、exit code `0`、stderr 空、stdout に global usage が出ることを確認する
- `OfficialRuleMeta` に公式 alias を保持し、config 読み込み時に alias を canonical rule id へ解決する
- config schema は canonical rule id と alias の両方を同じ rule schema で公開する
- `RulePropertyType` に `integer|integer[]` 相当の型を追加し、`MD022.lines_above` / `MD022.lines_below` の数値と数値配列を受け付ける
- `check` / `fix` はファイル lint 前に有効設定を検証し、config error があれば既定では lint に進まない
- `--ignore-config-errors` 指定時は config error を出力しつつ、不正な設定項目を無視して lint を続行する

## 検証観点

- `kml` が Markdown file discovery を起動しない
- `kml help` / `kml --help` / `kml -h` が引き続き global help を出す
- `kml --locale ja help` / `kml check --help --locale ja` が日本語 help を出す
- `kml version` / `kml --version` / `kml -V` / `kml -v` が引き続き version を出す
- `kml check --config .markdownlint.json` が公式 alias と `MD022.lines_below` を config error にしない
- config error が残る場合は lint 診断と混ざらず、設定修正または `--ignore-config-errors` の案内が出る
- dogfood が専用 config へ逃げなくても config validation 起因では落ちない
- Homebrew 版 `0.17.5` の `kml help` / `kml -v` は正常で、今回の実修正対象は引数なしであることを記録する
