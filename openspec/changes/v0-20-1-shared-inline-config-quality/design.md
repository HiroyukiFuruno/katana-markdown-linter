## Context

v0.20.0は共通FixEngineで構文→内容→layout、毎回再check、cycle/上限失敗を実装済み。追加監査は同じHEAD f0cbbccで再現済み（issues/133/audit.md）。この反復を増設せず、その入力になる構文と設定、および正解検証の境界を改善する。

## Goals / Non-Goals

Goals: escaped literal保護、MD049/MD050の重複解析除去、CLI/editor layout設定一致、schema/validation/optionsの共通プロパティ契約、manifest衝突拒否、独立正解と設定/構造行列、v0.20.1公開まで。
Non-Goals: Markdown全仕様の完全再実装、公開range/API破壊、別repo変更、Marketplace新規公開、品質baseline/閾値の緩和。

## Decisions

1. Source adapter境界: inline/scanのescape/parity/searchを共通authorityへ整備する。MD011のreversed-link候補抽出はsource adapterへ移し、ruleはpolicy/diagnostic生成だけを担う。MD049/MD050のdelimiter抽出・intraword・code marker保護を共通emphasis adapterへ集約する。invalid syntaxを直す候補はCommonMark parser eventに存在しないため、既存source byte rangeを保つlexical adapterを使う。全AST導入・各ruleへのbackslash例外追加は採用しない。inline code内部ではescapeが働かない既存契約を維持する。
2. Config authority: rule共通プロパティenabledの定義を一か所に置き、schema生成・値検証・options変換が同じ定義を消費する。既存official rule metadataと共通プロパティの責務を区別し、metadata全体の重複は作らない。
3. Formatter policy: MarkdownFormatter::layout_lint_options_fromを既存の共有policyとして使う。CLIはfile/stdinとも実効設定を検証・変換して渡し、LSPと同じsubset/個別無効化/数値設定を使う。設定探索・severity・構造化config errorと終了codeを維持し、formatterに別FixEngineを作らない。
4. Harness identity: ManifestがID一意性とinput/answer mappingの健全性を起動前に検証する。runner/validatorが別々の定義を持たず、同じManifest契約に従う。IDは一時パスへ安全に写像され、重複/不正入力はkml起動前に失敗する。
5. Expected data: 現行258件を維持し、独立した原文/正解をescape・Unicode・CRLF・入れ子で追加する。同じdefault configの件数を増やすだけでなく、設定差とfile/stdin/library/editorの適切な入口を小さな直交行列で確認する。変異入力→既知の正解、answer自身の不変、3回byte一致、safe診断残0を維持する。style-only修正は補助的な意味保存oracleを用い、意図的な構文修復と混同しない。

## Responsibility inventory

| Concern | Owner | Consumers |
| --- | --- | --- |
| Escape/parity and delimiter source range | shared inline source adapters | MD011/MD049/MD050 and existing inline scanners |
| Rule policy/style/diagnostic candidates | individual rule | shared source/context |
| Common rule config definition | config shared metadata | schema/validator/options conversion |
| Operation settings lifetime | ConfigSession and stateless ConfigLoader authority | CLI preflight/check/fix/fmt file/stdin |
| Layout subset and numeric/enable settings | formatter layout policy | CLI file/stdin, LSP |
| Iteration/edit conflicts | existing FixEngine/FixApplicator | unchanged shared entrypoints |
| Sample identity and fixture mapping | Manifest | runner and answer validator |
| Physical execution filenames | shared ordinal execution_path allocator | input runner and answer validator |
| Independent expected data/matrix | test fixtures/harness | local and existing CI/release gates |

## Risks / Trade-offs

Shared scanner変更は既存emphasis/code/linkへ波及するためRED→GREENと既存rule corpus/upstream/性能を維持する。期待値を現行出力から生成して合わせない。新設定で既存default:false意味を不用意に変えず、enabledの公表済み契約だけを一致させる。cfgやlocaleの全直積は避け、対象を明記した独立ケースへ絞る。意味保存oracle自体の誤判定は対象ruleをstyle-onlyへ限定して避ける。

## Migration Plan

失敗する回帰を先に追加し各境界で実装、親が統合と構造検査を行う。全既存gateと依存監査後にmetadata0.20.1を揃え、Draft review→指摘対応/reply resolve→Ready→CI→merge。既存merge起動Releaseを監視し、release-verify後にspec同期/archiveとIssue/branch整理を実施する。自動runと手動dispatchを重複実行しない。

## Open Questions

なし。実装で新たな互換性問題が再現した場合は根拠を記録して最小差分で判断する。

## Additional confirmed cases and shared safety decisions

追加人間指示: 追加で見つかった問題も個別例外ではなく共通ロジックで修正し、品質基準を下げずにv0.20.1を公開する。

- CommonMark left/right flankingとunderscore固有条件を共有delimiter primitiveへ集約する。Unicode whitespaceはZs+TAB/LF/FF/CR、punctuationはP/S。日本語や個別文字列のif例外は作らない。
- Triple/nested/crossing/link優先で単純scannerの候補範囲が誤ることを再現した。全ての有効なstyle候補で既存SyntaxIndexのpulldown Emphasis/Strong offset rangesをauthorityとして使う。既存code block解析がfull-parserを使った場合は同じ走査でstyle rangesも採取し、未使用時は遅延cacheを使う。backend importをruleへ増やさない。
- Style markerの変換で意味が変わる場合はsafe fixを提示せず、原文と診断を保持する。複雑な変換は共有backendのbefore/after event比較で構造/文字内容を検証する。元と同じ幅・payloadでsource/target markerの内包/隣接がなく、開き専用/閉じ専用の枠を証明できる変換では不要なfull parser走査をしない。完全な任意Markdown conformanceは主張しない。
- CLI fmtに新たに到達する矛盾設定cycleも、既存CliError::ruleのJSON/exit2契約を消費し、file原文不変/他file継続/stdin構造化errorを維持する。
- 正解・設定回帰の一時ディレクトリ名はnonce+atomic連番で分離し、並列実行の設定混入を防ぐ。

