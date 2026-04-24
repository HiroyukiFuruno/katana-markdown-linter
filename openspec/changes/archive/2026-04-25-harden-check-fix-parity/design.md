## Context

この repository は Rust 組み込みライブラリを主軸とし、CLI は同じ check / fix engine を利用する。
`mdxxx.rs` は rule 固有の check / fix だけを担当し、有効化、設定注入、順序制御、複合 fix strategy は別責務とする。

## Goals / Non-Goals

### Goals

- public catalog の fixable metadata、fixture matrix、実装済み fix_info を整合させる
- 設定依存 rule は config property を明示的に受け取る
- 公式 document examples を可能な限り fixture 化する
- unsafe fix は診断のみ、または明示設定時のみ fixable にする
- benchmark / dogfood / unit test で check と fix 後の結果を検証する

### Non-Goals

- upstream markdownlint の実装をコピーしない
- unsafe な自動修正を「対応rule数」のために有効化しない
- CLI UX の大幅な追加はこの change では扱わない

## Decisions

### 1. Matrix を source of truth として復旧する

fixture matrix は現行実装の進捗表ではなく、公式 document と local safety policy の合意点を表す。
実装が進んだら matrix の `fixable`、`fix`、`manual_required` を同時に更新する。

### 2. Config property を rule runtime に渡す

`LintOptions.rules[rule_id].properties` を `MarkdownRule::evaluate_configured` に渡す。
既存 rule は default 実装で `evaluate` を使い続け、設定依存 rule だけ override する。

### 3. Safety-first fix policy

fix は以下のいずれかを満たす場合だけ `fix_info` を出す。

- 公式 document の before/after が一意である
- local parser が違反範囲を正確に特定できる
- config property により正しい変換先が明示されている

上記を満たさない場合は diagnostic のみ、または `manual_required` に理由を記録する。

### 4. Heuristic rule は小さく安全にする

`MD044`, `MD051`, `MD054` など文脈依存 rule は、default で推測しない。
安全に判断できる subset だけを fixture として固定し、残りは manual_required または future work とする。

## Risks / Trade-offs

- 公式完全互換を一度に狙うと scope が肥大化するため、fixture 化できた範囲を先に安全に固定する
- 設定注入の導入により trait surface が増えるが、default method で既存 rule の変更を最小化する
- matrix 更新を怠ると品質ゲートが再び嘘をつくため、matrix consistency test を強める

## Migration Plan

1. OpenSpec artifacts を作成して scope を固定する
2. fixture matrix と Markdown summary を現行実装へ同期する
3. config property runtime path を導入する
4. 設定依存 rule を default-safe にする
5. 公式 document examples 由来の fixture を追加する
6. `make check`, dogfood, benchmark fix validation を通す
