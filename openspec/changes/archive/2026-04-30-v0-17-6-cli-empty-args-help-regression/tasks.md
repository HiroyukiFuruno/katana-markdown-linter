# v0.17.6 CLI entrypoint and config compatibility regression tasks

## Context

- 2026-05-01: `/opt/homebrew/bin/kml --version` は `0.17.5` を返し、`kml help` と `kml -v` は正常に動作することを確認した。
- 2026-05-01: `kml --version` が `0.16.1` を返す環境では、PATH 上の `~/.cargo/bin/kml` が `/opt/homebrew/bin/kml` より先にあることを確認した。
- 2026-05-01: `/opt/homebrew/bin/kml` でも引数なし `kml` は `check` へ流れるため、`v0.17.6` の製品不具合として修正する。
- 2026-05-01: `kml check` でも `.markdownlint.json` の公式 alias と `MD022.lines_below` が config error になるため、dogfood 信頼性に関わる不具合として同じ `v0.17.6` に含める。
- 2026-05-01: config error がある場合に lint まで進む設計は不適切なため、既定は設定修正を促して停止し、明示指定時だけ `--ignore-config-errors` で続行する。
- 2026-05-01: help の多言語化も漏れていたため、`--locale ja` の help を `v0.17.6` に含める。
- 2026-05-01: `make release-check VERSION=v0.17.6` は成功した。

## Tasks

- [x] 1.1 `parse_args(Vec::new())` を `Command::Help(None)` として扱う
- [x] 1.2 引数なし `kml` の CLI binary 回帰テストを追加する
- [x] 1.3 `kml help` / `kml --help` / `kml -h` の既存回帰テストを維持する
- [x] 1.4 `kml version` / `kml --version` / `kml -V` / `kml -v` の既存回帰テストを維持する
- [x] 1.5 公式 markdownlint alias を canonical rule id へ解決する回帰テストを追加する
- [x] 1.6 `MD022.lines_above` / `MD022.lines_below` の `integer|integer[]` 回帰テストを追加する
- [x] 1.7 alias と `integer|integer[]` を含む config で `kml check` が config error を出さない CLI binary 回帰テストを追加する
- [x] 1.8 config error がある場合は lint に進まず、修正または `--ignore-config-errors` を案内する回帰テストを追加する
- [x] 1.9 `--ignore-config-errors` 指定時だけ invalid config entry を無視して lint を続行する回帰テストを追加する
- [x] 1.10 `--locale ja` 指定時の help 日本語表示と `--locale` 説明文の回帰テストを追加する
- [x] 2.1 `parse_args` の空引数を `Command::Help(None)` として扱う
- [x] 2.2 公式 markdownlint alias を config validation / lint options / schema で受け付ける
- [x] 2.3 `MD022.lines_above` / `MD022.lines_below` の数値と数値配列を受け付ける
- [x] 2.4 config error を lint 実行前の blocking gate として扱う
- [x] 2.5 `--ignore-config-errors` で invalid config entry を無視して続行できるようにする
- [x] 2.6 `--locale ja` の help を日本語表示にし、`--locale` が help 文にも効くことを説明する
- [x] 3.1 Cargo / npm / PyPI / MCP metadata を `0.17.6` に更新する
- [x] 3.2 README / docs / CHANGELOG を `0.17.6` に更新する
- [x] 4.1 `cargo test --test cli_core_contract --locked`
- [x] 4.2 `cargo test cli::args::tests::parses_cli_parity_commands_and_options --locked`
- [x] 4.3 `cargo test rules::markdown::config::tests::validate_accepts_official_aliases_and_number_or_array_properties --locked`
- [x] 4.4 `make ast-lint`
- [x] 4.5 `make dogfood`
- [x] 4.6 `make release-check VERSION=v0.17.6`
- [x] 4.7 `make release-task-ledger-check VERSION=v0.17.6`

## Quality Score

| 項目 | 最大 | 現在 | 根拠 |
| --- | ---: | ---: | --- |
| empty args behavior | 15 | 15 | 引数なし `kml` は help として扱い、CLI binary 回帰テストで確認済み。 |
| config compatibility | 20 | 20 | 公式 alias と `integer|integer[]` を config validation / lint options / schema で受け付ける。 |
| config preflight | 15 | 15 | config error は既定で lint 前に停止し、`--ignore-config-errors` の明示時だけ続行する。 |
| help/version regression | 15 | 15 | `help` / `-h` / `--help` と `version` / `-v` / `-V` / `--version` を回帰テストで確認済み。 |
| help i18n | 10 | 10 | `--locale ja` と端末 locale による日本語 help、`--locale` 説明文を回帰テストで確認済み。 |
| release metadata | 10 | 10 | Cargo / npm / PyPI / MCP metadata、README、docs、CHANGELOG を `0.17.6` に更新済み。 |
| verification | 10 | 10 | `make release-check VERSION=v0.17.6` 成功。 |
| ledger | 5 | 5 | `make release-task-ledger-check VERSION=v0.17.6` 対象として台帳を閉じた。 |
| 合計 | 100 | 100 | 完了。 |
