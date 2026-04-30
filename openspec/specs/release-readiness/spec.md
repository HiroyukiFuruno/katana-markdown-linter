## Purpose

Release readiness defines the package metadata, validation gates, and install contract needed before publishing.

## Requirements

### Requirement: package metadata SHALL satisfy crates.io publishing requirements

システムは、crates.io へ publish できる package metadata を満たさなければならない（SHALL）。

#### Scenario: package を検証する

- **WHEN** developer が publish 前の package を作る
- **THEN** system は `license`、`readme`、`repository`、`description`、`keywords`、`categories` の metadata を含む
- **THEN** system は publish に不要なファイルを package に含めない

### Requirement: release validation SHALL run before publish

システムは、publish 前に package validation を実行しなければならない（SHALL）。

#### Scenario: dry-run を通す

- **WHEN** release pipeline が動く
- **THEN** system は `cargo package` か `cargo publish --dry-run` を実行する
- **THEN** system は失敗時に publish を止める

### Requirement: installed binary SHALL be usable with cargo install

システムは、ユーザーが `cargo install` で binary を導入できるようにしなければならない（SHALL）。

#### Scenario: binary を導入する

- **WHEN** user が crates.io から package を install する
- **THEN** system は `kml` executable target を提供する
- **THEN** user は PATH 上の `kml` コマンドとして呼び出せる

### Requirement: repository SHALL define quality gate responsibilities

repositoryは、localとCIで実行するquality gateの責務を明確に定義しなければならない（SHALL）。

#### Scenario: local gate を実行する

- **WHEN** developer が `make lint` を実行する
- **THEN** system は Clippy zero warning を検証する
- **WHEN** developer が `make ast-lint` を実行する
- **THEN** system は repository 固有の不変条件を検証する

### Requirement: repository SHALL protect release-critical invariants with AST lint

repositoryは、release と rule 互換性に関わる不変条件を AST lint で保護しなければならない（SHALL）。

#### Scenario: release-critical invariant を検証する

- **WHEN** developer が `make ast-lint` を実行する
- **THEN** system は fixture coverage、upstream drift、CLI traversal、signed release tag workflow を検証する
- **THEN** system は破壊的変更または未反映変更を失敗として報告する

### Requirement: v0.12.3 release readiness SHALL include UT and IT expansion evidence

`v0.12.3` の公開準備（release readiness）は、単体テスト（UT）と結合テスト（IT）の拡充結果を含まなければならない（SHALL）。

#### Scenario: release 前に test evidence を確認する

- **WHEN** developer が `v0.12.3` の公開確認（release check）を実行する
- **THEN** system はルール単位の単体テスト（rule-local unit tests）と文書単位の結合テスト（document-level integration tests）の追加範囲を tasks に記録している
- **THEN** system は OS 固有の path / shell / binary suffix 差分が既存 CI で検証されていることを確認する
- **THEN** system は test helper の都合だけで production behavior を変更していないことを確認する

### Requirement: release gates SHALL keep performance checks informative but stable

公開ゲート（release gate）は速度確認（performance check）を有用な情報として扱い、環境依存の不安定な閾値に依存してはならない（SHALL NOT）。

#### Scenario: performance gate を実行する

- **WHEN** developer が `make perf-check` を実行する
- **THEN** system は基準値比較（baseline comparison）を表示する
- **THEN** system は required benchmark case と report schema を検証する
- **THEN** system は CI の時計時間ゆらぎ（wall-clock fluctuation）だけで release を止めない

### Requirement: v0.12.4 release readiness SHALL confirm core commands before distribution work resumes

`v0.12.4` の release readiness は、配布展開へ戻る前に中核コマンドの品質 gate を確認しなければならない（SHALL）。

#### Scenario: release 前 gate を確認する

- **WHEN** developer が `v0.12.4` release を準備する
- **THEN** system は `check` の no-write contract を確認する
- **THEN** system は `fix` と `check --fix` の safe fix contract を確認する
- **THEN** system は `fmt` の formatter contract と冪等性を確認する
- **THEN** system は performance evidence が command semantics を弱めていないことを確認する
- **THEN** system は release 後に `v0.13.0` 以降の配布展開へ戻れる状態にする

### Requirement: stable release readiness SHALL be score-based

安定版 readiness は、数値化された stable score と hard blocker で評価されなければならない（SHALL）。

#### Scenario: stable score を評価する

