## Goal and Definition of Done

最新ユーザー指示: 再監査の4点を共通化・境界設計で改善し、正解データと回帰テストで検証し、リリースまで完了する。自然な次patch v0.20.1を使用（release target sanity check成功）。Issue133、対象repoのみ。新worktree・通常stash・master直接編集・品質基準低下は禁止。実装からDraft review/指摘対応/reply resolve/Ready/CI/merge/公開検証/整理まで継続する。

## 1. Design and reproduction

- [x] 1.1 再監査の再現例と責務inventory、DoD、仕様を固定する。
- [x] 1.2 逃しやすいescape/設定/manifest問題を回帰のREDとして再現する。

## 2. Implementation

- [x] 2.1 shared escape/inline source adapterとMD011/MD049/MD050移行を実装する。
- [x] 2.2 共通ruleプロパティのschema/validation/options契約とCLI/editor layout設定適用を統一する。
- [x] 2.3 manifest identity/mappingをkml起動前に検証し、コピー衝突を拒否する。
- [x] 2.4 独立正解データ・設定/構造行列・3回fix・style意味保存の検証を追加する。
- [x] 2.5 共有境界の再発防止検査を追加し、公開資料とmetadata0.20.1を同期する。
- [x] 2.6 Rust/editor/Actionの直接・推移依存とlockfileを監査し、最新互換版を評価する。

- [x] 2.7 追加のflanking/nested style誤修正とformatter cycle error契約を共有処理で修正し、意味保存回帰を通す。

- [x] 2.8 PR134のreference binding破壊P1とnested候補の二乗解析P2を共有semantic scopeで修正し、RED/GREEN/scaleを確認する。

- [x] 2.9 logical IDと物理一時filenameを共通ordinal allocatorで分離し、case-fold上書きの実CLI RED/GREENと既存corpus保持を確認する。
- [x] 2.10 実行内ConfigSessionでsource探索/parse/validation/optionsを共有し、別実行のfreshness・階層優先・error契約を維持して性能基準を通す。

- [x] 2.11 個別safe editの組合せによるreference binding変更を共通修正plan境界で防ぎ、複数rule/単回API/CLIの独立正解回帰を追加する。

## 3. Verification

- [x] 3.1 狭い回帰・正解行列・全既存tests/AST/lint/schema/dogfood/upstream/public confidenceを通す。
- [x] 3.2 coverageとstrict性能を既存baseline/閾値を維持して通す。
- [x] 3.3 完全release-check、配布smoke、自己reviewを通す。

## 4. Review and publication

- [x] 4.1 署名付きsingle commitのDraft PRで@codex reviewと自己レビューを実施する。
- [x] 4.2 指摘の修正・各threadへのreply/resolve、P0/P1ゼロを確認する。
- [x] 4.3 Ready化しrequired checks/署名を確認してmergeする。
- [x] 4.4 merge起動の既存Release workflowを回収し、release-target state再生成後release-verifyで公開物を確認する。


## User Review Phase

- [/] 4点を局所パッチの寄せ集めにせず、構文・設定・検証データの責務境界で改善する。
- [/] 正解一致と回帰で誤修正を検出し、3回の安定性だけを正しさの根拠にしない。

## Evidence

