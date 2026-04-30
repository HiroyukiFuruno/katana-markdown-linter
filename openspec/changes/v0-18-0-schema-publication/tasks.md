# Tasks

## Definition of Ready

- [ ] 0.1 `schema/markdownlint.schema.json` が現在の `kml config schema --output json` と一致しているか確認する
- [ ] 0.2 stable schema URL の実体をどこで配信するか確認する
- [ ] 0.3 versioned schema artifact 名を release asset naming に合わせて決める
- [ ] 0.4 editor extension 実装はこの change に含めないことを確認する

## 1. Schema Generation and Regression

- [ ] 1.1 schema generation / comparison script を追加する
- [ ] 1.2 `make schema-check` を追加する
- [ ] 1.3 `kml config schema --output json` と `schema/markdownlint.schema.json` の一致テストを追加する
- [ ] 1.4 schema compatibility fixture を追加する
- [ ] 1.5 rule metadata 変更時に schema fixture 更新漏れで失敗する test を追加する

## 2. Published Schema Contract

- [ ] 2.1 `$id` と stable schema URL の扱いを docs に固定する
- [ ] 2.2 versioned schema artifact を release output に追加する
- [ ] 2.3 stable URL と pinned release artifact URL の使い分けを docs に書く
- [ ] 2.4 schema compatibility policy を `docs/editor-integration.md` または `docs/distribution.md` に追加する

## 3. Release Gate

- [ ] 3.1 `make release-check VERSION=v0.18.0` に schema check を追加する
- [ ] 3.2 release workflow に schema artifact upload または publication step を追加する
- [ ] 3.3 release preflight workflow に schema check を追加する
- [ ] 3.4 AST lint で release gate から schema check が外れないことを検証する
- [ ] 3.5 release notes が schema diff の有無を説明できるようにする

## 4. Editor Documentation

- [ ] 4.1 README の config schema section を published schema contract に合わせる
- [ ] 4.2 `docs/editor-integration.md` の VS Code schema mapping を更新する
- [ ] 4.3 `docs/editor-integration.md` の Zed schema mapping を更新する
- [ ] 4.4 local schema file を使う fallback 手順を更新する
- [ ] 4.5 editor extension 実装予定は後続 change への参照に留める
- [ ] 4.6 npm package README に `kml help` / `kml --help` / `kml -h` と command help の例を追加する
- [ ] 4.7 PyPI package README に `kml help` / `kml --help` / `kml -h` と command help の例を追加する
- [ ] 4.8 npm `description` と PyPI `summary` を help / version 対応後の CLI 導線が伝わる文言へ更新する

## 5. Verification

- [ ] 5.1 `make fmt-check`
- [ ] 5.2 `make lint`
- [ ] 5.3 `make ast-lint`
- [ ] 5.4 `cargo test --workspace --locked`
- [ ] 5.5 `make dogfood`
- [ ] 5.6 `git diff --check`
- [ ] 5.7 `make schema-check`
- [ ] 5.8 `make release-check VERSION=v0.18.0`

## Definition of Done

- [ ] 6.1 committed schema file と CLI schema output の一致が機械的に検証される
- [ ] 6.2 stable URL と versioned schema artifact の両方が docs で説明されている
- [ ] 6.3 release gate が schema publication を検証する
- [ ] 6.4 editor integration docs が extension なしの config validation を説明している
- [ ] 6.5 `v0.18.1` の VS Code extension が依存できる schema contract が固定されている
- [ ] 6.6 npm / PyPI の README と package description が help / version 導線を説明している
