## Context

公開最新版は v0.19.6。2026-10-29 時点で GitHub connector と gh の両方で Open Issue/Open PR は0件。同じ cwd の重複作業セッションなし。master clean、stash 0、既存 worktree は現 checkout のみ。

## Goals / Non-Goals

Goals: 仕様根拠のある構文精度改善、入力に基づく回帰検査、minor release と公開検証。
Non-Goals: Markdown レンダラーの全面再実装、別 repo 変更、Marketplace への新規公開。既存の editor publication 計画は独立して保持する。

## Decisions

- 公開最新版の自然な次 minor v0.20.0 を使う。既存 Marketplace 計画の版番号は未実施計画として後続へ整理する。
- 各ルールの局所修正ではなく共有構文インデックスを改善し、byte range と公開型を保持する。
- 仕様例に基づく専用 regression suite と既存 upstream golden、fix収束性、品質・性能 gate で検証する。
- 未閉じコードスパンの既存 conservative contract は維持し、複数行判定はブロック境界を超えない設計とする。
- Action 更新の編集担当は .github/workflows のみ。main は実装・統合・リリースを担当する。

## Risks / Trade-offs

共有解析の変更が既存ルールへ波及するため、狭い回帰検査を先に失敗させ、既存全検査・coverage・性能を維持する。性能 baseline は不具合を隠す目的では更新しない。

## User feedback: shared semantics, diagnosis and safe fix

検索語: shared syntax authority, no per-rule parser, source-preserving fix.

- アンチパターン: MD010とMD011がそれぞれ backtick を反転してコード領域を推測し、複数backtick・複数行・escapeで異なる修正を作る。
- 採用原則: 共通構文の意味論は DocumentContext の1つの解析インデックスへ集約し、ruleはポリシー判定と修正候補の生成だけを担う。fix適用は既存 FixApplicator を使い、formatter・CLI・LSPへ別エンジンを作らない。
- 候補比較: 全自作parser拡張は依存増がないがCommonMark container/inline境界を再実装する保守費用が高い。comrakは完全ASTと多彩な拡張を備えるが本処理には所有ASTを要しない。pulldown-cmarkはイベントとbyte offsetを返し、共通構文判定を委譲しながら原文と既存公開range型を保持できる。公開レジストリ版・ライセンス・回帰・性能の実測を確認して採否を決める。
- 実装候補: 1文書1回のparser走査からfenced blockとclosed inline-code rangeを抽出し、既存原文インデックスへ写像する。未閉じinline codeは既存の保守的なfix抑制として明示的に補足する。共有の表・ATX lexical parserは原文range取得のadapterとして扱い、同じ意味論をruleへ再実装しない。
- 修正契約: コード内のMD010/MD011/MD034は同じrange包含判定を使う。MD038は複数行rangeでpanicしない。空白だけのcodeを空文字へ潰さず、内部改行とUTF-8を保持する。`fix(fix(x)) == fix(x)` を実際のAPI/CLIで検証する。
- FixApplicatorのUTF-8境界、重複・競合編集、CRLFとEOFの契約を確認し、再現できた欠陥だけ修正する。

## Design review acceptance

ユーザー代弁: 有効な文書に誤診断・誤修正を加えない。自己否定: parserの導入による既存互換・性能・未閉じspan差分を調べる。悪魔の代弁者: 全面AST移行を避け、共有の重要構文と重複判定を対象にする。証跡は回帰失敗→修正後成功、全既存ゲート、性能reportとする。

## Fix convergence architecture

現状の確定事実: src/linter/core.rs、src/cli/workflow/common.rs、src/formatter.rs がそれぞれ独立した最大8回のcheck/fix反復を持ち、上限で未収束でも成功を返す。src/fix/mod.rs は位置順で競合を抑制するが同一位置の異種fix優先度が明記されていない。

採用設計:

1. Syntax adapter: pulldown-cmark 0.13.5 の offset event でコード境界を1文書1回で取得。原文rangeのadapterと未閉じcodeの保護契約を分ける。
2. Rules: 共有contextを参照し診断と修正候補のみ生成。MD010/MD011/MD034のコード判定は同じ包含APIを使用。
3. FixApplicator: 1スナップショットに対するUTF-8 byte editの競合調整と適用。既存公開fix_with_resultsはsingle-pass契約のまま。
4. FixEngine: 共有の反復責務。構文修復→内容/スタイル→レイアウトの段階で候補を選択し、適用後は必ず最新原文を再checkする。古い座標の候補を次の段階に使い回さない。候補選択と競合の順番は安定するようrule idも比較する。
5. Convergence: 同一出力を再訪したらcycle、上限後も変更可能なら未収束error。CLIは成功として書き戻さない。内容が同じno-op候補は適用数に含めない。外部UIに別の反復ループを持たせない。
6. Public entrypoints: library fix、CLI check/fix、formatterが同じengineを呼ぶ。path/config/severity/unsafe opt-inは呼出側から渡し、その挙動を維持する。LSP/MCPは既存library経由で再利用する。
7. Verification: 既存250件の正解データを維持し、入力→正解一致、2回目/3回目のbyte不変、answer自身のfix不変をrelease blockerにする。新規仕様/順序/余白/CRLF/UTF-8/競合パターンは独立した期待出力として拡張する。全ての任意Markdownを網羅したという主張はしない。

## Reviewed answer fixture defect

250件の既存正解比較で11件が不一致となった。対象は public-028/037/045/081/090/098/134/143/151/187/196。全件で新しいfix結果は入力原文とbyte一致、旧answerのみが quadruple-backtick code sample の内側へ `text` と空行を加えていた。CommonMark 4.5ではclosing fenceにinfo文字列を認めないため、info付きのquadruple行は外側fenceを閉じず、その後もcode contentである。新しい出力からanswerを生成する運用にはせず、原文をそのまま保護すべきケースとして期待値をレビューし、独立したparser範囲テストを追加したうえで旧answerを原文へ訂正する。

## Measured parser strategy

full parserを全contextで実行するとcontext生成5.67倍、inline token index約2倍となりstrict性能条件を破った。baselineは変更しない。SyntaxIndexを遅延生成し、既存source-preserving単一行scannerで全runが同じ行で閉じ、fence候補もない場合は共有scanner結果を再利用する。未閉じrunまたはfence候補があればpulldown-cmarkでblock/container/multiline境界を解決する。これはruleごとの別parserではなく、1つのadapter内部の明示した高速経路である。未閉じrunの保護は仕様構文と別の安全policyとして維持する。さらに全ruleがctxを受け取る評価経路へ移行し、block membershipをsort済みrangeの二分探索へ変更する。

## Same-machine performance reference

historical baselineのcatalog-onlyケースだけが1.42倍で残ったため、公開済み開始commit0898b93を git archive で target/perf-reference-0898b93 へ展開し、新規worktreeやbranchを作らず同一環境で再計測した（benchmark inputの既知MD012違反のみ同じ修正）。catalog sourceは候補と同じ。旧baseline 0.006206583ms、公開済みcommit再計測0.008474291ms、候補0.008809667msで、候補/公開済みは1.04倍。全20ケースで同一環境基準のstrict1.4を通過。historical baselineと閾値は改変せず保持する。raw証跡はtarget/perf-reference-report.jsonとtarget/perf-report.json。

## Responsibility inventory

| Concern | Owner | Consumer rule |
| --- | --- | --- |
| Fenced/multiline boundary and single-line source scanner | DocumentContext / SyntaxIndex | No rule owns backtick toggle state |
| ATX marker, whitespace and text range | document/headings source adapter | RuleHelpers delegates; duplicate heading uses the shared heading index |
| GFM table cell/source range | document/tables source adapter | MD055/056/058/060 consume the same rows |
| Diagnostic policy and fix candidate | Each MarkdownRule | No text mutation or convergence orchestration |
| Edit coordinate mapping and conflicts | fix/position + FixApplicator | No rule implements edit application |
| Order, recheck, cycle and pass budget | FixEngine | library / CLI / formatter delegate |
| Expected output and repeated fix evaluation | Existing document answer harness | Shared 258-fixture source of verification |

The source representation deliberately retains distinct byte-coordinate and editor UTF-16 boundaries: their adapters have different coordinate contracts. This is not a reason to duplicate Markdown semantics in the UI. Scope does not claim a wholesale replacement of every style-specific list/emphasis policy; syntax adapters and policy responsibilities remain explicit.

## Draft review corrections

Codexが指摘した2件を採用した。escaped backtick runは全体ではなく1文字だけをescapeし、残りを共通scannerで構文解析する。MD011はcandidate全体の包含ではなく、共通DocumentContextの範囲交差APIでcodeとの交差を保護する。両方向の境界交差とMD034/MD039の具体的なリテラルを修正前RED・修正後GREENで検査する。
