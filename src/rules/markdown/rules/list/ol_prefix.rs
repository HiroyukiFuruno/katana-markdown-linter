use super::ordered_state::{
    is_ordered_list_continuation, is_ordered_list_lazy_continuation, ordered_marker_width,
    OrderedListState, OrderedListStyle,
};
use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
    RuleParityStatus,
};
use crate::types::RuleConfig;
use std::collections::BTreeMap;
use std::path::Path;

pub struct OlPrefixRule;

struct OrderedItem<'a> {
    index: usize,
    line_text: &'a str,
    indent: usize,
    trimmed: &'a str,
    number: u32,
}

impl MarkdownRule for OlPrefixRule {
    fn id(&self) -> &'static str {
        "MD029"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD029",
            title: "ol-prefix",
            description: "Ordered list item prefix.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md029.md",
            aliases: &["ol-prefix"],
            parity: RuleParityStatus::Official,
            is_fixable: true,
            properties: &[crate::rule_prop_enum!(
                "style",
                "List style",
                "one_or_ordered",
                &["one", "ordered", "one_or_ordered", "zero"]
            )],
        })
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let ctx = DocumentContext::new(file_path, content);
        self.evaluate_context(&ctx, None)
    }

    fn evaluate_context(
        &self,
        ctx: &DocumentContext<'_>,
        config: Option<&RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD029");
        let mut diagnostics = Vec::new();
        let mut expected_numbers = BTreeMap::<usize, OrderedListState>::new();
        let style = OrderedListStyle::from_config(config);
        for (index, line) in ctx.lines().iter().enumerate() {
            evaluate_line(
                &mut diagnostics,
                &mut expected_numbers,
                ctx,
                index,
                line.text,
                &meta,
                style,
            );
        }
        diagnostics
    }
}

fn evaluate_line(
    diagnostics: &mut Vec<MarkdownDiagnostic>,
    expected_numbers: &mut BTreeMap<usize, OrderedListState>,
    ctx: &DocumentContext<'_>,
    index: usize,
    line_text: &str,
    meta: &OfficialRuleMeta,
    style: OrderedListStyle,
) {
    let trimmed = line_text.trim_start();
    let indent = line_text.len() - trimmed.len();
    if ctx.is_code_line(index) {
        clear_non_continuation(trimmed, indent, expected_numbers);
        return;
    }
    if let Some(number) = RuleHelpers::get_ordered_number(trimmed) {
        let item = OrderedItem {
            index,
            line_text,
            indent,
            trimmed,
            number,
        };
        evaluate_ordered_item(diagnostics, expected_numbers, ctx, meta, style, item);
        return;
    }
    if !trimmed.is_empty() && !is_ordered_list_lazy_continuation(indent, trimmed, expected_numbers)
    {
        expected_numbers.retain(|level, _| *level < indent);
    }
}

fn clear_non_continuation(
    trimmed: &str,
    indent: usize,
    expected_numbers: &mut BTreeMap<usize, OrderedListState>,
) {
    if !trimmed.is_empty() && !is_ordered_list_continuation(indent, expected_numbers) {
        expected_numbers.clear();
    }
}

fn evaluate_ordered_item(
    diagnostics: &mut Vec<MarkdownDiagnostic>,
    expected_numbers: &mut BTreeMap<usize, OrderedListState>,
    ctx: &DocumentContext<'_>,
    meta: &OfficialRuleMeta,
    style: OrderedListStyle,
    item: OrderedItem<'_>,
) {
    expected_numbers.retain(|level, _| *level <= item.indent);
    let state = expected_numbers.entry(item.indent).or_default();
    let expected = state.expected_number(item.number, style);
    if let Some(expected_number) = expected {
        push_ordered_prefix_diagnostic(
            diagnostics,
            ctx,
            item.index,
            item.line_text,
            meta,
            expected_number,
        );
    }
    state.content_indent = item.indent + ordered_marker_width(item.trimmed);
}

fn push_ordered_prefix_diagnostic(
    diagnostics: &mut Vec<MarkdownDiagnostic>,
    ctx: &DocumentContext<'_>,
    index: usize,
    line_text: &str,
    meta: &OfficialRuleMeta,
    expected_number: u32,
) {
    let dot_pos = line_text.find(". ").expect("ordered marker contains dot");
    let start_col = line_text
        .find(|ch: char| ch.is_ascii_digit())
        .expect("ordered marker contains digit");
    let fix = crate::rules::markdown::types::DiagnosticFix {
        start_line: index + 1,
        start_column: start_col + 1,
        end_line: index + 1,
        end_column: dot_pos + 1,
        replacement: expected_number.to_string(),
    };
    RuleHelpers::push_diag_with_fix(
        diagnostics,
        ctx.file_path(),
        index,
        line_text,
        meta,
        DiagnosticSeverity::Warning,
        Some(fix),
    );
}
