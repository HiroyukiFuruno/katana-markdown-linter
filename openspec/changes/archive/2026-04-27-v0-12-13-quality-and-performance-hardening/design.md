# Design for v0.12.13 Quality & Performance Hardening

## 1. src/cli/workflow.rs の責務分割

### 1.1 現状の問題

`src/cli/workflow.rs` は 1013 コード行・35 関数・3 impl ブロックを持つ size_score 1197 のファイルで、split_candidates のトップに位置します。
check / fix / fmt / config の 4 コマンドフローが一枚岩で実装されており、可読性・テスト容易性・将来の機能追加コストに悪影響を与えています。

### 1.2 分割方針

`src/cli/workflow.rs` を `src/cli/workflow/` ディレクトリに変換し、責務別に 4 ファイルへ分割します。

```
src/cli/workflow/
  mod.rs          # pub use re-export のみ（スコア ≤20）
  common.rs       # 共有型・共有関数（UnsafeFixPolicy, FixedContent,
                  #   validate_effective_config, load_effective_config,
                  #   apply_fixes_until_stable など）
  check.rs        # check/fix コマンドフロー
                  #   run_check_like, run_stdin_check_like,
                  #   resolve_unsafe_fix_policy など
  fmt.rs          # fmt コマンドフロー
                  #   run_fmt, run_stdin_fmt, format_stdin_content など
  config_cmd.rs   # rule/config コマンド群
                  #   run_rule, run_config, render_rule, render_config,
                  #   prompt_unsafe_confirmation など
```

### 1.3 分割手順

循環 import を防ぐため、以下の順序で移動します。

1. `common.rs` を先に作成し、共有型（`UnsafeFixPolicy`, `FixedContent` 等）と共有関数を移動する
2. `check.rs` を作成し、`run_check_like` / `run_stdin_check_like` を移動する（`common` を use）
3. `fmt.rs` を作成し、`run_fmt` / `run_stdin_fmt` を移動する（`common` を use）
4. `config_cmd.rs` を作成し、rule/config 系関数を移動する
5. `mod.rs` に `pub use` を集約し、旧 `workflow.rs` を削除する
6. `src/cli/mod.rs` の `pub mod workflow;` を維持（変更不要）

各ステップで `cargo check` を実行し、常にコンパイルが通る状態を維持します。

### 1.4 目標スコア

| ファイル | 目標 size_score |
|---|---|
| src/cli/workflow/mod.rs | ≤20 |
| src/cli/workflow/common.rs | ≤250 |
| src/cli/workflow/check.rs | ≤400 |
| src/cli/workflow/fmt.rs | ≤150 |
| src/cli/workflow/config_cmd.rs | ≤200 |

## 2. md059.rs の中間 Vec 除去

### 2.1 現状

```rust
fn normalize_link_text(link_text: &str) -> String {
    link_text
        .split_whitespace()
        .collect::<Vec<_>>()  // ← 不要な Vec アロケーション
        .join(" ")
        .to_lowercase()
}
```

`split_whitespace()` の結果を `Vec` に collect してから `join` しているが、`join` は `&[&str]` を要求するため中間 Vec が必要に見えるが、等価な実装を Vec なしで書ける。

### 2.2 修正案

```rust
fn normalize_link_text(link_text: &str) -> String {
    let mut result = String::new();
    for (i, word) in link_text.split_whitespace().enumerate() {
        if i > 0 {
            result.push(' ');
        }
        result.push_str(word);
    }
    result.to_lowercase()
}
```

中間 Vec アロケーションを排除しながら同一の出力を保証します。

## 3. 品質ゲート継続

### 3.1 必須ゲート

`v0.12.12` と同一の品質ゲートを全通過させます。

- `make ast-lint`
- `cargo test --all-features --locked`
- `make perf-check-strict`（ratio ≤ 1.40x）
- `make public-confidence`（unclassified: 0）
- `make internal-quality-check`（workflow.rs が split_candidates 最上位から除外）
- `make coverage-blocking`（uncovered ≤ baseline）
- `make release-check VERSION=v0.12.13`

### 3.2 パフォーマンスベースライン

モジュール分割後に `make perf-check-strict` を通過させ、改善根拠がある場合のみ `make perf-refresh-baseline` を実行します。
ratio が 1.40x を超えた場合は `#[inline]` 付与または関数統合で対応し、ベースライン更新より先に改善を優先します。
