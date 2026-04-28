# Design for v0.12.14 Precision & Performance Hardening

## 1. 根本負債の特定

初期 OpenSpec 草稿では「4 つの OnceLock 走査の統合」「MD051 regex キャッシュ」「MD046 code_line_flags 置換」を候補としていた。しかしコードベースを精査した結果、これらは以下の理由で誤診断または非効果だった:

- **MD046** はすでに `ctx.is_code_line(idx)` (O(1) `code_line_flags` 索引) を使っている
- **MD051 regex** は unit struct rule で per-file 1 回コンパイル。規模は問題にならない
- **OnceLock 統合** は遅延初期化の依存グラフを壊すリスクがあり、効果も限定的

実際の根本負債は `src/rules/markdown/inline/` 内部に存在する。

---

## 2. F-A: `line_in_blocks()` → `code_line_flags` 直接参照

### 現状

`src/rules/markdown/inline/scan.rs:43-47`:
```rust
pub(super) fn line_in_blocks(line_index: usize, blocks: &[BlockRange]) -> bool {
    blocks.iter().any(|block| (block.start_line..=block.end_line).contains(&line_index))
}
```

全 4 extractor がライン毎にこれを呼ぶ。`b` 個の code block に対して O(L×b) の全量を線形走査。
一方 `DocumentContext` はすでに `build_code_line_flags` でこれを O(L) で事前計算しており、`is_code_line(idx)` で O(1) 参照を提供している。

### 修正

`extract_inline_code_spans` / `extract_inline_html_elements` / `extract_inline_links` / `extract_reference_definitions` の引数から `code_blocks: &[BlockRange]` を除去し、`code_line_flags: &[bool]` に置き換える。

```rust
// 変更前
if line_in_blocks(idx, code_blocks) { continue; }

// 変更後
if *code_line_flags.get(idx).unwrap_or(&false) { continue; }
```

`document.rs` の各 OnceLock 初期化コードから `&self.code_blocks` を `&self.code_line_flags` に変更する。`scan.rs` の `line_in_blocks` は pub(super) なので削除可能（もし他から使われていなければ）。

計算量: O(L×b) → O(L)（L: 行数、b: code block 数）

---

## 3. F-B: `inside_code_span()` → `partition_point` 二分探索

### 現状

`src/rules/markdown/inline/scan.rs:33-41`:
```rust
pub(super) fn inside_code_span(code_spans: &[InlineCodeSpan], line_index: usize, offset: usize) -> bool {
    code_spans.iter().any(|span| {
        span.line == line_index && span.full_range.start <= offset && offset < span.full_range.end
    })
}
```

`html_elements_on_line` と `next_link_open` の内側（キャラクタ位置ごとのループ）から呼ばれる。
O(s) per call（s: code span 数）。dense inline doc では 文字数 × code span 数 = O(L×C×s) になる。

### 修正

`DocumentContext::is_inside_inline_code` と同じ `partition_point` アプローチ（document.rs:220-228 が正解実装）:

```rust
pub(super) fn inside_code_span(code_spans: &[InlineCodeSpan], _line_index: usize, offset: usize) -> bool {
    // code_spans は full_range.start でソート済み（行順・左→右順に抽出されるため）
    let idx = code_spans.partition_point(|span| span.full_range.start <= offset);
    // offset を含む可能性があるのは idx-1 のスパンのみ
    idx > 0 && {
        let span = &code_spans[idx - 1];
        offset < span.full_range.end  // start <= offset は partition_point の定義から保証
    }
}
```

`line_index` は不要になる（byte offset がグローバルに一意なため）。パラメータは互換性のため残すか、呼び出し元を合わせて削除する。

計算量: O(s) → O(log s)

---

## 4. F-C: `"`".repeat(marker_len)` String アロケーション除去

### 現状

`src/rules/markdown/inline/code_spans.rs:34`:
```rust
let marker = "`".repeat(marker_len);
let Some(close_relative) = line.text[content_start..].find(&marker) else { ... };
```

closing backtick sequence を探すために `String` を毎回確保している。

### 修正

`find(&marker)` の代わりに、バイト列を直接走査して marker_len 個の連続バッククォートを探す専用 helper を使う:

```rust
fn find_closing_marker(text: &str, start: usize, marker_len: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut cursor = start;
    while cursor < bytes.len() {
        if bytes[cursor] != b'`' {
            cursor += 1;
            continue;
        }
        let run = bytes[cursor..].iter().take_while(|&&b| b == b'`').count();
        if run == marker_len {
            return Some(cursor - start); // relative to start, matching find() semantics
        }
        cursor += run;
    }
    None
}
```

`String` アロケーションゼロ。関数シグネチャは `find(&marker)` → `find_closing_marker(text, start, marker_len)` の内部変更のみで外部 API 不変。

---

## 5. 等価性ゲート

3 つの変更はいずれも抽出ロジックの**出力を変えない**:
- F-A: `line_in_blocks` と `code_line_flags[idx]` は同じ真偽値
- F-B: `partition_point` と `iter().any()` は同じ真偽値（コードスパンがソート済みである前提）
- F-C: `find(&marker)` と `find_closing_marker` は同じ相対オフセット

`tests/ast_linter.rs` 全テスト + `make public-confidence` で diagnostic の完全一致を証明する。

---

## 6. ロードマップ更新

- `active-roadmap.md` に v0.12.14 行を追加（In Progress → Done で更新）
- 精度 fix+ 候補 (MD052/MD046/MD043/MD056/MD034/MD051 fragment) を v0.12.15+ エントリとして登録
- MD013 wrap fix は unsafe-fix mode の独立 change (`v0.13.x`) として Deferred セクションに追加
