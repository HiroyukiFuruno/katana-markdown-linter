## Definition of Ready
- [ ] phase3 の publish validation が CI で通っていること
- [ ] CLI command set が `check` / `fix` / `init-config` に固定されていること
- [ ] `.markdownlint.json` helper の出力仕様が固まっていること
- [ ] executable name が `kml` として固定されていること
- [ ] `.markdownlint.json` と `.markdownlint.jsonc` の両方を config 入力として扱うことが確定していること
- [ ] `--format json` を phase4 に含めることが確定していること
- [ ] `fix` exit code と glob handling がユーザーと合意されていること

## 1. Commands

- [ ] 1.1 `check` command を追加する
- [ ] 1.2 `fix` command を追加する
- [ ] 1.3 `.markdownlint.json` を作る `init-config` command を追加する
- [ ] 1.4 `--format json` を `check` と `fix` command に追加する

## 2. Configuration Discovery

- [ ] 2.1 `--config` で明示指定できるようにする
- [ ] 2.2 `.markdownlint.json` / `.markdownlint.jsonc` の探索順を `--config`、current directory、parent directory search の順で実装する
- [ ] 2.3 helper が生成した config をそのまま再利用できるようにする

## 3. UX and Exit Codes

- [ ] 3.1 check / fix の exit code contract を実装し、違反あり / IO error / config error を区別する
- [ ] 3.2 file / glob 入力の contract を実装し、対象なしの場合の扱いを明示する
- [ ] 3.3 失敗理由を CLI 出力で rule error / config error / filesystem error として判別できるようにする

## Definition of Done
- [ ] cargo install 後に CLI が使えること
- [ ] check / fix / init-config がそれぞれ独立して動き、exit code contract がテストで固定されていること
- [ ] `.markdownlint.json` helper が初回導入で役に立つこと
