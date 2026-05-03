# Config Schema Publication Design

## Context

現在の schema は `src/config/schema.rs` から生成され、`kml config schema` と
`schema/markdownlint.schema.json` によって確認できる。
README と `docs/editor-integration.md` は stable schema ID として
`https://schemas.katana.tools/kml/markdownlint.schema.json` を示している。

ただし、release 時に committed schema と CLI 出力の一致を保証する gate が弱く、
versioned artifact と compatibility policy も明文化されていない。

## Goals / Non-Goals

**Goals:**

- `schema/markdownlint.schema.json` を published schema の canonical file として扱う
- `kml config schema --output json` と canonical file の byte-for-byte または semantic equality を検証する
- release asset または stable hosting に versioned schema artifact を含める
- schema compatibility policy を docs と tests に固定する
- editor extension が schema URL と local schema file のどちらにも依存できるようにする

**Non-Goals:**

- Markdown rule の追加や safe fix の変更
- VS Code / Zed extension の実装
- 外部 website repository の大規模改修
- markdownlint upstream と完全同一の schema 表現に寄せること
- schema だけで全 config error の line / column precision を保証すること

## Decisions

### D-1: canonical schema は repository 内の generated file にする

`schema/markdownlint.schema.json` を canonical schema file とする。
`kml config schema --output json` は同じ JSON 構造を出力しなければならない。
差分がある場合は、rule metadata 変更に伴う schema 更新漏れとして release gate で失敗させる。

### D-2: stable ID と versioned artifact を分ける

`$id` は stable URL のまま維持する。
release では versioned artifact を別名で添付し、editor docs は stable URL と
release-pinned URL の両方を説明する。

例:

- stable: `https://schemas.katana.tools/kml/markdownlint.schema.json`
- pinned: `markdownlint.schema.vX.Y.Z.json`

### D-3: compatibility は additive-first にする

既存 rule property の型、enum、default、description key を無断で破壊しない。
rule 追加や property 追加は additive change として許容する。
breaking schema change が必要な場合は、OpenSpec change で明示し、docs に migration note を残す。

### D-4: editor validation docs は schema の消費方法だけを扱う

`docs/editor-integration.md` は `.markdownlint.json` / `.markdownlint.jsonc` の
schema mapping を説明する。
VS Code / Zed extension の起動や marketplace packaging は後続 change に送る。

### D-5: release gate は local と CI で同じ script を使う

schema generation / comparison / validation は script 化し、
`just schema-check` と release workflow の両方から呼ぶ。
AST lint には、release gate から schema check が外れないことを確認する guard を追加する。

## Risks / Trade-offs

- stable URL の hosting が外部 repository に依存する
  - この change では versioned release artifact も作り、外部 hosting 不備でも pinned schema を参照できるようにする
- schema diff が property order だけで失敗する
  - comparison は stable serializer か semantic equality を使う
- rule metadata の typo が schema と docs の両方へ広がる
  - fixture-backed regression と `kml rule` metadata との整合性を確認する
- editor ごとの schema association 設定が違う
  - extension 実装ではなく、schema file と URL contract だけをここで固定する

## Migration Plan

1. schema generation check script を追加する
2. committed schema file と CLI 出力の一致テストを追加する
3. schema compatibility fixture を追加する
4. release workflow に schema artifact と schema check を追加する
5. README / editor integration docs / distribution docs を更新する
6. `just VERSION=v0.18.0 release-check` に schema gate を通す

## Open Questions

- stable URL の実体をこの repository の GitHub Pages で持つか、別 site で持つかは implementation 開始時に確認する
- versioned artifact 名は release asset 一覧の既存 naming と合わせて最終決定する
