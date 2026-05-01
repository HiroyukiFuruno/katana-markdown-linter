# v0.18.7 CLI unsafe fix help tasks

## Context

- 2026-05-01: `kml fix --unsafe --yes` は README と `kml fix --help` に表示されていた。
- 2026-05-01: `kml check --fix --unsafe --yes` は実装上は動作するが、`kml check --help` に表示されないことを確認した。
- 2026-05-01: `MD036` の unsafe fix で `check --fix --unsafe --yes` が実際に適用されることを手元で確認した。
- 2026-05-01: CLI の安全性に関わる opt-in 導線のため、`v0.18.7` の release blocker として扱う。

## Tasks

- [x] 1.1 `kml check --help` に `--unsafe --yes` を追加する
- [x] 1.2 global help の `--unsafe --yes` 説明を `fix` と `check --fix` の両方に合わせる
- [x] 1.3 日本語 help も同じ内容に更新する
- [x] 1.4 `check --fix --unsafe --yes` が unsafe fix を適用する CLI 回帰テストを追加する
- [x] 1.5 `kml check --help` に unsafe fix opt-in が表示される CLI 回帰テストを追加する
- [x] 1.6 README の CLI usage に `kml check --fix --unsafe --yes README.md` を追加する
- [x] 2.1 Cargo / npm / PyPI / MCP metadata を `0.18.7` に更新する
- [x] 2.2 README / docs / CHANGELOG を `0.18.7` に更新する
- [x] 3.1 `cargo test --test cli_core_contract unsafe`
- [x] 3.2 `cargo test --test cli_core_contract`
- [x] 3.3 `make fmt-check`
- [x] 3.4 `make ast-lint`
- [x] 3.5 `make dogfood`
- [x] 3.6 `make lint`
- [x] 3.7 `cargo test --workspace --locked`
- [x] 3.8 `git diff --check`
- [x] 3.9 `make release-check VERSION=v0.18.7`
- [x] 3.10 `make release-task-ledger-check VERSION=v0.18.7`

## Quality Score

| 項目 | 最大 | 現在 | 根拠 |
| --- | ---: | ---: | --- |
| CLI help | 25 | 25 | `check --help` と global help に unsafe fix opt-in を表示した。 |
| unsafe fix contract | 25 | 25 | `check --fix --unsafe --yes` が `MD036` の unsafe fix を適用する回帰テストを追加した。 |
| documentation | 15 | 15 | README の CLI usage と unsafe fix 説明を `fix` / `check --fix` 両対応に更新した。 |
| release metadata | 15 | 15 | Cargo / wrappers / MCP / README / docs / CHANGELOG を `0.18.7` へ更新した。 |
| verification | 15 | 15 | 狭い CLI 契約、fmt-check、ast-lint、dogfood、lint、workspace test、diff check、release-check が成功。 |
| ledger | 5 | 5 | `make release-task-ledger-check VERSION=v0.18.7` の対象形式へ更新済み。 |
| 合計 | 100 | 100 | release 前 gate 完了。 |
