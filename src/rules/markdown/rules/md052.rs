use crate::rules::markdown::document::{LineInfo, SourceRange};
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
    RuleConfig,
};
use std::collections::HashSet;
use std::path::Path;

/// MD052 / reference-links-images — Reference links and images.
pub struct ReferenceLinksImagesRule;

impl MarkdownRule for ReferenceLinksImagesRule {
    fn id(&self) -> &'static str {
        "MD052"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD052")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let ctx = DocumentContext::new(file_path, content);
        self.evaluate_context(&ctx, None)
    }

    fn evaluate_context(
        &self,
        ctx: &DocumentContext<'_>,
        _config: Option<&RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD052");
        let definitions = reference_definition_labels(ctx);
        let ignored_labels = ignored_labels(_config);
        let include_shortcut = shortcut_syntax(_config);
        let indented_code_lines = indented_code_line_indexes(ctx);
        let mut diagnostics = Vec::new();
        for link in ctx.inline_links() {
            if !link.kind.is_reference() {
                continue;
            }
            if indented_code_lines.contains(&link.line) {
                continue;
            }
            let Some(label) = link.effective_label() else {
                continue;
            };
            if should_report_label(label, &definitions, &ignored_labels) {
                diagnostics.push(diagnostic_for_range(ctx, &meta, link.full_range));
            }
        }
        if include_shortcut {
            for shortcut in shortcut_references(ctx, &indented_code_lines) {
                if should_report_label(shortcut.label, &definitions, &ignored_labels) {
                    diagnostics.push(diagnostic_for_range(ctx, &meta, shortcut.range));
                }
            }
        }
        diagnostics
    }
}

struct ShortcutReference<'a> {
    label: &'a str,
    range: SourceRange,
}

fn reference_definition_labels(ctx: &DocumentContext<'_>) -> HashSet<String> {
    ctx.reference_definitions()
        .iter()
        .map(|definition| normalize_label(definition.label))
        .collect()
}

fn ignored_labels(config: Option<&RuleConfig>) -> HashSet<String> {
    config
        .and_then(|config| config.properties.get("ignored_labels"))
        .and_then(|raw| serde_json::from_str::<Vec<String>>(raw).ok())
        .unwrap_or_else(|| vec!["x".to_string()])
        .into_iter()
        .map(|label| normalize_label(&label))
        .collect()
}

fn shortcut_syntax(config: Option<&RuleConfig>) -> bool {
    config
        .and_then(|config| config.properties.get("shortcut_syntax"))
        .and_then(|value| value.parse::<bool>().ok())
        .unwrap_or(false)
}

fn should_report_label(
    label: &str,
    definitions: &HashSet<String>,
    ignored_labels: &HashSet<String>,
) -> bool {
    let normalized = normalize_label(label);
    !normalized.is_empty()
        && !definitions.contains(&normalized)
        && !ignored_labels.contains(&normalized)
}

fn indented_code_line_indexes(ctx: &DocumentContext<'_>) -> HashSet<usize> {
    let mut indexes = HashSet::new();
    let mut in_block = false;
    for (line_index, line) in ctx.lines().iter().enumerate() {
        if ctx.is_code_line(line_index) {
            in_block = false;
            continue;
        }
        if line.text.trim().is_empty() {
            continue;
        }
        if !line.text.starts_with("    ") {
            in_block = false;
            continue;
        }
        if is_list_marker_line(&line.text[4..]) || is_definition_list_continuation(ctx, line_index)
        {
            in_block = false;
            continue;
        }
        if in_block || previous_line_allows_indented_code(ctx, line_index) {
            indexes.insert(line_index);
            in_block = true;
        } else {
            in_block = false;
        }
    }
    indexes
}

fn previous_line_allows_indented_code(ctx: &DocumentContext<'_>, line_index: usize) -> bool {
    line_index == 0 || ctx.lines()[line_index - 1].text.trim().is_empty()
}

fn is_definition_list_continuation(ctx: &DocumentContext<'_>, line_index: usize) -> bool {
    if line_index == 0 {
        return false;
    }
    for previous_index in (0..line_index).rev() {
        let previous = ctx.lines()[previous_index].text;
        if previous.trim().is_empty()
            || previous.starts_with("    ")
            || ctx.is_code_line(previous_index)
        {
            continue;
        }
        return previous.trim_start().starts_with(':');
    }
    false
}

fn is_list_marker_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.starts_with("- ")
        || trimmed.starts_with("* ")
        || trimmed.starts_with("+ ")
        || trimmed == "-"
        || trimmed == "*"
        || trimmed == "+"
    {
        return true;
    }
    let digit_count = trimmed
        .bytes()
        .take_while(|byte| byte.is_ascii_digit())
        .count();
    digit_count > 0
        && trimmed
            .get(digit_count..)
            .is_some_and(|rest| rest.starts_with(". ") || rest.starts_with(") "))
}

