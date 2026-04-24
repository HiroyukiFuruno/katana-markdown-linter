## Definition of Ready

- [x] phase3 の publish validation が CI で通っていること
- [x] phase1, phase2, phase3 の `tasks.md` が全て完了していること
- [x] CLI command set が `check` / `fix` / `init-config` に固定されていること
- [x] `.markdownlint.json` helper の出力仕様が固まっていること
- [x] executable name が `kml` として固定されていること
- [x] `.markdownlint.json` と `.markdownlint.jsonc` の両方を config 入力として扱うことが確定していること
- [x] `--format json` を phase4 に含めることが確定していること
- [x] `fix` exit code と glob handling がユーザーと合意されていること

## 1. Commands

- [x] 1.1 `check` command を追加する
- [x] 1.2 `fix` command を追加する
- [x] 1.3 `.markdownlint.json` を作る `init-config` command を追加する
- [x] 1.4 `--format json` を `check` と `fix` command に追加する
- [x] 1.5 `rumdl` と `mado` の `check` / `fix` UX を参照し、`kml` の command contract に反映する

## 2. Configuration Discovery

- [x] 2.1 `--config` で明示指定できるようにする
- [x] 2.2 `.markdownlint.json` / `.markdownlint.jsonc` の探索順を `--config`、current directory、parent directory search の順で実装する
- [x] 2.3 helper が生成した config をそのまま再利用できるようにする

## 3. UX and Exit Codes

- [x] 3.1 check / fix の exit code contract を実装し、違反あり / IO error / config error を区別する
- [x] 3.2 file / glob 入力の contract を実装し、対象なしの場合の扱いを明示する
- [x] 3.3 失敗理由を CLI 出力で rule error / config error / filesystem error として判別できるようにする

## Definition of Done

- [x] cargo install 後に CLI が使えること
- [x] check / fix / init-config がそれぞれ独立して動き、exit code contract がテストで固定されていること
- [x] `.markdownlint.json` helper が初回導入で役に立つこと
