## Definition of Ready

- [x] 前提changeがarchive済みで、active change がこの作業と競合しないこと
- [x] public catalog の active rule / fixable rule が取得できること
- [x] fixture matrix JSON / Markdown summary が存在すること
- [x] `mdxxx.rs` は check / fix のみを責務にする方針を維持すること
- [x] unsafe fix は実装しない、または明示設定時だけ有効化する方針が明確であること

## 1. Matrix Consistency

- [x] 1.1 public catalog fixable set と fixture matrix fixable set を一致させる
- [x] 1.2 実装済み fix_info と matrix の fix / manual_required を同期する
- [x] 1.3 stale な manual_required を削除または現状に合わせて書き換える
- [x] 1.4 Markdown summary を JSON と同じ内容に更新する
- [x] 1.5 matrix inconsistency を検出する test を追加または強化する

## 2. Config Property Runtime

- [x] 2.1 `LintOptions.rules[*].properties` を rule runtime へ渡す
- [x] 2.2 CLI config object の properties を `LintOptions` に保持する
- [x] 2.3 default `MarkdownRule::evaluate_configured` を追加して既存 rule 互換を保つ
- [x] 2.4 設定依存 rule の default-safe 挙動を unit test で固定する

## 3. Rule Safety And Parity

- [x] 3.1 `MD044` は `names` 未指定時に診断/fixしない
- [x] 3.2 `MD054` は該当 style が無効化されている場合だけ診断/fixする
- [x] 3.3 `MD049` / `MD050` の style config を反映する
- [x] 3.4 `MD051` の heading slug / custom anchor / ignored pattern を強化する
- [x] 3.5 `MD058` の table boundary detection を公式doc例で固定する
- [x] 3.6 `MD060` は unsafe fix として明示し、strategy-aware formatting を future work 化する

## 4. Official Example Coverage

- [x] 4.1 公式 document の check pass / fail examples を matrix に反映する
- [x] 4.2 公式 document の fixable examples を before/after fixture に反映する
- [x] 4.3 parameterized rule は config 別 fixture を追加する
- [x] 4.4 edge cases を rule group ごとに追加する

## 5. Fix Strategy Verification

- [x] 5.1 multi-line fix range を適用できる fix engine にする
- [x] 5.2 overlapping fix range は安全に skip する
- [x] 5.3 CLI fix は安定するまで反復適用する
- [x] 5.4 rule dependency / priority が必要なケースを fixture で可視化する

## 6. Verification

- [x] 6.1 `cargo test --workspace` を通す
- [x] 6.2 `make check` を通す
- [x] 6.3 `cargo build --release --bin kml --locked` を通す
- [x] 6.4 dogfood baseline を通す
- [x] 6.5 cross-tool benchmark の fix validation を通す
- [x] 6.6 `git diff --check` を通す

## Definition of Done

- [x] fixture matrix が public catalog、実装済み fix、manual_required と矛盾しないこと
- [x] 設定依存 rule が default で推測修正しないこと
- [x] 公式 document examples の不足が `manual_required` または task として明示されていること
- [x] `make check`, dogfood, release build, benchmark fix validation, `git diff --check` が通っていること
