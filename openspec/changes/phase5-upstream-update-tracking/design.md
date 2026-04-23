## Context

この repository の価値は、単なる markdownlint の再実装ではなく、upstream の仕様変化に追従し続けられることにある。
そのためには、human-readable な docs を読み込むだけでなく、rule ID、metadata、config properties を machine-readable にして比較できる必要がある。

## Goals / Non-Goals

**Goals:**

- upstream rule の増減を検出する
- deprecated / removed rule を local の一覧と比較して可視化する
- rule doc の構造を解析し、local 実装との差分を検出する
- config schema と rule property の drift を検出する

**Non-Goals:**

- upstream の各 rule を再実装すること自体
- CLI UX の拡張
- publish / release 手順の変更

## Decisions

### 1. upstream docs は構造化入力として扱う

rule docs を単なる Markdown テキストではなく、rule id、title、summary、properties、examples、fix notes の構造を持つ input として扱う。
これにより、description の文言差だけでなく、property 追加・削除・型変化も比較できる。

### 2. 差分は add / remove / deprecate / mismatch に分類する

更新追従で必要なのは「違う」ことの一括表示ではない。
新規 rule、削除 rule、deprecated rule、そして metadata mismatch を分けて報告する。

### 3. 乖離検査は保守用の ast-lint として実装する

local 実装の source tree と upstream docs を比較する lint を、開発用の保守チェックとして持つ。
これは end-user 向け lint ではなく、repository を壊さず upstream 追従するための内部品質ゲートである。

## Risks / Trade-offs

- upstream docs の書式変更で parser が壊れる可能性がある
- deprecated / removed の判断基準が文書だけでは曖昧な場合があるため、判断不能な rule は `unknown_needs_review` として CI failure にする
- 構造化しすぎると parser 実装が本体より重くなる

## Confirmed Decisions

以下はユーザー確認済みの決定事項である。

- upstream source は `DavidAnson/markdownlint` の default branch 追従とする

## User Decisions

以下は update tracking 実装前にユーザーと協議して確定する。

- upstream snapshot を repository に commit するか、CI 実行時に取得するか
- deprecated / removed 判定を docs 内の明示記述だけに限定するか、upstream source code の metadata も解析対象に含めるか
- unknown drift を常に CI failure にするか、allowlist file に明示した差分だけ許可するか
- drift report の保存先を `target/` の一時 artifact にするか、`tmp/` や `docs/` に reviewable file として出すか

## Migration Plan

1. upstream rule catalog の snapshot を作る
2. rule doc parser を導入する
3. local rule metadata と比較する checker を作る
4. drift report を CI で回し、unknown drift を失敗扱いにする
