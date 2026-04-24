## Definition of Ready

- [ ] phase5 の upstream doc parser が存在していること
- [ ] `DavidAnson/markdownlint` default branch を参照できること
- [ ] fixture matrix が後続changeのDoRとして使われることが合意されていること
- [ ] 自動抽出できない公式doc記載を `manual_required` として扱うことが合意されていること
- [ ] rumdl / mado はUXとcoverage観点の参考であり、実装コピーしないことが明示されていること

## 1. Matrix Schema

- [ ] 1.1 rule fixture matrix のJSON schemaを定義する
- [ ] 1.2 rule id、aliases、tags、parameters、fixableをmatrixに含める
- [ ] 1.3 `check_pass` / `check_fail` / `fix` / `config_valid` / `config_invalid` / `edge` / `manual_required` をschemaに含める

## 2. Upstream Extraction

- [ ] 2.1 公式rule documentからexample候補を抽出する
- [ ] 2.2 parameters/default値をconfig fixture候補へ正規化する
- [ ] 2.3 fixabilityをfixture metadataへ反映する
- [ ] 2.4 自動抽出できない記載を `manual_required` として出力する

## 3. Reporting

- [ ] 3.1 matrix JSONを出力する
- [ ] 3.2 rule別Markdown summaryを出力する
- [ ] 3.3 rule別の不足数、manual_required数、fixture stale数を集計する

## 4. Verification

- [ ] 4.1 parser fixture testを追加する
- [ ] 4.2 matrix schema validation testを追加する
- [ ] 4.3 upstream doc変更時にstale matrixを検出できるtestを追加する

## Definition of Done

- [ ] 公式doc由来のrule fixture matrixが生成または更新できること
- [ ] 各ruleのcheck/fix/config/edge coverage状況がJSONとMarkdownで確認できること
- [ ] 後続changeがmatrixをDoRとして参照できること