- **WHEN** developer が `v0.12.8` の安定版判定を行う
- **THEN** system は 100 点満点の score を算出する
- **THEN** system は Precision correctness、Safe command behavior、Performance stability、Release reproducibility、Evidence quality の category score を記録する
- **THEN** system は hard blocker の有無を記録する
- **THEN** score が 90 点以上、hard blocker が 0 件の場合だけ stable candidate として扱う

### Requirement: stable release SHALL require user acceptance

安定版 release は、最終的なユーザー受け入れ判断を必要としなければならない（SHALL）。

#### Scenario: ユーザー受け入れを確認する

- **WHEN** system が stable score、hard blocker、known limitation、verification result を提示する
- **THEN** user は安定版として受け入れるか判断する
- **THEN** user が受け入れた場合だけ `v0.13.0` の DoR を満たす
- **THEN** user が受け入れない場合、追加の `v0.12.x` hardening scope を定義する

### Requirement: distribution work SHALL wait for stable acceptance

配布展開は、安定版受け入れが完了するまで進めてはならない（SHALL NOT）。

#### Scenario: v0.13.0 に進む

- **WHEN** developer が `v0.13.0` の配布計画に着手する
- **THEN** system は `v0.12.8` の stable score が 90 点以上であることを確認する
- **THEN** system は hard blocker が 0 件であることを確認する
- **THEN** system は user acceptance が記録されていることを確認する

### Requirement: distribution expansion SHALL wait for v0.12.9 public confidence completion

配布展開は、`v0.12.9` の public confidence hardening が完了するまで進めてはならない（SHALL NOT）。

#### Scenario: v0.13.0 に進む

- **WHEN** developer が `v0.13.0` の配布計画に着手する
- **THEN** system は `v0.12.8` の stable score が 90 点以上であることを確認する
- **THEN** system は `v0.12.8` の hard blocker が 0 件であることを確認する
- **THEN** system は `v0.12.8` の user acceptance が記録されていることを確認する
- **THEN** system は `v0.12.9` の public confidence score が 90 点以上であることを確認する
- **THEN** system は `v0.12.9` の release-blocking issue が 0 件であることを確認する

### Requirement: public confidence score SHALL be recorded before release

`v0.12.9` release 前に、public confidence score が記録されなければならない（SHALL）。

#### Scenario: public confidence score を評価する

- **WHEN** developer が `v0.12.9` の release 前確認を行う
- **THEN** system は external corpus confidence、precision regression、command convergence、performance stability、release reproducibility を採点する
- **THEN** system は hard blocker の有無を記録する
- **THEN** system は known limitation と non-blocking follow-up を記録する

### Requirement: システムは `v0.12.11` で品質スコアを算出する

`v0.12.11` の品質判定は、`v0.12.9` と同一方針で 100 点満点基準の数値として残さなければならない（SHALL）。

#### Scenario: `v0.12.11` の quality score を算出する

- **WHEN** `v0.12.11` の validation が完了する
- **THEN** system は `public-confidence-score.json` を出力する
- **THEN** score は `total` と `threshold` を含み、`status` / `version` / `technical_hard_blockers` を明示する
- **THEN** score は 100 点満点基準でカテゴリ別採点を保存すること

### Requirement: システムは `v0.12.9` と同一カテゴリ構成を維持する

採点カテゴリは下記 5 軸を維持し、`v0.12.9` 時点の評価フレームを引き継がなければならない（SHALL）。

- `External corpus confidence`
- `Precision regression`
- `Command convergence`
- `Performance stability`
- `Release reproducibility`

配点は `v0.12.9` と同じく 30 / 25 / 20 / 15 / 10、合計 100 点とする。

#### Scenario: カテゴリ採点を再現可能に残す

- **WHEN** quality score を計算する
- **THEN** 各カテゴリは `score` / `max` / `evidence` を持つ
- **THEN** evidence には `make public-confidence`、`make perf-check`、収束実行、`release-check` の結果を含める
- **THEN** `v0.12.9` の配点（30 / 25 / 20 / 15 / 10）を採用し、`v0.12.11` でもカテゴリ合計が 100 点になる

### Requirement: システムは release 進行条件として score を機械的に利用する

`score` と `technical_hard_blockers` は、release 前の進行条件として扱わなければならない（SHALL）。

#### Scenario: 合否判定を記録する

- **WHEN** `public-confidence-score.json` が更新される
- **THEN** `score.total >= 90` かつ `technical_hard_blockers = []` を満たすことを release 判断用タスクに反映する
- **THEN** 残存する `known_limitations`、`non_blocking_follow_ups`、`verification` を score レポートに残す
- **THEN** `release` 前提時点で未分類 high-risk finding を残さない

