## Context

現在の i18n は English / Japanese のみを supported locale として扱う。
短い説明文（description）は `localized_rule_description`、localized catalog、CLI `kml rule`、MCP `rule_list` / `rule_get` で返る。
ルール本文 Markdown は `get_rule_documentation` と MCP `rule_doc_get` で返るが、localize 先は English / Japanese に限定されている。

KatanA 本体の locale は `en`, `ja`, `zh-CN`, `zh-TW`, `ko`, `pt`, `fr`, `de`, `es`, `it` である。
v0.16.1 ではこの集合へ合わせる。ただし kml は generic library なので、KatanA 固有の UI 文言や設定は持ち込まない。

## Goals / Non-Goals

**Goals:**

- KatanA 本体と同じ 10 言語を kml の supported locale にする。
- description 系 API と rule document Markdown API の両方で localized content を返す。
- English の単純コピーを翻訳欠落として検出する。
- CLI、Rust API、local / remote MCP で同じ locale resolver と fallback policy を使う。
- 翻訳は各言語で自然な文として作る。機械的な English コピーや placeholder は許容しない。

**Non-Goals:**

- KatanA 固有の画面文言を kml に持ち込むこと。
- Diagnostic message の全 rule 固有化をこの patch に含めること。
- Upstream markdownlint 文書の全段落を逐語訳すること。
- `Locale` 以外の public API を破壊的に変えること。

## Decisions

### Locale source

Supported locale は kml 側の `Locale` enum と `supported_locales()` に固定し、KatanA の `languages.json` から runtime で読む構成にはしない。
理由は、kml が standalone library / CLI / MCP として動くためである。
ただし対象集合は KatanA 本体と同じ 10 言語に揃える。

代替案として KatanA の locale file を参照する方法もあるが、repository boundary を越えて release artifact の再現性が落ちるため採用しない。

### Translation storage

短い rule description は Rust-native catalog を維持する。
ただし `src/i18n.rs` に多言語テーブルを集中させず、locale / description / message rendering を責務単位で分割する。
既存ファイルはすでに大きいため、v0.16.1 の追加でさらに肥大化させない。

Rule document Markdown は `upstream_docs/<locale>/md*.md` を優先して読む。
Supported locale の document が欠けている場合は、テストと AST lint が失敗する。
Runtime fallback は unsupported locale だけ English に戻す。

### Translation quality gate

Translation coverage は「存在する」だけではなく、English の単純コピーを failure とする。
判定対象は説明文と rule document Markdown の通常本文で、rule ID、config key、alias、URL、code fence 内の example など翻訳すべきでない token は除外する。

Latin 系言語では English と語彙が一部重なるため、全文一致や主要 prose の一致を禁止対象にする。
日本語・中国語・韓国語は script-specific text を含むことも確認する。

### API behavior

CLI explicit locale は既存通り strict とし、unsupported value は error にする。
Library helper と MCP locale parameter は既存通り lenient とし、unsupported value は English に fallback する。
Supported locale で翻訳が欠けた場合は、開発時 gate で検出する設計とし、runtime の silent English fallback にはしない。

### Documentation

README と MCP documentation は English で更新する。
OpenSpec artifact は作業用 artifact として日本語で記述する。

## Risks / Trade-offs

- Risk: 翻訳量が多く、表現の不自然さが残る可能性がある → 各言語の文として読める短い文へ寄せ、English コピー検査で最低品質を固定する。
- Risk: Rule document を逐語訳しないことで upstream 文書との差分が出る → kml の rule help として必要な意味、設定 key、例、rationale を保ち、upstream drift check とは別責務にする。
- Risk: `Locale` variant 追加で内部 match が増える → wildcard ではなく compiler に列挙漏れを検出させ、coverage gate で supported locale 全体を走査する。
