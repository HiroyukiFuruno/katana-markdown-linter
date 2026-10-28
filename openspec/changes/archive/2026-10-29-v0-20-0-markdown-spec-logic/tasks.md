## Goal and Definition of Done

最新指示: Markdown仕様追従・既存ロジック改善・Open Issue対応・minor release・GitHub Actions plugin最新化。共通部分の個別実装を避け、責務分離と再利用を設計する。fix順序・反復収束・誤検知/誤修正/余白増殖を正解データで検証する。ボード作成は不要、物理台帳で管理する。

対象はこのrepoのみ。新規worktree、通常作業のstash、master直接編集、品質ゲート低下は禁止。既存merge起動のRelease workflowで公開し、未起動時のみjust releaseを利用する。公開確認・Issue整理・安全なbranch-hygieneまでが完了条件。Marketplace新規公開は独立したv0.21.0計画で保持する。

## 1. Audit and specification

- [x] 1.1 最新公式仕様、repo状態、既存chat、Open Issue/Open PRを照合する。
- [x] 1.2 proposal/design/specs/tasksとDoDを物理ファイルへ固定する。
- [x] 1.3 仕様差分を回帰テストで再現する。

## 2. Implementation

- [x] 2.1 表、見出し、フェンス、複数行コードの共有解析を修正する。
- [x] 2.2 仕様対応資料と継続追従の検証導線を追加する。
- [x] 2.3 GitHub Actionsを最新安定版へ更新し契約を確認する。
- [x] 2.4 直接・推移依存を調査し、最新互換版へ更新する。
- [x] 2.5 v0.20.0配布メタデータ・changelog・既存計画を整合する。

## 3. Validation and release

- [x] 3.1 回帰・全機能テスト・AST lint・upstream golden/drift・dogfood・公開confidence・性能・coverageを検証する。
- [x] 3.2 release-target-check、release-check、自己レビューを完了する。
- [x] 3.3 Draft PR作成、@codex review回収、指摘対応・reply/resolve、P0/P1ゼロを確認する。
- [x] 3.4 Ready化、required CI確認、mergeする。
- [x] 3.5 既存Release workflowとrelease-verifyで全公開物を検証する。
- [x] 3.6 branch-hygiene、master clean、stash0、worktree、Issue残件を確認する。

## User Review Phase

- [/] fix精度向上: 保護領域・空白のみ・複数行・UTF-8/CRLF・冪等性を検証する。
- [/] ちゃんと設計: 共有責務、代替案、移行リスク、安全なfix契約を記録しレビューする。
- [/] 共通部品の個別実装を防止: 共通解析authorityとrule移行、再発防止検査を実装する。
- [/] fix順序・反復: 構文→内容/スタイル→レイアウト、毎回再check、cycle/上限で失敗・原文不変を検証する。
- [/] 正解データ: 独立した期待出力と3回fixのbyte一致、安全fix残0で余白増殖・誤修正を検査する。

## 4. Shared fix pipeline

- [x] 4.1 library/CLI/formatterの反復を共通FixEngineへ移行する。
- [x] 4.2 明示したfix段階順序・競合の決定性・cycle/上限の失敗を実装する。
- [x] 4.3 正解データを仕様/順序/余白/UTF-8/CRLFで拡張し3回の冪等性を検証する。
- [x] 4.4 共有責務が再び重複しない構造検査を追加する。

## 5. Release dependency follow-through

- [x] 5.1 VSCode開発依存をESLint10/TS6/typescript-eslint8/現行test-electronへ移行し、実際のVSCode1.75 hostで既存3 smoke assertionsと拡張検査を通す。
- [x] 5.2 runtime languageclient8とminimum editor^1.75を互換性契約として保持し、TS7はparser peer範囲外として不採用理由を記録する。
- [x] 5.3 旧release手順のbranch/自動公開/レビュー順序を現在のworkflowとAGENTSへ整合し、二重dispatchを防止する。