### Requirement: v0.12.20 release readiness SHALL prove performance changes preserve correctness

`v0.12.20` の release 前確認は、性能変更が正しさを壊していないことを証明しなければならない（SHALL）。

#### Scenario: performance release gate を通す

- **WHEN** developer が `v0.12.20` の release check を行う
- **THEN** system は `make perf-check` または `make perf-check-strict` の結果を記録する
- **THEN** system は `make test`、`make ast-lint`、`make dogfood` の結果を記録する
- **THEN** system は説明不能な重大性能退行がある場合、`v0.12.21` へ進まない

### Requirement: v0.12.20 SHALL prepare the handoff to KatanA feedback sweep

`v0.12.20` は、次の `v0.12.21` で KatanA 側ドキュメントを使った feedback sweep を行える状態で完了しなければならない（SHALL）。

#### Scenario: v0.12.21 へ引き継ぐ

- **WHEN** `v0.12.20` の validation が完了する
- **THEN** system は性能上の known limitation と non-blocking follow-up を tasks または release evidence に残す
- **THEN** system は KatanA feedback sweep で確認すべき観点を `v0.12.21` の tasks に引き継ぐ
- **THEN** system は未分類の performance blocker を残したまま closeout に進まない

### Requirement: v0.12.21 closeout SHALL block v0.13.0 when release-blocking feedback remains

`v0.12.21` closeout は、release-blocking feedback が残る場合に `v0.13.0` へ進んではならない（SHALL NOT）。

#### Scenario: closeout 判定を行う

- **WHEN** developer が `v0.12.21` の release 前確認を行う
- **THEN** system は KatanA feedback sweep の release-blocking issue が 0 件であることを確認する
- **THEN** system は by-design 宣言対象が README と fixture matrix に反映されていることを確認する
- **THEN** system は未分類の high-risk finding を残さない

### Requirement: KatanA false positives and bad fixes SHALL block release until fixed

KatanA feedback sweep で見つかった `check` の誤検知と `fix` の誤修正は、release 前に kml 側で修正されなければならない（SHALL）。

#### Scenario: precision blocker を扱う

- **WHEN** KatanA feedback sweep で false-positive または bad-fix が見つかる
- **THEN** system は該当 pattern を kml repository の regression test に落とす
- **THEN** system は production code を修正し、test 都合だけの挙動変更をしない
- **THEN** system は該当 pattern が再発しないことを確認するまで release readiness を満たさない

### Requirement: v0.12.21 SHALL record follow-up issues separately from release blockers

`v0.12.21` は、後続対応でよい issue と release blocker を混同してはならない（SHALL NOT）。

#### Scenario: follow-up を記録する

- **WHEN** KatanA feedback sweep で non-blocking finding が見つかる
- **THEN** system は後続版で扱う理由を記録する
- **THEN** system は `v0.13.0` の配布計画に影響するものと影響しないものを分ける
- **THEN** system は follow-up を by-design 宣言と混同しない

### Requirement: v0.17.3 release readiness SHALL close the npm package visibility gap

`v0.17.3` の release readiness は、npm package page の README / metadata 不足を release blocker として扱わなければならない（SHALL）。

#### Scenario: v0.17.3 release is prepared

- **WHEN** developer prepares `v0.17.3`
- **THEN** system confirms `wrappers/npm/README.md` exists and is included in the npm tarball
- **AND** system confirms npm package metadata has search and support fields
- **AND** system confirms trusted publisher configuration is present for `HiroyukiFuruno/katana-markdown-linter` and `release.yml`
- **AND** system keeps the npm package as a thin wrapper with no independent lint logic

### Requirement: v0.17.3 release readiness SHALL close the PyPI package visibility gap

`v0.17.3` の release readiness は、PyPI project page の README / metadata 不足を release blocker として扱わなければならない（SHALL）。

#### Scenario: v0.17.3 release is prepared

- **WHEN** developer prepares `v0.17.3`
- **THEN** system confirms `wrappers/python/README.md` explains install, usage, supported platforms, and wrapper contract
- **AND** system confirms PyPI metadata has search and support fields
- **AND** system confirms the built wheel metadata contains the Markdown long description
- **AND** system keeps the PyPI package as a thin wrapper with no independent lint logic

### Requirement: v0.17.3 post-release verification SHALL prove npm publication

`v0.17.3` の公開後検証は、npm registry と npm wrapper 起動を確認しなければならない（SHALL）。

#### Scenario: v0.17.3 npm publication is verified

