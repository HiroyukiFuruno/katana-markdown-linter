# v0.18.4 Editor Release Gate Finalization

## 対象バージョン

- `v0.18.4`

## 目的

`v0.18.3` の editor extension hardening の成果をもとに、release 判定に直結する gate を最終確定する。
`release-verify` で editor artifact の状態（`published`/`deferred`）が説明されることと、`release-check` が同じ前提で状態解釈することを固定する。

## 変更内容

- `release-verify` が editor artifact の状態を明示する
- `release-check` が `release-verify` の状態ルールを前提に分岐する
- その結果を `release-runbook` で再現可能な形で保存する
- `open spec change` と `active-roadmap` の状態記述を一致させる

## 検証対象（明示）

- VS Code extension package（配布 artifact の 1 つとして `vscode`）
- Zed extension package（配布 artifact の 1 つとして `zed`）
- いずれかが `published` / `deferred` のいずれかに評価されること
- `open spec` と `release-check` は、`release-verify` の同一キー集合を参照して判定すること

## 非対応範囲

- 外部 marketplace の実公開実行は含めない
- `check` / `fix` / `format` エンジン変更は含めない

## 影響対象

- `scripts/release/**`
- `Justfile`
- `docs/release-runbook.md`
- `openspec/changes/active-roadmap.md`
