# v0.17.5 CLI help and version aliases design

## 方針

既存の軽量 parser を維持し、`--help` / `-h` を最優先で検出する。ヘルプ要求時は lint 対象の探索を行わず、標準出力に usage を出して exit code `0` で終了する。

## 設計

- `Command::Help(Option<HelpTopic>)` を追加し、global help と command help を同じ command として扱う。
- `HelpTopic` は既存 command 名からだけ解決する。未知の command は global help に倒す。
- `--version` / `-V` / `-v` は `Command::Version` として早期解決する。
- `kml <command> --help` と `kml <command> -h` は、その command の usage を表示する。
- 回帰テストは CLI binary を実行し、空の作業ディレクトリでも lint 処理へ進まないことを確認する。

## 互換性

既存の `kml version`、`kml --version`、`kml -V` は維持する。`-v` は新しく version alias として追加する。

## Homebrew tap 更新

`homebrew-katana` の `Formula/kml.rb` は `v0.17.1` のまま止まっていた。release workflow は formula を生成していたが、tap repository へ反映していなかったためである。

`v0.17.5` では、release workflow の GitHub Release 作成後に `HOMEBREW_KATANA_GIT_TOKEN` を使って `homebrew-katana` を更新する。`github.token` への fallback は持たない。

更新対象は次の 2 種類とする。

- `Formula/kml.rb`: 最新版を指す formula
- `Formula/kml@X.Y.Z.rb`: その version を固定して導入する formula

公開後検証では、生成した formula と実際の tap 上の formula を比較する。これにより、GitHub Release / npm / PyPI は新しいのに Homebrew だけ古い状態を検出する。

`v0.17.1` 以降の過去 version は、npm / PyPI の公開済み version に合わせ、`0.17.1`、`0.17.3`、`0.17.4` を versioned formula として登録する。`0.17.2` は npm / PyPI に存在しないため対象外とする。
