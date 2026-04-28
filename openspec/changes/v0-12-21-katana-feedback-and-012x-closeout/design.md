## Context

`v0.12.21` は単なる文書整理ではなく、`v0.12.20` 後の実運用確認を受ける buffer として使う。
KatanA 側の複数ドキュメントを読ませることで、synthetic fixture では見えない false positive、false negative、fix 事故、性能問題を拾う。

そのうえで、0.12.x で安全に実装するものと、意図的に実装しないものを分ける。

## Goals / Non-Goals

**Goals:**

- KatanA feedback で出た issue を `v0.12.21` の対象として分類する。
- release-blocking な bug は `v0.12.21` で修正する。
- 残り diagnostic-only rule の by-design 理由を公開表示に反映する。
- 0.12.x を閉じ、`v0.13.0` の配布計画へ進める条件を明確にする。

**Non-Goals:**

- KatanA repository を required CI dependency にしない。
- 全ての feedback を 0.12.x に詰め込まない。
- 人間の意図が必要な unsafe fix を default safe-fix に混ぜない。

## Decisions

### D-1. KatanA feedback は分類してから扱う

feedback は次の分類で扱う。

- `false-positive`: 診断すべきでないものを診断した。
- `false-negative`: 診断すべきものを見逃した。
- `unsafe-fix-risk`: 自動修正すると意味が変わる可能性がある。
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

## Risks / Trade-offs

- [Risk] KatanA 固有の文書に引きずられ、汎用 linter の境界を崩す。
  - Mitigation: finding は汎用 rule behavior として分類できるものだけ修正する。
- [Risk] feedback を全部詰め込み、0.12.x が終わらない。
  - Mitigation: release-blocking と follow-up を明確に分ける。
- [Risk] by-design 宣言が単なる未対応リストに見える。
  - Mitigation: 各 rule に「なぜ safe-fix しないか」を短く書く。
