# Tasks

## Definition of Ready

 - [x] 0.1 `schema/markdownlint.schema.json` が現在の `kml config schema --output json` と一致しているか確認する
 - [x] 0.2 stable schema URL の実体をどこで配信するか確認する
 - [x] 0.3 versioned schema artifact 名を release asset naming に合わせて決める
 - [x] 0.4 editor extension 実装はこの change に含めないことを確認する

## 1. Schema Generation and Regression

 - [x] 1.1 schema generation / comparison script を追加する
 - [x] 1.2 `just schema-check` を追加する
 - [x] 1.3 `kml config schema --output json` と `schema/markdownlint.schema.json` の一致テストを追加する
 - [x] 1.4 schema compatibility fixture を追加する
 - [x] 1.5 rule metadata 変更時に schema fixture 更新漏れで失敗する test を追加する

## 2. Published Schema Contract

 - [x] 2.1 `$id` と stable schema URL の扱いを docs に固定する
 - [x] 2.2 versioned schema artifact を release output に追加する
 - [x] 2.3 stable URL と pinned release artifact URL の使い分けを docs に書く
 - [x] 2.4 schema compatibility policy を `docs/editor-integration.md` または `docs/distribution.md` に追加する

## 3. Release Gate

 - [x] 3.1 `just VERSION=v0.18.0 release-check` に schema check を追加する
 - [x] 3.2 release workflow に schema artifact upload または publication step を追加する
 - [x] 3.3 release preflight workflow に schema check を追加する
 - [x] 3.4 AST lint で release gate から schema check が外れないことを検証する
 - [x] 3.5 release notes が schema diff の有無を説明できるようにする

## 4. Editor Documentation

 - [x] 4.1 README の config schema section を published schema contract に合わせる
 - [x] 4.2 `docs/editor-integration.md` の VS Code schema mapping を更新する
 - [x] 4.3 `docs/editor-integration.md` の Zed schema mapping を更新する
 - [x] 4.4 local schema file を使う fallback 手順を更新する
 - [x] 4.5 editor extension 実装予定は後続 change への参照に留める
 - [x] 4.6 npm package README に `kml help` / `kml --help` / `kml -h` と command help の例を追加する
 - [x] 4.7 PyPI package README に `kml help` / `kml --help` / `kml -h` と command help の例を追加する
 - [x] 4.8 npm `description` と PyPI `summary` を help / version 対応後の CLI 導線が伝わる文言へ更新する

## 5. Verification

 - [x] 5.1 `just fmt-check`
 - [x] 5.2 `just lint`
 - [x] 5.3 `just ast-lint`
 - [x] 5.4 `cargo test --workspace --locked`
 - [x] 5.5 `just dogfood`
 - [x] 5.6 `git diff --check`
 - [x] 5.7 `just schema-check`
 - [x] 5.8 `just VERSION=v0.18.0 release-check`

## Definition of Done

 - [x] 6.1 committed schema file と CLI schema output の一致が機械的に検証される
 - [x] 6.2 stable URL と versioned schema artifact の両方が docs で説明されている
 - [x] 6.3 release gate が schema publication を検証する
 - [x] 6.4 editor integration docs が extension なしの config validation を説明している
 - [x] 6.5 `v0.18.1` の VS Code extension が依存できる schema contract が固定されている
 - [x] 6.6 npm / PyPI の README と package description が help / version 導線を説明している
