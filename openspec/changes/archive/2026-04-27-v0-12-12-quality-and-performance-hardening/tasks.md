## Definition of Ready

- [x] `v0.12.11` の公開向け gate が完了し、リリース済みであること
- [x] 本バージョンが「機能追加ゼロ」の品質・性能特化リリースであることの合意
- [x] パフォーマンス向上よりも検査精度を絶対優先する「`precision-first`」の誓約（一切の挙動退行を許容しない）
- [x] `make internal-quality-check` および `make perf-check` を事前に実行し、リファクタリング前の現状値（ベースライン）が `target/` 配下に保存されていること

## 1. Quick Wins & Stabilization (Phase 1)

*目的: リファクタリング中の検証を阻害するノイズを最初に取り除き、検証環境を完全にクリーンにする*

- [x] 1.1 `md-broken-link`（ローカルファイルパスのリンク切れをチェックするKML独自ルール）による誤検知を解消するため、ルールのコード自体は将来の機能拡張のために残しつつも、デフォルトの検証フローでは実行（評価）されないように仕様変更する
- [x] 1.2 `make public-confidence` および `make external-katana-dogfood` を実行し、未分類エラーがゼロの「完全にクリーンな状態」になったことを確認する

## 2. Quality Gate Overhaul (Phase 2)

*目的: リファクタリング作業で新たな技術的負債が混入しないよう、防波堤（Quality Gate）を先に拡張・厳格化する*

- [x] 2.1 `tests/ast_linter.rs` 内の特定ファイルへの依存（例: `read_workspace_file("src/cli.rs")`）を排除し、`scan_rust_sources` などを活用して `src/`, `tests/`, `build.rs` を例外なく対象とするようルールを汎用化する
- [x] 2.2 上記に伴い、「`WalkBuilder` には `.build_parallel()` を強制する」等のCLI固有ルールがテスト等で誤検知しないよう、ルール内で `if path.starts_with("src/cli")` の判定を入れ「適材適所」にフィルタリングする
- [x] 2.3 `scripts/ci/internal-quality.py` 等の検証スクリプトの実行引数・対象パスを拡張し、`tests/` と `build.rs` の負債化も検出・監視できるようにする

## 3. Architecture Refactoring (Phase 3)

*目的: 厳格化されたGateの元で、巨大なモノリスを安全に解体する*

- [x] 3.1 `src/cli.rs` の責務を明確に定義し、`cli` モジュール下でサブモジュール化する。変更は一度に行わず、機能ごと（`args`, `input`, `workflow` 等）に漸進的にコミットし、常に `cargo check` が通る状態を維持する
- [x] 3.2 既存の CLI 契約テスト (`cli_convergence_contract`, `cli_path_context_contract`) が100%通過することを確認する
- [x] 3.3 肥大化ファイル（`src/upstream.rs` 等）のモジュール分割または構造体の責務再定義を実施する
- [x] 3.4 **[重要]** リファクタリング後、`make internal-quality-check` を実行し、`src/cli.rs` のサイズスコアが**例外なく 200 以下** になっていることを確認する
- [x] 3.5 **[退行検証]** アーキテクチャ変更によって性能が劣化していないことを証明するため、この時点で一度 `make perf-check-strict` を実行し通過させる

## 4. Performance Optimization (Phase 4)

*目的: 整理されたアーキテクチャの上で、ミクロな性能最適化を施す*

- [x] 4.1 ホットパス（AST評価・ルールの適用時など）での不要な `String::clone`, `to_string()`, `to_owned()` を特定し、参照借用（`&str`）に置き換える（severity_map を `HashMap<&str,...>` に変更）
- [x] 4.1b **[精度修正]** MD003 誤検知修正：setext heading 診断をunderlineではなくheading textの行（i-1）に修正
- [x] 4.1c **[精度修正]** MD046 誤検知修正：4スペースインデントのリストアイテム（`-`, `*`, `+`, ordered）をindented code blockとして誤判定しないよう `is_list_marker_line` を追加
- [x] 4.2 複数要素を返す処理での中間 `Vec` 生成 (`collect::<Vec<_>>()`) を削減し、イテレータのまま処理できるよう改修する（ライフタイム複雑化による保守性低下を招く場合は見送る）
  - `heading_style.rs`, `style.rs`, `heading_duplicates.rs` の `Vec<&str>` → `ctx.lines()` 直参照 (`&[LineInfo<'_>]`) に変更
- [x] 4.3 再度 `make perf-check-strict` を実行し、Phase 3 時点（または v0.12.11 時点）と比較して性能がさらに向上、または同等であることを確認する
- [x] 4.4 明確な改善根拠とともに `make perf-refresh-baseline` を実行し、新ベースラインを確立する

## 5. Quality Scoring (Phase 5)

- [ ] 5.1 作成済みの `public-confidence-score.json` テンプレートを開く
- [ ] 5.2 カテゴリ (`External corpus confidence`, `Precision regression`, `Command convergence`, `Performance stability`, `Release reproducibility`) の検証結果・根拠を記載する
- [ ] 5.3 合否ルール（`score >= 90` かつ `technical_hard_blockers` 0 件）の達成を確認し、残存課題があれば `known_limitations` に追記する

## Verification

- [x] `make ast-lint`
- [x] `cargo test --all-features --locked`
- [x] `cargo test --test public_confidence_contract --locked`
- [x] `cargo test --test cli_convergence_contract --locked`
- [x] `cargo test --test cli_path_context_contract --locked`
- [x] `make perf-check-strict`
- [x] `make public-confidence`
- [x] `make internal-quality-check`
- [x] `public-confidence-score.json` の作成・反映
- [x] `make coverage-blocking`
- [x] `make release-check VERSION=v0.12.12`

## Definition of Done

- [x] `md-broken-link` が適切に無効化され、外部コーパス検証のノイズが除去されている
- [x] `tests/ast_linter.rs` が汎用化され、`src/` 配下全体に加え `tests/` や `build.rs` のルールとしても機能している
- [x] `src/cli.rs` が責務ごとに分割され、サイズスコアが例外なく 200 以下になっている
- [x] リファクタリングによって新たな `internal-quality` 指摘（技術的負債）が発生していないこと
- [x] パフォーマンス向上施策が適用され、`make perf-check-strict` が通過した上で新ベースラインが更新されている
- [x] 全ての精密性・収束性テスト (precision / convergence test) が引き続きパスし、精度退行がないことが証明されている
- [x] public-confidence-score が作成され、リリース基準（90点以上、ブロッカなし）を満たしている
- [x] `make release-check` がエラーなく通過する