- **WHEN** npm wrapper publication for `v0.17.3` completes
- **THEN** system verifies npm contains `katana-markdown-linter` version `0.17.3`
- **AND** system runs `npx --yes katana-markdown-linter@0.17.3 --version`
- **AND** command output is `0.17.3`
- **AND** verification result is recorded before `v0.18.0` work resumes

### Requirement: Release readiness SHALL include document answer fix evaluation

Release readiness SHALL include document-level answer fixture evaluation before publishing `v0.16.2`.

#### Scenario: release check runs document answer evaluation

- **WHEN** developer runs `make release-check VERSION=v0.16.2`
- **THEN** system runs the document answer fix evaluation
- **AND** release check fails if any fixed output differs from its answer fixture
- **AND** release check fails if the public corpus has fewer than 200 valid public GitHub samples
- **AND** release check fails if the original corpus has fewer than 50 valid original samples

### Requirement: Release readiness SHALL limit v0.16.2 scope to bugfixes

Release readiness SHALL keep `v0.16.2` focused on bugfixes found by document answer evaluation.

#### Scenario: scope is checked

- **WHEN** `v0.16.2` release preparation is reviewed
- **THEN** system records detected document answer mismatches and their fixes
- **AND** system does not include `v0.17.0` distribution expansion work in the same release
- **AND** system keeps Homebrew, standalone binary expansion, npm wrapper, and PyPI wrapper work deferred to `v0.17.0`

### Requirement: release readiness SHALL verify binary distribution artifacts

release readiness は、Cargo package だけでなく binary distribution artifact を検証しなければならない（SHALL）。

#### Scenario: release check validates binary artifacts

- **WHEN** developer が `make release-check VERSION=vX.Y.Z` を実行する
- **THEN** system は binary archive と checksum を local で作成する
- **AND** system は archive 展開後の `kml --version` を検証する
- **AND** system は小さな Markdown fixture に対して `kml check` を実行する

### Requirement: release readiness SHALL verify Homebrew formula output

release readiness は、Homebrew 定義ファイル（formula）の出力を検証しなければならない（SHALL）。

#### Scenario: release check validates formula

- **WHEN** developer が `make release-check VERSION=vX.Y.Z` を実行する
- **THEN** system は release archive URL と checksum を使って formula を生成する
- **AND** system は formula が `kml --version` を test block に含むことを検証する
- **AND** system は formula が存在しない asset や checksum を参照する場合に失敗する

### Requirement: release readiness SHALL record wrapper publication state

release readiness は、npm / pip wrapper の公開状態を明示しなければならない（SHALL）。

#### Scenario: wrapper publication is deferred

- **WHEN** package ownership または trusted publishing 設定が未確定である
- **THEN** tasks ledger は wrapper publication を deferred として記録する
- **AND** README と docs は wrapper を公式 install 導線として表示しない
- **AND** release note は wrapper source / smoke test の準備状態と publish 状態を分けて説明する

### Requirement: release readiness SHALL update public installation documentation accurately

release readiness は、公開済みの install 導線だけを README と docs に記載しなければならない（SHALL）。

#### Scenario: docs are updated for release

- **WHEN** release preparation updates README or docs
- **THEN** documentation は Cargo、GitHub Release binary、Homebrew、公開済み wrapper の状態を分けて説明する
- **AND** documentation は未公開 wrapper を公式導線として扱わない
- **AND** documentation 変更後に `make ast-lint` が成功する

### Requirement: release readiness SHALL preserve existing crate install contract

release readiness は、Cargo install の既存導入契約を壊してはならない（SHALL NOT）。

#### Scenario: binary distribution is added

- **WHEN** system が standalone binary と Homebrew support を追加する
- **THEN** `cargo install katana-markdown-linter` で `kml` を導入できる状態を維持する
- **AND** existing GitHub Action install-source behavior を維持する
- **AND** binary artifact support のために crate metadata を弱めない

### Requirement: v0.17.4 release readiness SHALL close wrapper stale-cache regression

`v0.17.4` の release readiness は、npm / PyPI wrapper が過去 version の unversioned binary cache を再利用しないことを release blocker として扱わなければならない（SHALL）。

#### Scenario: stale wrapper cache is present

- **WHEN** developer prepares `v0.17.4`
- **THEN** system creates a stale `bin/kml` cache that returns a mismatched version
- **AND** system runs the npm wrapper with `KML_WRAPPER_INSTALL_DIR` pointing at that stale cache
- **AND** system runs the PyPI wrapper with `KML_WRAPPER_INSTALL_DIR` pointing at that stale cache
- **AND** both wrappers return `0.17.4`
- **AND** release check fails before publication if either wrapper returns the stale binary version

