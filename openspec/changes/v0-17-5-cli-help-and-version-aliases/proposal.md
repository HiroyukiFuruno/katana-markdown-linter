# v0.17.5 CLI help and version aliases proposal

## 目的

`kml help`、`kml --help`、`kml -h` がヘルプを表示し、`kml -v` も `kml version` と同じように version を表示できる状態にする。

## 背景

`kml help` は入力ファイル `help` として扱われ、`kml --help` はヘルプ表示ではなく通常の検査処理へ流れていた。利用者が CLI の使い方を確認する入口として壊れているため、動作修正として patch release に含める。

## 範囲

- global help: `kml help`、`kml --help`、`kml -h`
- command help: `kml <command> --help`、`kml <command> -h`
- version aliases: `kml version`、`kml --version`、`kml -V`、`kml -v`
- Homebrew tap の current formula と versioned formula の自動更新
- npm / PyPI に公開済みの `v0.17.1` 以降に合わせた Homebrew versioned formula 登録
- README / CHANGELOG / release metadata の `0.17.5` 更新

## 範囲外

- 引数 parser の crate 置き換え
- subcommand ごとの詳細な tutorial 化
- 既存 lint / fix / fmt の処理仕様変更
- npm / PyPI に存在しない `v0.17.2` の Homebrew versioned formula 登録