## Final source authority refinement

有効なMD049/MD050のspan候補は全てSyntaxIndexの遅延cached backend rangesを消費する。marker個数が足りない行のcheap precheckだけを残し、flat scannerによる独立した構文解釈は削除する。CommonMark source-flanking検査と型付きText/Structuralイベント比較を共有safe replacement境界で使い、単純/複雑を問わず意味保存を確認する。既存code-only fastpathは維持し、ctx生成でparserを先走って起動しない。性能は既存strict1.4で実測して判断する。

新規original064は、旧scannerがnested emphasisを見逃していたためprematureにno-changeをanswerとしていた。consistentという上流MD049仕様と独立した意味比較に基づき、既存258件を保持したまま新規answerのnested style期待値だけを手書きレビューする。

## Semantic comparison cost and safety

全style候補で意味比較を必須にする。source scopeとEvent境界は共有backendの軽量collectorで一度だけ記録し、既存のfull code/fence走査では同時に採取する。所有するEvent signatureは必要時に遅延構築する。元fragmentが元documentの対応subtreeと一致し、元/変更後の外側delimiterが独立する場合だけ局所比較する。fragment用に除くのは一番外のParagraph一対だけで、他のblock構造は消さない。参照・脚注のglobal依存とbracketを持つblock、曖昧なcontainerは全体比較を維持する。blockのbracket有無も一度だけ記録し、候補ごとのparagraph走査を避ける。

独立したtriple frameは128個すべてで10byte scopeの実比較成功を固定し、同一paragraphと無関係な参照定義/脚注が文末にあってもN全文parseへ戻らないことを確認する。一般の全Markdown編集で局所化できるとは主張せず、global依存では保守的な全体検証を残す。性能baselineとstrict上限1.4は変更しない。

## PR134 review: semantic scope boundary

閉じたdelimiter frameだけではreference labelのraw identityを証明できない。意味比較の省略を廃止し、backendのsource scopeとEvent範囲を一度の走査で索引化する。局所fragmentが元documentの対応subtreeと一致し、blockにreference用bracketがなく、元/変更後outer inline frameが外側delimiterと干渉しないことを確認できる場合だけ局所Event比較を使う。***word***のouter Emphasis frameを比較するため、同一paragraphでN個あってもN全文parseをしない。証明できないblock/containerは元の全体比較へ戻す。Reference定義とbracketを含むblock、footnote等のglobal依存は全体比較を維持し、未解決referenceが変更後に新規bindingするケースも拒否する。Unicode casefoldを独自実装しない。backendのprivate module分割はdocument/syntax配下に閉じ、ruleの再parseを許可しない。

## Operation-scoped configuration and execution namespace

ConfigLoaderのstateless APIとsource探索/読込authorityを維持し、CLIの一実行だけConfigSessionを所有する。directory→source、source→prepared validation/optionsをcacheし、check preflightと実処理、file/stdin formatterが同じauthorityを使う。formatterのlayout policy導出はformatter側でsourceごとに一度だけ行う。global cacheは作らず、次のCliWorkflow::runで設定を読み直す。親/子dir優先とexplicit override、既存strict/ignore error契約を維持する。

Logical sample IDは報告と一意性のために使い、物理一時filenameはmanifest順の共通ordinal allocatorで割り当てる。runnerとanswer validatorが同じexecution_pathを消費する。大文字小文字やOS予約名を個別に列挙して補修せず、IDを物理パスから分離することで衝突を避ける。既存ID/mapping検証と270 fixture bytesは維持する。

## Combined edit semantics at the application boundary

個別候補の意味保存と、複数候補を同時適用した意味保存は異なる。FixApplicatorが重複/無効editを除いた共通planに対し、共有FixPolicyでmeaning-preservingと分類されたsafe style editの組合せを検証する。構文修復・明示unsafe等の他editだけを適用した文書をbaselineとし、全planの意味が一致すれば一括適用する。不一致時だけ候補groupを二分してsource順に検証し、baselineとの差を生むstyle editをrejectする。通常の独立128候補は一括検証・適用し、pass数制限のために一候補ずつ逐次実行する設計にはしない。

意味の定義とparser option/normalizationは既存document/syntax backendに閉じたopaque MarkdownSemanticSignatureで共有し、FixApplicatorやruleにparserを複製しない。FixStageとmeaning-preserving分類をprivate policyに集約し、反復engine・単回fix_with_results・CLIが同じ適用境界を通る。適用件数/detail/残effective判定も最終planを基準にする。API形状や意図的な構文修復は維持する。競合時は失敗groupだけ二分し、128候補中1箇所のreference競合でも127候補を適用して全文比較15回以内となることを実parserで固定する。全候補が個別に不正な最悪ケースでは候補数に応じた再parseを要するが、正常groupや通常の一括成功経路は候補数回の全文parseをしない。