## Evidence

- CommonMark0.31.2: <https://spec.commonmark.org/0.31.2/>、GFM0.29-gfm: <https://github.github.com/gfm/>。2026-10-29確認。全仕様・任意入力の網羅性は主張しない。
- 開始commit0898b9336b9f0aa36438b6a6620907b397257ecb。開始時Open Issue/Open PR0、master clean、stash0、worktree1。
- Issue: <https://github.com/HiroyukiFuruno/katana-markdown-linter/issues/130>。
- PR: <https://github.com/HiroyukiFuruno/katana-markdown-linter/pull/131>。signed source HEAD a024276c05f9c4dba3727df015a51193e88d1fd7、merge1277353eee9cb5a8903cfc8f405a56ad661b0db6。
- 2件のCodex P2は修正前RED→修正後GREEN、個別reply/resolve済み。escaped backtick runの1文字escapeとMD011の両方向code境界交差を共通adapter/APIで修正。最終HEAD再reviewはcompleted/no findings、自己reviewも未対応P0/P1ゼロ。
- required CI: test matrix40753342786のmacOS/Linux/Windowsとpreflight40753342777が全てpass。
- 構文回帰17、実CLI/library fix回帰4、共有境界契約4。15 ruleを共有context評価へ移行。MD012.maximumの既存無視も回帰で是正。
- 258件(200 public+58 original)の独立answerと3回fixが全件byte一致。answer診断/修正0、安全fix残0。旧answer11件はCommonMark4.5のfence規則に照らし、literal内へ誤挿入していたtext/空行を個別訂正。
- strict性能20/20、30 iterations×7 samples、historical baselineと1.4閾値を維持。Coverage804 uncovered<=baseline881。
- just check、all-features、lint、AST25件、upstream golden/drift、dogfood、public-confidence、完全release-check(dry-run/install/各配布smoke)が成功。
- Rustの13直接依存は最新互換、pulldown-cmark0.13.5を共有adapterに導入。registry依存のみ。taiki-e/install-action v2.87.33を3 workflow7参照へpin更新、他Actionも監査済み。
- editor: 最低対応VSCode1.75で実Extension Host検査成功、npm audit0。Node16互換を保つためMocha/globを除去し同じassertionsをNode標準assertで実行。
- target消失後に最終HEAD用check/release-check/answer/perf証跡を再生成。恒久証跡はissues/130/verification-summary.jsonとdependency-update-evidence.md、ログはtarget/review-fixes-*.log。
- Release workflow40754506735は全job成功。just VERSION=v0.20.0 release-verifyはexit0。GitHub Verified tag、GitHub Release、crates.io/npm/PyPI0.20.0、実CLI/MCP配布物、npm/PyPI wrapper、Homebrew default/versioned formulaを確認。target/final-release-verify.logとissues/130/verification-summary.jsonへ証跡保存。
- masterをfast-forwardし、merge済みsource branch release/v0.20.0を安全に削除。整理用release/cleanup-v0.20.0でspec同期と証跡保存を進める。新worktree/stashなし。

## 品質評価スコア

下記スコアは上記の回帰/品質/公開検証と整理の実測証跡に基づく。仕様追跡はmain specへ同期し、この完了changeをarchiveへ保存する。

| 項目 | 最大 | 現在 |
| --- | --- | --- |
| 構文精度と回帰 | 30 | 30 |
| 互換性と品質検査 | 30 | 30 |
| 配布検証とレビュー | 30 | 30 |
| 仕様追跡と整理 | 10 | 10 |
| 合計 | 100 | 100 |

- Scope cleanup: local master matches origin/master, stash0, worktree1. Source release branch is merged/deleted. Existing remote-only release/v0.19.6 and release/cleanup-v0.19.6 are retained because remote cleanup was not requested. The current release/cleanup-v0.20.0 branch carries spec sync/archive and evidence updates through its own Draft review/CI/merge before final session completion.
