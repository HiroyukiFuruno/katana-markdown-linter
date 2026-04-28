use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD007 / ul-indent — Unordered list indentation.
pub struct UnorderedListIndentRule;

impl MarkdownRule for UnorderedListIndentRule {
    fn id(&self) -> &'static str {
        "MD007"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD007")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD007");
        let mut diagnostics = Vec::new();
        let indent = 2;
        let mut ordered_parent_indents = Vec::<OrderedParentIndent>::new();
        let mut unordered_parent_indents = Vec::<UnorderedParentIndent>::new();

        let ctx = DocumentContext::new(file_path, content);
        for (i, line) in ctx.lines().iter().enumerate() {
            let line = line.text;
            let trimmed = line.trim_start();
            let leading = line.len() - trimmed.len();
            if ctx.is_code_line(i) {
                if !trimmed.is_empty()
                    && !ordered_parent_indents
                        .iter()
                        .any(|list_indent| leading >= list_indent.content)
                {
                    ordered_parent_indents.clear();
                }
                continue;
            }

            if RuleHelpers::get_ordered_number(trimmed).is_some() {
                ordered_parent_indents.retain(|list_indent| list_indent.content <= leading);
                unordered_parent_indents.retain(|list_indent| list_indent.actual <= leading);
                ordered_parent_indents.push(OrderedParentIndent {
                    actual: leading,
                    content: leading + ordered_marker_width(trimmed),
                });
                continue;
            }
            if RuleHelpers::get_bullet_char(trimmed).is_some() {
                ordered_parent_indents.retain(|list_indent| list_indent.actual < leading);
                let unordered_parent = nearest_unordered_parent(leading, &unordered_parent_indents);
                let ordered_parent = nearest_ordered_parent(leading, &ordered_parent_indents);
                let expected_indent =
                    expected_indent(leading, indent, unordered_parent, ordered_parent);
                if let Some(expected_indent) = expected_indent.filter(|it| leading != *it) {
                    let fix = crate::rules::markdown::types::DiagnosticFix {
                        start_line: i + 1,
                        start_column: 1,
                        end_line: i + 1,
                        end_column: leading.saturating_add(1),
                        replacement: " ".repeat(expected_indent),
                    };
                    RuleHelpers::push_diag_with_fix(
                        &mut diagnostics,
                        file_path,
                        i,
                        line,
                        &meta,
                        DiagnosticSeverity::Warning,
                        Some(fix),
                    );
                }
                let parent_expected = expected_indent.unwrap_or(leading);
                unordered_parent_indents.retain(|list_indent| list_indent.actual <= leading);
                if !unordered_parent_indents
                    .iter()
                    .any(|list_indent| list_indent.actual == leading)
                {
                    unordered_parent_indents.push(UnorderedParentIndent {
                        actual: leading,
                        expected: parent_expected,
                    });
                }
            } else if !trimmed.is_empty() {
                ordered_parent_indents.retain(|list_indent| list_indent.content <= leading);
                unordered_parent_indents.retain(|list_indent| list_indent.actual < leading);
            }
        }

        diagnostics
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
