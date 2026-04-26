# Tasks

## Definition of Ready

- [ ] `v0.12.6` の release と archive が完了している。
- [ ] context-sensitive rule migration の分類と evidence が存在する。
- [ ] link-heavy / inline-code-heavy / reference-heavy benchmark case が存在する。
- [ ] `v0.12.x` は精度、速度、安定性だけを扱う方針である。
- [ ] 進行中に task 外の高リスク不足が露見した場合だけ、作業を中断してユーザー判断を仰ぐ。

## 0. Evidence Inventory

- [ ] `v0.12.5` と `v0.12.6` の parser / migration evidence を読み直す。
- [ ] 未説明の performance regression、fix oscillation、fmt idempotence gap を分類する。
- [ ] `v0.12.8` の score に必要な evidence の不足を tasks に残す。

## 1. Performance Hardening

- [ ] 実装前に `make perf-check` を実行し、baseline を記録する。
- [ ] parser index construction の repeated cost を確認する。
- [ ] API lint / API fix / CLI check / CLI fix / CLI fmt の hot path を分類する。
- [ ] 必要な最適化だけを行い、rule semantics を変えない。
- [ ] baseline refresh が必要な場合は、正しさの gate 後に理由付きで行う。

## 2. Check / Fix / Fmt Convergence

- [ ] `check` が no-write contract を保つことを再確認する。
- [ ] `check --fix` と `fix` が同じ safe fix contract に従うことを確認する。
- [ ] `fix` 再実行で同じ変更を繰り返さないことを確認する。
- [ ] `fmt` が formatter policy の範囲だけを変更することを確認する。
- [ ] `fmt` 再実行で差分が出ないことを確認する。

## 3. Corpus Expansion

- [ ] mixed corpus に parser-heavy document を追加する。
- [ ] link-heavy / inline-code-heavy / table-heavy / reference-heavy の check/fix/fmt expectations を固定する。
- [ ] dogfood 対象で見つかった違和感を再現可能な finding として記録する。

## 4. Stable Score Dry Run

- [ ] `v0.12.8` の安定版スコア項目に沿って dry-run 採点する。
- [ ] 90点未満または hard blocker に相当する不足を次の task として分類する。
- [ ] 採点根拠を tasks に残す。

## 5. Release Preparation

- [ ] crate version を `0.12.7` に更新する。
- [ ] `CHANGELOG.md` に performance / convergence hardening を英語で記載する。
- [ ] OpenSpec delta を main specs に同期し、完了後に archive する。
- [ ] release 前に `make release-check VERSION=v0.12.7` を通す。

## Verification

- [ ] `make fmt-check`
- [ ] `make lint`
- [ ] `make ast-lint`
- [ ] `cargo test --workspace --locked`
- [ ] `cargo test --workspace --all-features --locked`
- [ ] `cargo test --locked --test document_false_positive_regressions`
- [ ] `cargo test --locked --test rule_fixture_harness`
- [ ] `cargo test --locked --test upstream_golden_comparison`
- [ ] `make dogfood`
- [ ] `make perf-check`
- [ ] `make release-check VERSION=v0.12.7`
- [ ] `git diff --check`

## Definition of Done

- [ ] parser / context migration 後の performance evidence が揃っている。
- [ ] check / fix / fmt の収束性が corpus で固定されている。
- [ ] stable score dry-run が実行され、不足が分類されている。
- [ ] 配布展開に関係する task は進んでいない。
