## Definition of Ready

- [x] phase5 の upstream doc parser が存在していること
- [x] `DavidAnson/markdownlint` default branch を参照できること
- [x] fixture matrix が後続changeのDoRとして使われることが合意されていること
- [x] 自動抽出できない公式doc記載を `manual_required` として扱うことが合意されていること
- [x] rumdl / mado はUXとcoverage観点の参考であり、実装コピーしないことが明示されていること

## 1. Matrix Schema

- [x] 1.1 rule fixture matrix のJSON schemaを定義する
- [x] 1.2 rule id、aliases、tags、parameters、fixableをmatrixに含める
- [x] 1.3 `check_pass` / `check_fail` / `fix` / `config_valid` / `config_invalid` / `edge` / `manual_required` をschemaに含める

## 2. Upstream Extraction

- [x] 2.1 公式rule documentからexample候補を抽出する
- [x] 2.2 parameters/default値をconfig fixture候補へ正規化する
- [x] 2.3 fixabilityをfixture metadataへ反映する
- [x] 2.4 自動抽出できない記載を `manual_required` として出力する

## 3. Reporting

- [x] 3.1 matrix JSONを出力する
- [x] 3.2 rule別Markdown summaryを出力する
- [x] 3.3 rule別の不足数、manual_required数、fixture stale数を集計する

## 4. Verification

- [x] 4.1 parser fixture testを追加する
- [x] 4.2 matrix schema validation testを追加する
- [x] 4.3 upstream doc変更時にstale matrixを検出できるtestを追加する

## Definition of Done

- [x] 公式doc由来のrule fixture matrixが生成または更新できること
- [x] 各ruleのcheck/fix/config/edge coverage状況がJSONとMarkdownで確認できること
- [x] 後続changeがmatrixをDoRとして参照できること
