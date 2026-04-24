## Context

現在の `make lint` はClippy、`make ast-lint` はrepository固有テストを実行する。
直近で ast-lint はlazy macro、parallel walker、signed release tag、rule id uniquenessを検査するようになったが、fixture matrixやCLI拡張を守るには追加のgateが必要になる。

## Goals / Non-Goals

**Goals:**

- `make lint` はClippy zero warning gateとして維持する
- `make ast-lint` はrepository固有の構造・運用不変条件を検査する
- CIとlocal make targetの対応を明確化する
- coverage reportとblocking gateを分離する
- release readinessに必要なgateを明示する

**Non-Goals:**

- CI providerをGitHub Actions以外へ増やさない
- すべてのcoverageを100% blockingにしない
- KatanAのUI固有lintをこのrepoに持ち込まない

## Decisions

### 1. lint と ast-lint を分離する

`lint` はClippyだけを担う。
`ast-lint` はこのrepo固有の不変条件を担う。

### 2. ast-lint は増やしすぎず、回帰価値の高いものに限定する

追加対象は次に限定する。

- rule fixture matrix coverage
- upstream drift unknown check
- release signed tag workflow
- CLI traversal / gitignore behavior
- lazy macro禁止
- public API / rule catalogの破壊的変更検出

### 3. CI required checksはMakefile targetと対応させる

GitHub required checks名とlocal commandをrunbookで対応付ける。
必要な場合のみrequired checkを更新する。

## Risks / Trade-offs

- ast-lintが増えすぎると修正コストが高くなる
- upstream default branchに依存するgateは外部要因で失敗する
- coverageをblockingにすると初期開発速度が落ちる

## Migration Plan

1. lint / ast-lint / check / coverage の責務をREADMEまたはrunbookに明記する
2. fixture matrix coverage gateをast-lintへ追加する
3. upstream drift gateの実行モードを整理する
4. release readiness gateをMakefileに整理する
5. CI required checksを必要に応じて更新する
