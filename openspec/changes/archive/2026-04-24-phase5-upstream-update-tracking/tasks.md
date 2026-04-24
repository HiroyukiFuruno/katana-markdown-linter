## Definition of Ready
- [x] phase1, phase2, phase3, phase4 の `tasks.md` が全て完了していること
- [x] phase2 の rule catalog が全 active rule を `implemented_check` として扱っていること
- [x] upstream `markdownlint` の rule document を参照できること
- [x] local rule metadata の source of truth が決まっていること
- [x] upstream source ref が `DavidAnson/markdownlint` default branch として固定されていること
- [x] snapshot 管理方法、unknown drift の扱いがユーザーと合意されていること

## 1. Upstream Catalog Tracking

- [x] 1.1 upstream rule catalog を snapshot として取り込めるようにする
- [x] 1.2 MD0XX の追加・削除・並び替えを検出する
- [x] 1.3 deprecated / removed rule を distinct に可視化する

## 2. Rule Doc Parsing

- [x] 2.1 各 rule の document md を構造化して解析し、local の `Document` / `RuleMeta` と比較可能にする
- [x] 2.2 rule id、name、summary、tags、properties を抽出する
- [x] 2.3 fixability と default config の情報を normalized JSON として比較可能にする

## 3. Drift Checking

- [x] 3.1 local rule 実装と upstream doc の差分を `Document` ベースで check する
- [x] 3.2 local config helper と upstream config contract の差分を check する
- [x] 3.3 drift report を JSON と Markdown summary の両方で出力する

## 4. Quality Gates

- [x] 4.1 catalog diff の regression test を追加する
- [x] 4.2 parser fixture を追加する
- [x] 4.3 stale / deprecated / missing の分類が安定していることを確認する

## Definition of Done
- [x] upstream 変更に対する追従ポイントが JSON report と Markdown summary で確認できること
- [x] deprecated / removed / missing / mismatch が lifecycle state と drift type として区別できること
- [x] local rule と config の drift を CI で検出し、未知の drift があれば失敗すること
