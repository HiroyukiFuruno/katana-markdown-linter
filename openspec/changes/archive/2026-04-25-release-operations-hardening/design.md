## Design

release flow は 4 段階に分ける。

1. `release-check`: local preflight。GitHub Actions の required checks と同じ target / features / lockfile 条件で実行する。
2. `release-tag`: signed annotated tag を作成し、tag が Cargo package version と一致することを検証する。
3. `release-github`: tag push workflow で GitHub Release と crate package artifact を作成する。
4. `release-publish`: workflow_dispatch で crates.io publish を実行する。

## Tag Verification

Release tag は local GPG verification だけでなく、GitHub API が `Verified` と返すことを必須にする。
`release-tag` は GitHub account に紐づく tagger name/email と signing key を使い、tag push 後に GitHub verification state を確認する。
GitHub が `Unverified` と返す場合、GitHub Release と crates.io publish には進まない。

## Retry Policy

- GitHub Release が未作成、crates.io publish 前、かつ failed workflow のみなら、tag を修正して再実行してよい。
- GitHub Release が作成済みなら、tag target は変更しない。追加修正は次 patch version とする。
- crates.io publish 済みなら、同じ version は再 publish しない。追加修正は次 patch version とする。
- retry 前には `gh release view`, `cargo info --registry crates-io`, `git rev-parse tag^{}` を確認する。

## Local/CI Parity

`make lint` は CI と同じ `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` を使う。
今後 release workflow に gate を追加する場合、対応する Makefile target も同じ change で更新する。

## Release Notes

`CHANGELOG.md` を source of truth とする。
過去 release notes が空または changelog と乖離している場合は、release 前に補正する。

## Non-Goals

- 自動 version bump は含めない
- crates.io token 管理方式の変更は含めない
- release branch workflow の導入は含めない
- GitHub organization/repository settings の全面変更は含めない
