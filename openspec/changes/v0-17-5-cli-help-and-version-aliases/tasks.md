# v0.17.5 CLI help and version aliases tasks

## Context

- 2026-05-01: `kml help` が入力ファイルとして扱われ、`help: filesystem error` で失敗することを確認した。
- 2026-05-01: `kml --help` がヘルプ表示ではなく通常の check 処理へ流れることを確認した。
- 2026-05-01: CLI の入口動作に関わるため、修正版は `v0.17.5` patch release とする。
- 2026-05-01: GitHub Release / crates.io / npm / PyPI の最新公開 version は `0.17.4` であり、`v0.17.5` は未使用である。
- 2026-05-01: `homebrew-katana` の `Formula/kml.rb` は `v0.17.1` のままで、`v0.17.3` / `v0.17.4` の formula が登録されていないことを確認した。
- 2026-05-01: npm は `0.17.0` / `0.17.3` / `0.17.4`、PyPI は `0.17.0` / `0.17.1` / `0.17.3` / `0.17.4` を公開済みであることを確認した。
- 2026-05-01: Homebrew の過去 version 登録は、npm / PyPI に合わせて `v0.17.1` 以降の `0.17.1` / `0.17.3` / `0.17.4` を対象とし、npm / PyPI に存在しない `0.17.2` は対象外とする。
- 2026-05-01: Homebrew tap 更新 token は `HOMEBREW_KATANA_GIT_TOKEN` に固定する。

## Tasks

- [x] 1.1 `kml help`、`kml --help`、`kml -h` を global help として扱う
- [x] 1.2 `kml <command> --help`、`kml <command> -h` を command help として扱う
- [x] 1.3 `kml -v` を `kml version` と同じ version alias として扱う
- [x] 1.4 help / version alias の回帰テストを追加する
- [x] 2.1 Homebrew formula generator が `kml.rb` と `kml@X.Y.Z.rb` を生成できるようにする
- [x] 2.2 release workflow が `HOMEBREW_KATANA_GIT_TOKEN` で `homebrew-katana` の current / versioned formula を更新するようにする
- [x] 2.3 `release-verify` が実際の `homebrew-katana` tap formula と生成 formula の差分を検出するようにする
- [x] 2.4 `homebrew-katana` に `kml@0.17.1` / `kml@0.17.3` / `kml@0.17.4` を登録する
- [x] 2.5 versioned formula を `keg_only :versioned_formula` として生成する
- [x] 3.1 Cargo / npm / PyPI / MCP metadata を `0.17.5` に更新する
- [x] 3.2 README / distribution docs / wrapper README / CHANGELOG を `0.17.5` に更新する
- [x] 4.1 `cargo test --test cli_core_contract --locked`
- [x] 4.2 `cargo test cli::args::tests::parses_cli_parity_commands_and_options --locked`
- [x] 4.3 `make homebrew-formula-check VERSION=v0.17.5`
- [x] 4.4 `make ast-lint`
- [x] 4.5 `make dogfood`
- [x] 4.6 `scripts/openspec validate release-readiness --strict`
- [ ] 4.7 `make release-task-ledger-check VERSION=v0.17.5`
- [x] 4.8 `make release-check VERSION=v0.17.5`
- [ ] 5.1 `release/v0.17.5` PR を作成し、CI と signed commit verification を確認する
- [ ] 5.2 merge 後に `make release VERSION=v0.17.5` を実行する
- [ ] 5.3 `make release-verify VERSION=v0.17.5` で GitHub Release / crates.io / npm / PyPI / wrapper launch / Homebrew tap を確認する

## Quality Score

| 項目 | 最大 | 現在 | 根拠 |
| --- | ---: | ---: | --- |
| help behavior | 30 | 30 | global help と command help を実装し、CLI binary 回帰テストを追加済み。 |
| version alias | 20 | 20 | `version` / `--version` / `-V` / `-v` を同じ出力として検証済み。 |
| Homebrew tap | 20 | 20 | workflow / verification / 過去 versioned formula 登録を実装済み。 |
| release metadata | 15 | 15 | Cargo / wrappers / MCP / README / docs / CHANGELOG を `0.17.5` へ更新済み。 |
| verification | 10 | 9 | CLI tests、Homebrew formula check、ast-lint、dogfood、OpenSpec validation、release-check が成功。 |
| release execution | 5 | 0 | PR / merge / publish / post-release verification 待ち。 |
| 合計 | 100 | 94 | task ledger check と公開処理が未完了。 |
