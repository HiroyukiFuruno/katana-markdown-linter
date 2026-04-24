## Design

`kml` の localization は CLI presentation layer に閉じる。
rule evaluation と public library API は canonical diagnostic data を返し、CLI が output format と selected locale に応じて表示文字列を決定する。

## Locale Resolution

Locale は次の順で決定する。

1. CLI option `--local <locale>` または `-l <locale>`
2. OS default locale
3. English fallback

Accepted values は最初の実装で最低限 `en`, `en-US`, `ja`, `ja-JP` を扱う。
比較は case-insensitive とし、region 付き locale は primary language を fallback candidate にできる。

未対応 locale が明示指定された場合は English fallback だけで黙って進めず、CLI error として扱う。
OS default locale が未対応の場合は English fallback にする。

## Message Model

Diagnostic message は text を直接翻訳 key にしない。
各 diagnostic は stable message id と parameter を持つ。

Example categories:

- rule diagnostic: `rule.MD001.heading_increment`
- config error: `config.invalid_root`
- filesystem error: `filesystem.read_failed`
- CLI summary: `summary.statistics`
- fix status: `fix.fixed_count`

parameter には expected/actual heading level、rule id、path、counts など表示に必要な値だけを入れる。

## Output Policy

Text output:

- selected locale で user-facing message を表示する
- rule id、path、line、column は現行 format を維持する
- translation が欠けている message id は English fallback を表示する

JSON output:

- existing fields and exit code semantics を維持する
- automation が壊れないよう、stable message id と params を追加する
- existing `message` field は selected locale の user-facing text として扱う
- JSON consumer が English 固定を必要とする場合は `--local en` を指定する

## Translation Catalog

Translation catalog は repository 内に置き、compile-time に取り込む。
初期対応は English source と Japanese translation の 2 系統にする。

Catalog requirements:

- supported locale ごとに同じ message id set を持つ
- missing translation は test failure にする
- English fallback は runtime fallback として常に利用可能にする
- message params の placeholder 名は locale 間で一致させる

## CLI Scope

`--local` / `-l` は少なくとも `check`, `fix`, `fmt`, `rule`, `config`, `version` の parser で衝突しない global option として扱う。
この change の acceptance target は `check` 系 output だが、parser と shared rendering は future commands へ拡張できる形にする。

## OS Default Locale

OS default locale detection は platform-specific code を CLI boundary に置く。
Unix/macOS では `LC_ALL`, `LC_MESSAGES`, `LANG` を尊重し、library API には locale detection を持ち込まない。
Windows 対応を追加する場合も CLI boundary で吸収し、rule engine には影響させない。

## Compatibility

- `LintResult` の existing public fields は削除しない
- `kml check` の exit code は変えない
- `--output json` の top-level structure は維持する
- dogfood / CI では output locale を明示するか、English OS default であることを前提として drift を避ける

## Non-Goals

- consumer application の localization
- config file の locale directive
- runtime network translation
- locale-specific rule semantics
