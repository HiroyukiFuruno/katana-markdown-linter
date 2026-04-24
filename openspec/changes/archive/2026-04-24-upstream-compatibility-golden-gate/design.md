## Context

この repository は markdownlint 互換を目指しており、公式 document の fixture matrix と upstream drift check は既に導入されている。一方で、document に書かれていない edge behavior や実装都合の診断位置は、本家 implementation と直接比較しないと検出しにくい。

Golden comparison は、通常の unit test と同じ速度・安定性を求める gate ではなく、互換性の根拠を強めるための専用 gate として扱う。通常 test は deterministic な local fixture を使い、upstream 追従確認は明示 target に分離する。

## Goals / Non-Goals

**Goals:**

- upstream markdownlint と `kml` の診断差分を fixture 単位で比較する
- message 文言ではなく、互換性判断に必要な構造化 fields を比較する
- known delta と unknown delta を分離する
- rule coverage dashboard で rule ごとの実装・fixture・golden 状態を可視化する

**Non-Goals:**

- upstream markdownlint の message text を完全一致させること
- network access を通常 test の必須条件にすること
- 全差分を一度にゼロにすること
- CLI UX 改善や MCP server 実装を同時に行うこと

## Decisions

- Comparison harness は正規化済み診断を比較する。比較対象は rule id、line、column、range、fix applicability、fixed output digest を中心にし、message text は参考情報に留める。
- Known delta は構造化ファイルで管理する。理由、対象 rule、fixture、期限または解消条件を必須にする。
- 通常の `make test` には network-dependent update を入れない。`make upstream-golden` のような明示 target で oracle 更新と差分検出を行う。
- Dashboard は生成物として `docs/` 配下に置き、rule ID ごとに check / fix / config / edge / golden / known delta を一覧化する。
- Upstream binary は reproducibility を優先し、通常 gate では固定 version または lock された実行環境を使う。default branch 追従は既存の upstream drift gate と連携して、別 target で確認する。

## Risks / Trade-offs

- upstream markdownlint の出力仕様変更で false positive が出る可能性がある。
  Mitigation: comparison を normalized schema 経由にし、message text 依存を避ける。

- known delta が例外リストとして肥大化する可能性がある。
  Mitigation: reason と解消条件を必須化し、dashboard で可視化する。

- Node/npm 依存が Rust crate の通常開発を重くする可能性がある。
  Mitigation: explicit target と CI job 分離で通常 gate から切り離す。
