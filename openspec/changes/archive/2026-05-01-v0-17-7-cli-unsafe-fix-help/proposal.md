# v0.17.7 CLI unsafe fix help proposal

## 目的

`kml check --fix --unsafe --yes` が利用できることを CLI help と README から分かるようにする。

## 背景

`fix` と `check --fix` は同じ修正処理を共有しており、unsafe fix も `--unsafe --yes` で明示的に許可できる。
しかし `kml check --help` には `--unsafe --yes` が表示されず、README の例も `kml fix --unsafe --yes` に偏っていた。

この状態では利用者が `check --fix` では unsafe fix を使えないと誤解するため、CLI 契約と公開ドキュメントを揃える。

## 範囲

- `kml check --help` に `--unsafe --yes` を表示する
- global help の説明を `fix` と `check --fix` の両方に合わせる
- `check --fix --unsafe --yes` が unsafe fix を適用する CLI 回帰テストを追加する
- README の CLI usage に `check --fix --unsafe --yes` の例を追加する
- `0.17.7` の release metadata と release notes を更新する

## 範囲外

- unsafe fix の対象ルール追加
- confirmation prompt の仕様変更
- `fix` と `check --fix` の挙動差分追加
- `v0.18.0` 以降の schema / editor extension change の実装
