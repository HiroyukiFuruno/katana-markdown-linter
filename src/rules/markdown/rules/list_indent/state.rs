use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, OfficialRuleMeta,
};

#[derive(Default)]
pub(super) struct ListParents {
    ordered: Vec<OrderedParentIndent>,
    unordered: Vec<UnorderedParentIndent>,
}

impl ListParents {
    pub(super) fn check_line(
        &mut self,
        ctx: &DocumentContext<'_>,
        i: usize,
        line: &str,
        indent: usize,
        meta: &OfficialRuleMeta,
        diagnostics: &mut Vec<MarkdownDiagnostic>,
    ) {
        let trimmed = line.trim_start();
        let leading = line.len() - trimmed.len();
        if ctx.is_literal_block_line(i) {
            self.observe_literal(trimmed, leading);
            return;
        }
        if RuleHelpers::get_ordered_number(trimmed).is_some() {
            self.observe_ordered(trimmed, leading);
            return;
        }
        if RuleHelpers::get_bullet_char(trimmed).is_some() {
            let expected = self.observe_unordered(leading, indent);
            if let Some(expected) = expected.filter(|expected| leading != *expected) {
                diagnostics.push(indent_diagnostic(ctx, i, line, leading, expected, meta));
            }
        } else if !trimmed.is_empty() {
            self.ordered.retain(|parent| parent.content <= leading);
            self.unordered.retain(|parent| parent.actual < leading);
        }
    }

    fn observe_literal(&mut self, trimmed: &str, leading: usize) {
        if !trimmed.is_empty() && !self.ordered.iter().any(|parent| leading >= parent.content) {
            self.ordered.clear();
        }
    }

    fn observe_ordered(&mut self, trimmed: &str, leading: usize) {
        self.ordered.retain(|parent| parent.content <= leading);
        self.unordered.retain(|parent| parent.actual <= leading);
        self.ordered.push(OrderedParentIndent {
            actual: leading,
            content: leading + ordered_marker_width(trimmed),
        });
    }

    fn observe_unordered(&mut self, leading: usize, indent: usize) -> Option<usize> {
        self.ordered.retain(|parent| parent.actual < leading);
        let expected = expected_indent(
            leading,
            indent,
            nearest_unordered_parent(leading, &self.unordered),
            nearest_ordered_parent(leading, &self.ordered),
        );
        self.unordered.retain(|parent| parent.actual <= leading);
        if !self.unordered.iter().any(|parent| parent.actual == leading) {
            self.unordered.push(UnorderedParentIndent {
                actual: leading,
                expected: expected.unwrap_or(leading),
            });
        }
        expected
    }
}

fn indent_diagnostic(
    ctx: &DocumentContext<'_>,
    i: usize,
    line: &str,
    leading: usize,
    expected: usize,
    meta: &OfficialRuleMeta,
) -> MarkdownDiagnostic {
    let fix = crate::rules::markdown::types::DiagnosticFix {
        start_line: i + 1,
        start_column: 1,
        end_line: i + 1,
        end_column: leading.saturating_add(1),
        replacement: " ".repeat(expected),
    };
    MarkdownDiagnostic {
        file: ctx.file_path().to_path_buf(),
        severity: DiagnosticSeverity::Warning,
        range: crate::rules::markdown::DiagnosticRange {
            start_line: i + 1,
            start_column: 1,
            end_line: i + 1,
            end_column: line.len().max(1),
        },
        message: format!(
            "{} [Expected: {}, Actual: {}]",
            meta.description, expected, leading
        ),
        rule_id: meta.code.to_string(),
        official_meta: Some(meta.clone()),
        fix_info: Some(fix),
    }
}

fn ordered_marker_width(trimmed: &str) -> usize {
    trimmed.find('.').map_or(0, |dot_pos| dot_pos + 2)
}

#[derive(Clone, Copy)]
struct UnorderedParentIndent {
    actual: usize,
    expected: usize,
}

#[derive(Clone, Copy)]
struct OrderedParentIndent {
    actual: usize,
    content: usize,
}

fn expected_indent(
    leading: usize,
    indent: usize,
    unordered_parent: Option<UnorderedParentIndent>,
    ordered_parent: Option<OrderedParentIndent>,
) -> Option<usize> {
    match (unordered_parent, ordered_parent) {
        (Some(unordered), Some(ordered)) if unordered.actual > ordered.actual => {
            Some(unordered.expected + indent)
        }
        (Some(unordered), None) => Some(unordered.expected + indent),
        (_, Some(ordered)) if leading < ordered.content => Some(ordered.content),
        (_, Some(_)) => None,
        (None, None) => Some(0),
    }
}

fn nearest_ordered_parent(
    leading: usize,
    ordered_parent_indents: &[OrderedParentIndent],
) -> Option<OrderedParentIndent> {
    ordered_parent_indents
        .iter()
        .copied()
        .filter(|list_indent| list_indent.actual < leading)
        .max_by_key(|list_indent| list_indent.actual)
}

fn nearest_unordered_parent(
    leading: usize,
    unordered_parent_indents: &[UnorderedParentIndent],
) -> Option<UnorderedParentIndent> {
    unordered_parent_indents
        .iter()
        .copied()
        .filter(|list_indent| list_indent.actual < leading)
        .max_by_key(|list_indent| list_indent.actual)
}
