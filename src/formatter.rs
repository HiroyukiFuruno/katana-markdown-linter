use crate::{fix_with_results, lint, Error, FixResult, LintOptions, RuleConfig};
use serde::Serialize;
use std::collections::{HashMap, HashSet};

const MAX_FORMAT_PASSES: usize = 8;
const LAYOUT_RULES: [&str; 9] = [
    "MD005", "MD007", "MD012", "MD022", "MD030", "MD031", "MD032", "MD047", "MD058",
];

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct FormatOptions {
    pub max_passes: usize,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            max_passes: MAX_FORMAT_PASSES,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub struct FormatResult {
    pub content: String,
    pub applied_operations: usize,
}

/// Applies deterministic Markdown layout formatting.
///
/// The formatter intentionally uses a narrow layout-only policy. It normalizes line endings and
/// then applies safe fixes for the formatter rule subset; it does not apply semantic/style rewrites.
pub fn format_markdown(content: &str, options: &FormatOptions) -> Result<FormatResult, Error> {
    let mut content = content.to_string();
    let mut applied_operations = 0;
    let normalized = normalize_line_endings(&content);
    if normalized != content {
        applied_operations += 1;
        content = normalized;
    }
    let terminal_normalized = normalize_terminal_newline(&content);
    if terminal_normalized != content {
        applied_operations += 1;
        content = terminal_normalized;
    }

    let lint_options = layout_lint_options();
    let max_passes = options.max_passes.max(1);
    for _ in 0..max_passes {
        let diagnostics = lint(&content, &lint_options)?;
        if !diagnostics
            .iter()
            .any(|diagnostic| diagnostic.fix.is_some())
        {
            break;
        }

        let fixed: FixResult = fix_with_results(&content, &diagnostics);
        if fixed.applied_fixes == 0 || fixed.content == content {
            break;
        }

        applied_operations += fixed.applied_fixes;
        content = fixed.content;
    }

    let terminal_normalized = normalize_terminal_newline(&content);
    if terminal_normalized != content {
        applied_operations += 1;
        content = terminal_normalized;
    }

    Ok(FormatResult {
        content,
        applied_operations,
    })
}

pub fn layout_lint_options() -> LintOptions {
    let layout_rules = LAYOUT_RULES.into_iter().collect::<HashSet<_>>();
    LintOptions {
        default_severity: crate::Severity::Warning,
        rules: crate::rules::markdown::MarkdownLinterOps::official_rules()
            .iter()
            .map(|rule_id| {
                (
                    rule_id.id().to_string(),
                    RuleConfig {
                        enabled: layout_rules.contains(rule_id.id()),
                        properties: HashMap::new(),
                    },
                )
            })
            .collect(),
    }
}

fn normalize_line_endings(content: &str) -> String {
    if !content.as_bytes().contains(&b'\r') {
        return content.to_string();
    }

    let mut normalized = String::with_capacity(content.len());
    let mut chars = content.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\r' {
            if chars.peek() == Some(&'\n') {
                chars.next();
            }
            normalized.push('\n');
        } else {
            normalized.push(ch);
        }
    }
    normalized
}

fn normalize_terminal_newline(content: &str) -> String {
    if content.is_empty() {
        return String::new();
    }
    let trimmed = content.trim_end_matches('\n');
    format!("{trimmed}\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formatter_normalizes_line_endings_and_final_newline() {
        let formatted = format_markdown("# Title\r\nText\r", &FormatOptions::default())
            .expect("format should succeed");

        assert_eq!(formatted.content, "# Title\n\nText\n");
        assert!(formatted.applied_operations >= 2);
    }

    #[test]
    fn formatter_applies_layout_subset_without_semantic_style_rewrites() {
        let source =
            "# Title\nText\n\n\n## Next\n```rust\ncode\n```\n| A | B |\n|---|---|\n| 1 | 2 |";
        let formatted =
            format_markdown(source, &FormatOptions::default()).expect("format should succeed");

        assert_eq!(
            formatted.content,
            "# Title\n\nText\n\n## Next\n\n```rust\ncode\n```\n\n| A | B |\n|---|---|\n| 1 | 2 |\n"
        );
    }

    #[test]
    fn formatter_is_idempotent() {
        let source = "# Title\nText\n\n\n-  item\n";
        let first = format_markdown(source, &FormatOptions::default())
            .expect("first format should succeed");
        let second = format_markdown(&first.content, &FormatOptions::default())
            .expect("second format should succeed");

        assert_eq!(first.content, second.content);
        assert_eq!(second.applied_operations, 0);
    }

    #[test]
    fn formatter_does_not_remove_trailing_spaces() {
        let formatted = format_markdown("hard break  \nnext\n", &FormatOptions::default())
            .expect("format should succeed");

        assert_eq!(formatted.content, "hard break  \nnext\n");
    }

    #[test]
    fn formatter_reduces_trailing_blank_lines_to_single_final_newline() {
        let formatted = format_markdown("Text\n\n\n", &FormatOptions::default())
            .expect("format should succeed");

        assert_eq!(formatted.content, "Text\n");
    }
}
