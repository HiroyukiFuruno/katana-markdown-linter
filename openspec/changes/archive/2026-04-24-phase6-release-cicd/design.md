## Context

KatanA 本体は release workflow で version 抽出、品質確認、artifact 作成、GitHub Release 作成をほぼ自動化している。
この repository は crate / CLI 配布が主目的であり、desktop artifact は不要だが、version source、quality gate、GitHub Release、crates.io publish の一貫性は必要である。

## Goals / Non-Goals

**Goals:**

- GitHub tag / release を CI/CD で作成できるようにする
- Cargo version と release version の不一致を release 前に止める
- `cargo publish --dry-run` と `cargo install --path` を release gate に含める
- crates.io publish は secret がある場合にだけ実行する
- release notes を GitHub Release に含める

**Non-Goals:**

- Homebrew / winget など外部 package manager 連携
- multi-platform binary artifact 配布
- KatanA desktop workflow の完全移植

## Decisions

### 1. Cargo.toml version を source of truth にする

`Cargo.toml` の `package.version` を release version の正とする。
workflow input / tag version と Cargo version が一致しない場合は release を停止する。

### 2. Release workflow は manual dispatch と tag push の両方を扱う

`workflow_dispatch` は `version` input から `vX.Y.Z` を作成する。
`push.tags: v*` は既存 tag を検証して GitHub Release を作成または更新する。

### 3. crates.io publish は opt-in にする

`workflow_dispatch` で `publish_crate` input が true の場合だけ publish する。
tag push は GitHub Release までに限定し、crates.io publish は不可逆なため manual opt-in に寄せる。

### 4. GitHub Release は crate package artifact と checksum を添付する

binary artifact は phase6 scope から外し、`cargo package` が生成する `.crate` と `sha256` を GitHub Release に添付する。

### 5. 品質ゲートは release workflow 内でも実行する

push 後の GitHub 品質ゲートに依存しきらず、release workflow 内で `fmt`、`test`、`clippy`、upstream drift、package dry-run、install smoke test を再実行する。

## Risks / Trade-offs

- release workflow 内で upstream `markdownlint` default branch を clone するため、upstream 側の一時的な障害で release が止まる
- crates.io publish は取り消せないため、manual dispatch の `publish_crate` は明示 opt-in にする
- GitHub Release は作成できても crates.io publish が失敗した場合、runbook に従って version bump または release の修正が必要になる

## Migration Plan

1. release workflow を追加する
2. release helper script を追加する
3. `CHANGELOG.md` と release runbook を更新する
4. CI/CD 上で release gate を実行できることを local で検証する
