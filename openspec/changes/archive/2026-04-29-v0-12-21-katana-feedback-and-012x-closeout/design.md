## Context

`v0.12.21` は単なる文書整理ではなく、`v0.12.20` 後の実運用確認を受ける buffer として使う。
KatanA 側の複数ドキュメントを読ませることで、synthetic fixture では見えない false positive、false negative、fix 事故、性能問題を拾う。

そのうえで、0.12.x で安全に実装するものと、意図的に実装しないものを分ける。

## Goals / Non-Goals

### Goals

- CLI の directory traversal で、通常 git 管理しない予約領域を既定除外にする。
- 予約領域を確認したい場合は、明示 opt-in で対象にできる。
- KatanA feedback で出た issue を `v0.12.21` の対象として分類する。
- `/tmp` の検証 worktree で `kml fix` を実行し、fix 後差分を全件確認する。
- release-blocking な bug は `v0.12.21` で修正する。
- 残り diagnostic-only rule の by-design 理由を公開表示に反映する。
- 0.12.x を閉じ、`v0.13.0` の配布計画へ進める条件を明確にする。

### Non-Goals

- KatanA repository を required CI dependency にしない。
- 元の KatanA checkout を直接書き換えない。
- KatanA の `verify` branch を remote に push しない。
- 全ての feedback を 0.12.x に詰め込まない。
- 人間の意図が必要な unsafe fix を default safe-fix に混ぜない。

## Decisions

### D-1. KatanA feedback は分類してから扱う

feedback は次の分類で扱う。

- `false-positive`: 診断すべきでないものを診断した。
- `false-negative`: 診断すべきものを見逃した。
- `unsafe-fix-risk`: 自動修正すると意味が変わる可能性がある。
- `bad-fix`: 診断は正しいが、自動修正後の Markdown が壊れる、または意味を変える。
- `fmt-policy-gap`: formatter 方針の未定義。
- `perf-regression`: 実文書で説明不能に遅い。
- `docs-only`: 表示や説明だけ直せばよい。

release-blocking かどうかを先に決め、blocking だけを `v0.12.21` の bugfix として扱う。

### D-2. by-design 宣言は「未着手」ではなく「意図的にしない」ことを示す

対象候補は、`v0.12.20` 時点で safe-fix が残っていない rule である。
初期候補は次の通り。

- `MD001`: 見出しレベル変更は文書構造と anchor を変える。
- `MD013`: 行折り返しは prose、code、table、inline reference の意図を壊し得る。
- `MD024`: 重複見出しの新しい名前は人間が決める必要がある。
- `MD033`: HTML の削除や置換は表示結果を変える。
- `MD041`: 最初の見出し本文は推測できない。
- `MD042`: 空リンクのリンク先は推測できない。
- `MD043`: 必須見出しの本文と順序は人間が決める必要がある。
- `MD045`: alt text は画像内容と意図を理解する必要がある。
- `MD059`: descriptive link text はリンク先の目的を人間が決める必要がある。
- `MD028`: `v0.12.19` で safe subset を定義できなかった場合のみ含める。

### D-3. 0.12.x closeout は KatanA feedback 後に判断する

`v0.12.21` の完了条件は、by-design 宣言だけではない。
KatanA feedback 由来の release-blocking issue が 0 件であることも必要にする。

### D-4. 予約領域は既定で再帰走査しない

`node_modules` や `.git` のように、通常は git 管理せず、依存物・cache・生成物を置く directory は既定で lint / fix / fmt 対象外にする。
これは `.gitignore` の有無に依存しない安全側の既定値とする。

予約領域をあえて確認したい場合は、明示 option で対象に戻せるようにする。
この option は `check`、`fix`、`fmt` の directory traversal で同じ意味を持つ。

### D-5. KatanA fix 検証は隔離 worktree で行う

KatanA の元 checkout は作業中差分やユーザー作業を含む可能性があるため、直接 fix しない。
`/tmp` 配下に git worktree を作成し、local branch `verify` を使う。