fn normalize_label(label: &str) -> String {
    label
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

fn diagnostic_for_range(
    ctx: &DocumentContext<'_>,
    meta: &OfficialRuleMeta,
    source_range: SourceRange,
) -> MarkdownDiagnostic {
    MarkdownDiagnostic {
        file: ctx.file_path().to_path_buf(),
        severity: DiagnosticSeverity::Warning,
        range: ctx.diagnostic_range(source_range),
        message: meta.description.to_string(),
        rule_id: meta.code.to_string(),
        official_meta: Some(meta.clone()),
        fix_info: None,
    }
}

fn shortcut_references<'a>(
    ctx: &DocumentContext<'a>,
    indented_code_lines: &HashSet<usize>,
) -> Vec<ShortcutReference<'a>> {
    let definition_lines = ctx
        .reference_definitions()
        .iter()
        .map(|definition| definition.line)
        .collect::<HashSet<_>>();
    let mut references = Vec::new();
    for (line_index, line) in ctx.lines().iter().enumerate() {
        if ctx.is_code_line(line_index)
            || indented_code_lines.contains(&line_index)
            || definition_lines.contains(&line_index)
        {
            continue;
        }
        references.extend(shortcut_references_on_line(ctx, line));
    }
    references
}

fn shortcut_references_on_line<'a>(
    ctx: &DocumentContext<'_>,
    line: &LineInfo<'a>,
) -> Vec<ShortcutReference<'a>> {
    let bytes = line.text.as_bytes();
    let mut references = Vec::new();
    let mut cursor = 0usize;
    while cursor < bytes.len() {
        if bytes[cursor] == b'\\' {
            cursor += 2;
            continue;
        }
        if bytes[cursor] == b'[' && cursor > 0 && bytes[cursor - 1] == b']' {
            cursor += 1;
            continue;
        }
        let (full_start, label_open) = match bytes[cursor] {
            b'!' if bytes.get(cursor + 1) == Some(&b'[') => (cursor, cursor + 1),
            b'[' => (cursor, cursor),
            _ => {
                cursor += 1;
                continue;
            }
        };
        let Some(label_close) = matching_bracket(line.text, label_open) else {
            cursor = label_open + 1;
            continue;
        };
        let after_label = label_close + 1;
        if matches!(bytes.get(after_label), Some(b'(' | b'[' | b':')) {
            cursor = after_label;
            continue;
        }
        let label_start = label_open + 1;
        if label_start == label_close {
            cursor = after_label;
            continue;
        }
        let range = SourceRange {
            start: line.content_range.start + full_start,
            end: line.content_range.start + after_label,
        };
        if !ctx.is_inside_inline_code(range) {
            references.push(ShortcutReference {
                label: &line.text[label_start..label_close],
                range,
            });
        }
        cursor = after_label;
    }
    references
}

fn matching_bracket(line: &str, open_bracket: usize) -> Option<usize> {
    let bytes = line.as_bytes();
    let mut cursor = open_bracket + 1;
    let mut depth = 1usize;
    while cursor < bytes.len() {
        if bytes[cursor] == b'\\' {
            cursor += 2;
            continue;
        }
        match bytes[cursor] {
            b'[' => depth += 1,
            b']' => {
                depth -= 1;
                if depth == 0 {
                    return Some(cursor);
                }
            }
            _ => {}
        }
        cursor += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_collapsed_reference_links_and_images() {
        let rule = ReferenceLinksImagesRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "[ref][] and ![alt][]\n");

        assert_eq!(diagnostics.len(), 2);
        assert!(diagnostics
            .iter()
            .all(|diagnostic| diagnostic.rule_id == "MD052"));
    }

    #[test]
    fn ignores_full_references_and_code_spans() {
        let rule = ReferenceLinksImagesRule;
        let content = concat!(
            "[ref][label] and ![alt][image]\n",
            "`[ref][]`\n",
            "[label]: https://example.com\n",
            "[image]: https://example.org/image.png\n",
        );
        let diagnostics = rule.evaluate(Path::new("doc.md"), content);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn reports_missing_full_references() {
        let rule = ReferenceLinksImagesRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "[ref][missing]\n");

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].fix_info.is_none());
    }

    #[test]
    fn ignores_defined_collapsed_reference() {
        let rule = ReferenceLinksImagesRule;
        let diagnostics = rule.evaluate(
            Path::new("doc.md"),
            "[ref][]\n\n[ref]: https://example.com\n",
        );

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn is_not_fixable_in_catalog() {
        let rule = ReferenceLinksImagesRule;
        let meta = rule.official_meta().expect("meta must be Some");
        assert!(!meta.is_fixable, "MD052 must not advertise a safe fix");
    }
}