- 初期master f0cbbcc084007758a4a70d8f423147e58996b3f0 clean、stash0、worktree1、Open Issue/Open PR0。
- 追跡Issue: <https://github.com/HiroyukiFuruno/katana-markdown-linter/issues/133>
- 初期再現: issues/133/audit.md、target/quality-followup/repro/*.json。

## 品質評価スコア

| 項目 | 最大 | 現在 |
| --- | --- | --- |
| 構文と設定契約 | 30 | 30 |
| 検証データと回帰 | 30 | 30 |
| 品質と配布検証 | 30 | 30 |
| レビューと整理 | 10 | 10 |
| 合計 | 100 | 100 |

- RED→GREEN: target/v0201-inline-red.log, config-red.log, harness-red.log。追加inline9/既存emphasis6/spec17、formatter設定4、manifest5とquality matrix成功。
- 正解270件 (public200/original70)、全pass exit0、mismatch/answer診断/answer修正/残safe fix全0。既存258件のmanifest record不変。
- 型定義・Action更新とmetadata0.20.1同期、schema-check成功。全既存gate/strict性能/coverageはこれから統合実行。

- 統合検査でtarget全体消失を実確認（source差分は保持、free40GiB）。削除元未特定。直近checkはexit101、消失したredirectログから理由回収不可のため成功扱いしない。以降の検査ログはCargo cache外のtmp/v0201-evidenceへ保存し、全gateを再回収する。

- tmp/v0201-evidence/baseline-red.jsonで公開済みv0.20.0実CLIの誤修正/設定無視と旧Manifestの重複受理を再現し、消失したRED証跡を再生成。
- 最新upstream main比較成功: tmp/v0201-evidence/upstream-drift.log。実VSCode1.75検査成功: tmp/v0201-evidence/vscode.log。統合checkは全tests通過後に新規台帳bare URLのdogfood1件で停止し、表記修正して再実行。

- 追加指示を反映: flankingとnested marker/安全変換、fmt JSON errorをdesign/specへ追記。共通logicの追加修正と実CLI/意味比較を進行中。

- 追加回帰: inline22/emphasis6/spec17成功、formatter設定/error6成功、matrix7ケースのlibrary/file/stdin各3pass成功。新規original064のnested style期待値は上流consistent仕様と独立Event比較で手書き訂正（既存258維持）。270全件diagnostic/mismatch/safe残0を再確認。性能実測はtmp/v0201-evidence/perf-strict.logへ進行中。

- 最終統合review: 必須matrix7行の保持契約を追加。marker枠の高速化はsource/target両marker内包/隣接なしの場合だけ適用。prefix/payload/suffixの1372組合せを実fix+独立Event比較/再fix byte一致で検証（成功）。links scannerのescape判定は構造marker時のみ実行し、長いbackslash runの二乗走査を回避。

- 最終gate: release-check exit0、just check exit0、coverage798<=881、strict性能20/20 (30iterations×7samples, baseline/1.4 unchanged)、正解270全一致・診断/変更/残safe0、matrix7・組合せ1372成功。durable証跡 issues/133/verification-summary.jsonとperf-final.json。256候補scale中央値40.048ms→4.304ms（同条件3sample、参考測定）。

- 追加のGit改行変換問題: fixture全体のbyte保存属性、matrix改行metadata/実byte検査を追加。staged CRLFとworking一致、既存258 record/516file変更0を確認。追加後release-check再実行exit0。

- PR134 exact8342e13 review: P1 reference label binding破壊、P2 nested候補N全文再parse。両方採用。全CIは元HEADで成功済みだが、新source変更後の全gate再回収までは検証taskを未完了へ戻す。既存1372combination保持、reference negative/positiveと8th matrix row追加。

- P1/P2修正後release-check/checkはexit0、coverage806<=881、270 answers診断/変更/残safe0、matrix8と1372+reference回帰、128同一paragraph全候補の10byte scope実比較Some(true)を確認。strict再計測は環境変動時6件超過、cache分離後1件(cli_fix)1.47。sourceで同Dir設定の探索/load/変換重複を特定し2.10で解消中。baseline/閾値は変更しない。
- 2.9実再現: Case-ID/case-idがこのFSで同一fileを上書き。旧HEAD scripts+実CLIはmismatch1/answer変化1/exit1、新allocator同data/CLIは2件全一致3pass/answer変化0/exit0。Rust regressionは最終統合で回収。
- Target cache再消失を確認（削除元不明、source/tmp証跡は保持）。以後CARGO_TARGET_DIRをtmp/v0201-bench-targetへ分離。既存gateのhardcoded target path互換用local symlinkはCargo clean実probeで実cache保持を確認。全ログはtmp/v0201-evidence。

- 最新全gate: isolated cacheでrelease-check/final-check両exit0、coverage806<=881、casefold実CLI契約含む全tests/dogfood/配布smoke成功。strictは慣らし5回・各30iterations×9samplesへ精度を上げ、既存baseline/1.4を維持して20/20成功（cli_check5.764ms/fix35.789ms/fmt34.771ms）。旧測定の失敗もtmp証跡に保持。install-action v2.88.0が最新であることを公開前に再確認。

- Ready後の追加自動reviewでbatch safety P2を検出。PR134をDraftへ戻し、2.11と全品質gateを再実施する。

- 2.11最終gate: batch contract6件/実parser128候補127保持・全文比較15回以内成功。release-check再実行exit0、just check exit0、coverage804<=881、strict性能20/20(同baseline/1.4、30iterations×9samples/warmup5)。旧CLIは全style変換で参照を活性化しexit0、新共通planは独立正解と一致し未修正診断exit1を保持。issues/133/batch-semantics-evidence.json。

## Outer delivery lifecycle

4.5「spec同期/archive、Issueクローズ、branch-hygiene/master clean/stash0/worktree確認」は、完了changeの外側の配信作業としてIssue133のFinal delivery ownershipとsession台帳のv0201-cleanupへ移管した。条件を削除・免除せず、整理PRのreview/CI/mergeとIssue Close、最終master cleanまでrunningで追跡する。自分自身をarchiveする前にarchive完了を偽ってチェックする循環を避け、実装changeと配信controllerの責務を分離する。

このchangeの100/100は、実装・独立正解/回帰・既存品質gate・レビュー・公開検証とsource release branch整理の証跡に基づく。全体の終了条件は、外側の配信作業を終えるまで未完了のまま保持する。Source release/v0.20.1はmerge/delete済み、masterはoriginとahead0/behind0でclean、stash0/worktree1。現在のrelease/cleanup-v0.20.1は仕様同期/証跡を正式履歴へ取り込むため保持する。

- Source PR134: signed/Verified f250cbc393299b8fb9f8338a553da9fe5b394463、merge a84d77a608b0de354f765a3edfa4728fd9a0068c。3thread reply/resolve済み、P0/P1ゼロ。最終headのmanual再依頼とReady-trigger reviewはいずれもcompleted/no findings。最初のmanual実行は60分以上結果がなく、成功と扱わず同条件で1回再依頼して完了結果を回収した。
- Required CI40831150182/preflight40831150237は成功。自動Release40838310577は全8job成功。公開前後にrelease-target stateを再生成し、published-release-verify.logはexit0。
- 公開URL: <https://github.com/HiroyukiFuruno/katana-markdown-linter/releases/tag/v0.20.1>。Verified tag、GitHub assets、crates.io/npm/PyPI0.20.1、実CLI/MCPとwrappers、VSIX/Zed assets、Homebrewを確認。Marketplaceへの新規公開は既存の別計画どおりdeferred。
