## Definition of Ready

- [x] current working tree の release/README 差分をこの change に含めるか別 commit に分けるか明確であること
- [x] `--local` / `-l` がユーザー決定として確定していること
- [x] 初期対応 locale が English と Japanese であること
- [x] public API compatibility を壊さない実装方針が維持されていること
- [x] `release-operations-hardening` と write set が競合する場合は先に差分を整理すること

## 1. CLI Locale Option

- [x] 1.1 global option `--local <locale>` と `-l <locale>` を parser に追加する
- [x] 1.2 option 省略時に OS default locale を解決する
- [x] 1.3 OS default locale が未対応または取得不能な場合は English fallback にする
- [x] 1.4 明示指定された未対応 locale は user-facing CLI error として fail する
- [x] 1.5 `--local en`, `--local ja`, `-l en`, `-l ja` の parse tests を追加する

## 2. Message Catalog

- [x] 2.1 English catalog と Japanese catalog を追加する
- [x] 2.2 catalog key と placeholder 名が locale 間で一致することを test で固定する
- [x] 2.3 translation fallback が English に戻ることを unit test で固定する
- [x] 2.4 rule diagnostic、config error、filesystem error、summary、fix status の key を定義する

## 3. Diagnostic Message Refactor

- [x] 3.1 rule diagnostics に stable message id と params を追加する
- [x] 3.2 existing `LintResult.message` は互換 field として残す
- [x] 3.3 MD001 の expected/actual など dynamic message を params 化する
- [x] 3.4 config validation errors を message id と params で表現できるようにする
- [x] 3.5 filesystem/glob/config-file-not-found errors を locale-aware rendering に通す

## 4. CLI Rendering

- [x] 4.1 text `check` output が selected locale の message を表示する
- [x] 4.2 `check --output json` が existing structure を維持しつつ stable message id と params を含む
- [x] 4.3 JSON `message` が selected locale の user-facing text になることを test で固定する
- [x] 4.4 `--local en` を指定すると OS locale に関係なく English output になることを test で固定する
- [x] 4.5 `--local ja` を指定すると representative diagnostics が Japanese output になることを test で固定する

## 5. Documentation And Gates

- [x] 5.1 README に `--local` / `-l` と OS default fallback policy を記載する
- [x] 5.2 CLI usage examples に English 固定と Japanese 指定の例を追加する
- [x] 5.3 dogfood / CI が locale drift しないよう必要なら `--local en` を指定する
- [x] 5.4 release notes または quality gate docs に localization catalog の検証方針を追記する

## Verification

- [x] `cargo fmt --all -- --check` が成功する
- [x] `cargo test --workspace --locked` が成功する
- [x] `kml check --local en <fixture>` が English diagnostics を出す
- [x] `kml check --local ja <fixture>` が Japanese diagnostics を出す
- [x] `kml check -l ja --output json <fixture>` が localized message と stable message id を含む
- [x] `make dogfood` が成功する
- [x] `git diff --check` が成功する

## Definition of Done

- [x] `kml check` の user-facing diagnostics が `--local` / `-l` で切り替え可能であること
- [x] option 省略時に OS default locale が使用されること
- [x] unsupported explicit locale は明確に fail すること
- [x] OS default が unsupported の場合は English fallback になること
- [x] English と Japanese catalog が同じ message id set を持つこと
- [x] public library API の existing fields と CLI exit code が維持されていること
