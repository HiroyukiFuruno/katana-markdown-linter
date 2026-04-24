## Definition of Ready

- [x] `rule-fixture-parity-matrix` のtasksが全て完了していること
- [x] fixture matrix JSON / Markdown summary が生成済みであること
- [x] `mdxxx.rs` は check / fix のみを責務にする方針が維持されていること
- [x] safe fix の判断基準が公式docまたは互換実装の挙動で説明できること
- [x] rule順序依存はrule実装ではなくstrategy側課題として記録すること

## 1. Test Harness

- [x] 1.1 fixture matrixを読み込むunit test harnessを追加する
- [x] 1.2 check pass / fail fixtureを実行できるようにする
- [x] 1.3 fix fixtureをbefore/after比較できるようにする
- [x] 1.4 config valid / invalid fixtureを検証できるようにする

## 2. Rule Check Coverage

- [x] 2.1 MD001 check fixtureを通す
- [x] 2.2 MD003 check fixtureを通す
- [x] 2.3 MD004 check fixtureを通す
- [x] 2.4 MD005 check fixtureを通す
- [x] 2.5 MD007 check fixtureを通す
- [x] 2.6 MD009-MD014 check fixtureを通す
- [x] 2.7 MD018-MD024 check fixtureを通す
- [x] 2.8 MD025-MD035 check fixtureを通す
- [x] 2.9 MD036-MD046 check fixtureを通す
- [x] 2.10 MD047-MD060 check fixtureを通す

## 3. Rule Fix Coverage

- [x] 3.1 fixable rule一覧をmatrixから確定する
- [x] 3.2 fixable ruleのbefore/after fixtureを追加する
- [x] 3.3 unsafeまたは曖昧なfixは非対応理由をmetadata化する
- [x] 3.4 複数fixの同一範囲衝突を検出するtestを追加する

## 4. Config Coverage

- [x] 4.1 全ruleのvalid config fixtureを通す
- [x] 4.2 全ruleのinvalid config fixtureを通す
- [x] 4.3 alias / deprecated / removed ruleの扱いをfixtureで固定する
- [x] 4.4 unknown property / wrong type / invalid enumをfixtureで固定する

## 5. Edge and Regression Coverage

- [x] 5.1 空ファイル、改行なし、巨大行、code fence内、HTML混在をfixture化する
- [x] 5.2 list / heading / table の境界値をfixture化する
- [x] 5.3 front matter とGFM extensionの扱いを明示する
- [x] 5.4 rumdl / mado の公開CLI挙動から有用なedge caseを参考として追加する

## Definition of Done

- [x] 全active ruleのcheck fixtureが通っていること
- [x] fixable ruleのfix fixtureが通っていること
- [x] 全ruleのconfig valid / invalid fixtureが通っていること
- [x] 未対応・非対応・manual_requiredが0または明示された例外として記録されていること
