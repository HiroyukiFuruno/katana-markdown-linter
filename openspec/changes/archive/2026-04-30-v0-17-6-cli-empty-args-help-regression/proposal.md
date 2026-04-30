# v0.17.6 CLI entrypoint and config compatibility regression proposal

## 目的

`kml` を引数なしで実行した場合に、workspace の Markdown 検査ではなく global help を表示する。

## 背景

`v0.17.5` で `kml help` / `kml --help` / `kml -h` / `kml -v` は追加されたが、引数なし `kml` は `check` の既定処理へ残った。
そのため repository root で `kml` を実行すると、意図せずファイル探索と lint が走る。

また、ユーザー環境では PATH 上の `~/.cargo/bin/kml` が Homebrew の `/opt/homebrew/bin/kml` より先にあり、古い `0.16.1` が `kml help` を入力ファイルとして扱っていた。
これは配布版の PATH shadowing であり、製品修正としては引数なし entrypoint の回帰を `v0.17.6` で閉じる。

追加で、`kml check` はこの repository の `.markdownlint.json` に含まれる公式 markdownlint alias と `MD022.lines_below` の数値設定を config error として扱っていた。
これは dogfood の信頼性に直結するため、同じ `v0.17.6` の回帰修正に含める。

## 範囲

- `parse_args(Vec::new())` を global help として扱う
- `--locale ja` 指定時の help を日本語で表示する
- CLI binary の引数なし実行を回帰テストに追加する
- 公式 markdownlint alias を設定キーとして受け付ける
- `MD022.lines_above` / `MD022.lines_below` の `integer|integer[]` 設定を受け付ける
- config error がある場合は既定で lint 実行へ進まず、設定修正または `--ignore-config-errors` を案内する
- alias と `integer|integer[]` を含む config の CLI 回帰テストを追加する
- `0.17.6` の version metadata と release notes を更新する

## 範囲外

- PATH 上の古い `~/.cargo/bin/kml` の削除や自動置換
- 引数 parser crate への置き換え
- `v0.18.0` に送った npm / PyPI README / description 改善
- 日本語以外の help 翻訳の追加