元 checkout に git 管理外 Markdown がある場合は、検証対象として必要なものを worktree 側に取り込み、baseline commit に含める。
その後にまず `kml check --output json` を実行し、fix 前の diagnostics を保存する。
この時点の check 結果は、diagnostics が出た file だけでなく、diagnostics 0 件の file も含めて全対象 file を評価対象にする。
diagnostics 0 件という結果も「対象 file に対して何も検出しなかった」という check 結果として台帳に残す。
check 評価台帳には、全対象 file ごとに diagnostics 数、rule 一覧、check 精査結果、根拠メモ、kml 側対応要否を記録する。
check 評価台帳は全対象 file 数と同じ行数を持ち、`未評価` が 0 件になるまで完了扱いにしない。
`markdownlint` など他実装との比較は、見逃し候補や誤検知候補を探すための補助線として使う。
ただし他実装の結果を正解として扱わず、最終判断は markdownlint rule の仕様、KatanA 文書の文脈、kml の安全性契約を合わせて行う。
仕様文書と他実装の挙動に差がある場合、または他実装側の false-positive / false-negative が疑われる場合は、markdownlint upstream issue / PR も確認する。
他実装に合わせない判断をする場合も、台帳に理由を残す。
check 側に release-blocking な false-positive / false-negative がある場合は、先に kml 側を修正し、再度 `kml check --output json` を実行して基準 check 結果を更新する。
fix はその最適化後の基準 check 結果を前提に実行する。
初回 check 後に得た fix 結果は観測資料であり、check 修正後の再実行なしに最終 release evidence として扱わない。
続いて `kml fix --output json` を実行し、どの rule に対して何が適用されたかを保存する。
fix 評価台帳にも、差分が出た file だけでなく差分 0 件の file を含め、全対象 file ごとに applied fix 数、rule 一覧、diff 有無、fix 精査結果、根拠メモ、kml 側対応要否を記録する。
fix 評価台帳も全対象 file 数と同じ行数を持ち、`未評価` が 0 件になるまで完了扱いにしない。
fix 後の git diff は、最適化後の check 結果と fix 結果を突き合わせて file / hunk 単位で確認する。
check が正当で diagnostics 0 件と評価された file は、通常 fix 評価では `-` になる。
その file に fix 差分が出た場合は、`check-fix-inconsistency` として扱い、check と fix の契約不一致を疑う。
この突き合わせのため、JSON report には file ごとの applied fix detail を含める。
事前 check 結果も正解として扱わず、diagnostic 自体の false-positive / false-negative 可能性を評価対象に含める。
確認の目的は KatanA 文書側の採用判断ではなく、kml の誤検知、誤修正、意味変化リスクを見つけることである。
大量の実ドキュメントで見つかった `check` の誤検知と `fix` の誤修正は、OSS library としての信頼性を直接下げるため release-blocking として扱う。
差分が多い場合も file / hunk ごとに評価する。
rule / change pattern の分類は進行管理の補助であり、最終評価は diff の周辺 Markdown 文脈を読んで行う。
機械的な一致だけで安全判定せず、表示、リンク、見出し構造、code block、table の意味が変わる場合は kml 側の改善候補として扱う。

## Risks / Trade-offs

- Risk: KatanA 固有の文書に引きずられ、汎用 linter の境界を崩す。
  - Mitigation: finding は汎用 rule behavior として分類できるものだけ修正する。
- Risk: feedback を全部詰め込み、0.12.x が終わらない。
  - Mitigation: release-blocking と follow-up を明確に分ける。
- Risk: by-design 宣言が単なる未対応リストに見える。
  - Mitigation: 各 rule に「なぜ safe-fix しないか」を短く書く。
- Risk: 予約領域の既定除外で、意図的に lint したい生成済み Markdown を見落とす。
  - Mitigation: 明示 opt-in option を提供し、help と regression test で挙動を固定する。
- Risk: KatanA fix 検証で元 checkout の作業を壊す。
  - Mitigation: `/tmp` worktree と baseline commit を使い、元 checkout には書き込まない。