### Requirement: release verification SHALL include public package registries

公開後検証（post-release verification）は、GitHub Release と crates.io だけでなく、npm と PyPI の公開状態を確認しなければならない（SHALL）。

#### Scenario: release verification checks registry versions

- **WHEN** developer runs `make release-verify VERSION=vX.Y.Z`
- **THEN** system verifies crates.io contains `katana-markdown-linter` version `X.Y.Z`
- **AND** system verifies npm contains `katana-markdown-linter` version `X.Y.Z`
- **AND** system verifies PyPI contains `katana-markdown-linter` version `X.Y.Z`
- **AND** system fails with a registry-specific error when a version is missing

### Requirement: release verification SHALL execute wrapper launch smoke tests

公開後検証は、公開済み wrapper から `kml` が起動することを確認しなければならない（SHALL）。

#### Scenario: release verification launches wrappers

- **WHEN** developer runs `make release-verify VERSION=vX.Y.Z`
- **THEN** system runs the npm wrapper through `npx --yes katana-markdown-linter@X.Y.Z --version`
- **AND** system runs the PyPI wrapper through `uvx --from katana-markdown-linter==X.Y.Z kml --version`
- **AND** both commands must print `X.Y.Z`

### Requirement: release verification SHALL include Homebrew formula evidence

公開後検証は、Homebrew formula が release artifact と一致していることを確認しなければならない（SHALL）。

#### Scenario: release verification checks formula output

- **WHEN** developer runs `make release-verify VERSION=vX.Y.Z`
- **THEN** system renders or reads the Homebrew formula for `vX.Y.Z`
- **AND** system verifies formula URL values reference the expected release archives
- **AND** system verifies formula checksum values match generated checksum files
- **AND** system verifies formula test block executes `kml --version`

### Requirement: v0.17.6 release readiness SHALL treat empty CLI arguments as help

`v0.17.6` の release readiness は、引数なし `kml` が lint 対象探索へ流れないことを release blocker として扱わなければならない（SHALL）。

#### Scenario: empty CLI args are requested

- **WHEN** developer prepares `v0.17.6`
- **THEN** system runs `kml` without arguments in an empty working directory
- **AND** command exits with code `0`
- **AND** command prints global usage
- **AND** command does not run Markdown file discovery

#### Scenario: existing help and version aliases keep working

- **WHEN** developer prepares `v0.17.6`
- **THEN** system runs `kml help`
- **AND** system runs `kml --help`
- **AND** system runs `kml -h`
- **AND** system runs `kml version`
- **AND** system runs `kml --version`
- **AND** system runs `kml -V`
- **AND** system runs `kml -v`
- **AND** each command exits with code `0`

#### Scenario: Japanese help is requested

- **WHEN** developer prepares `v0.17.6`
- **THEN** system runs `kml --locale ja help`
- **AND** system runs `kml check --help --locale ja`
- **AND** each command exits with code `0`
- **AND** each command prints Japanese usage text
- **AND** each help text explains that `--locale` changes diagnostic and help text language

### Requirement: v0.17.6 release readiness SHALL accept official markdownlint config aliases

`v0.17.6` の release readiness は、公式 markdownlint alias と `integer|integer[]` 型の設定値を config error にしないことを release blocker として扱わなければならない（SHALL）。

#### Scenario: repository-style config uses official aliases

- **WHEN** developer prepares `v0.17.6`
- **THEN** system runs `kml check` with a config containing `first-line-h1`, `first-line-heading`, `no-duplicate-heading`, and `no-inline-html`
- **AND** the config contains `MD022.lines_above` and `MD022.lines_below` as integer or integer array values
- **AND** command does not report `unknown markdownlint rule`
- **AND** command does not report `invalid rule property value`

### Requirement: v0.17.6 release readiness SHALL stop before linting when config is invalid

`v0.17.6` の release readiness は、config error を lint 診断と混ぜず、既定で lint 実行前に停止することを release blocker として扱わなければならない（SHALL）。

#### Scenario: invalid config is used without override

- **WHEN** developer runs `kml check` with invalid config
- **THEN** command exits with code `2`
- **AND** command reports the config error
- **AND** command advises fixing the config or rerunning with `--ignore-config-errors`
- **AND** command does not report file lint diagnostics

#### Scenario: invalid config is explicitly ignored

- **WHEN** developer runs `kml check --ignore-config-errors` with invalid config
- **THEN** command reports the config error
- **AND** command ignores invalid config entries
- **AND** command continues to report file lint diagnostics
