## Definition of Ready
- [ ] phase2 の rule / config quality が固まっていること
- [ ] `Cargo.toml` に入れる publish metadata の項目一覧が確定していること
- [ ] binary target 名と install 後の executable 名が `kml` として確定していること
- [ ] package name が `katana-markdown-linter`、license が MIT として確定していること
- [ ] initial distribution channel がユーザーと合意されていること

## 1. Package Metadata

- [x] 1.1 `Cargo.toml` に crates.io 向け metadata を追加する
- [x] 1.2 `license` / `readme` / `repository` / `description` / `keywords` / `categories` を `Cargo.toml` に設定する
- [ ] 1.3 package の include / exclude を定義する

## 2. Publish Validation

- [ ] 2.1 `cargo package` の検証を CI に追加する
- [ ] 2.2 `cargo publish --dry-run` を通す release gate を追加する
- [ ] 2.3 binary target が `cargo install` で導入できることを確認する

## 3. Release Readiness

- [x] 3.1 README に library API、CLI install、configuration、release policy の章を追加する
- [x] 3.2 README に library API、CLI install、configuration、release policy を記載する
- [ ] 3.3 publish 失敗時の復旧手順を release runbook に記載する

## Definition of Done
- [ ] crates.io 公開に必要な条件が README または release runbook に文書化されていること
- [ ] publish 前の dry-run が CI で検証されること
- [ ] `cargo install` 可能な binary target 名と executable 名が `kml` として固定されていること
