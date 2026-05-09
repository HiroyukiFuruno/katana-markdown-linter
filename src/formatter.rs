use crate::{Error, FixResult, LintOptions, MarkdownLinter, RuleConfig};
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

pub struct MarkdownFormatter;

impl MarkdownFormatter {
    /// Applies deterministic Markdown layout formatting.
    ///
    /// The formatter intentionally uses a narrow layout-only policy. It normalizes line endings and
    /// then applies safe fixes for the formatter rule subset; it does not apply semantic/style rewrites.
    pub fn format_markdown(content: &str, options: &FormatOptions) -> Result<FormatResult, Error> {
        Self::format_markdown_with_lint_options(content, options, &Self::layout_lint_options())
    }

    pub fn format_markdown_with_lint_options(
        content: &str,
        options: &FormatOptions,
        lint_options: &LintOptions,
    ) -> Result<FormatResult, Error> {
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

        let max_passes = options.max_passes.max(1);
        for _ in 0..max_passes {
            let diagnostics = MarkdownLinter::lint(&content, lint_options)?;
            if !diagnostics
                .iter()
                .any(|diagnostic| diagnostic.fix.is_some())
            {
                break;
            }

            let fixed: FixResult = MarkdownLinter::fix_with_results(&content, &diagnostics);
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
        Self::layout_lint_options_from(&LintOptions::default())
    }

    pub fn layout_lint_options_from(base: &LintOptions) -> LintOptions {
        let layout_rules = LAYOUT_RULES.into_iter().collect::<HashSet<_>>();
        let mut options = base.clone();
        options.default_severity = crate::Severity::Warning;

        let mut rules = HashMap::new();
        for rule in crate::rules::markdown::MarkdownLinterOps::official_rules() {
            let id = rule.id();
            let is_layout = layout_rules.contains(id);
            let config = options.rules.get(id);

            rules.insert(
                id.to_string(),
                RuleConfig {
                    enabled: is_layout
                        && config
                            .map(|rule_config| rule_config.enabled)
                            .unwrap_or(true),
                    properties: config
                        .map(|rule_config| rule_config.properties.clone())
                        .unwrap_or_default(),
                },
            );
        }
        options.rules = rules;
        options
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
        let formatted =
            MarkdownFormatter::format_markdown("# Title\r\nText\r", &FormatOptions::default())
                .expect("format should succeed");

        assert_eq!(formatted.content, "# Title\n\nText\n");
        assert!(formatted.applied_operations >= 2);
    }

    #[test]
    fn formatter_applies_layout_subset_without_semantic_style_rewrites() {
        let source =
            "# Title\nText\n\n\n## Next\n```rust\ncode\n```\n| A | B |\n|---|---|\n| 1 | 2 |";
        let formatted = MarkdownFormatter::format_markdown(source, &FormatOptions::default())
            .expect("format should succeed");

        assert_eq!(
            formatted.content,
            "# Title\n\nText\n\n## Next\n\n```rust\ncode\n```\n\n| A | B |\n|---|---|\n| 1 | 2 |\n"
        );
    }

    #[test]
    fn formatter_is_idempotent() {
        let source = "# Title\nText\n\n\n-  item\n";
        let first = MarkdownFormatter::format_markdown(source, &FormatOptions::default())
            .expect("first format should succeed");
        let second = MarkdownFormatter::format_markdown(&first.content, &FormatOptions::default())
            .expect("second format should succeed");

        assert_eq!(first.content, second.content);
        assert_eq!(second.applied_operations, 0);
    }

    #[test]
    fn formatter_does_not_remove_trailing_spaces() {
        let formatted =
            MarkdownFormatter::format_markdown("hard break  \nnext\n", &FormatOptions::default())
                .expect("format should succeed");

        assert_eq!(formatted.content, "hard break  \nnext\n");
    }

    #[test]
    fn formatter_reduces_trailing_blank_lines_to_single_final_newline() {
        let formatted = MarkdownFormatter::format_markdown("Text\n\n\n", &FormatOptions::default())
            .expect("format should succeed");

        assert_eq!(formatted.content, "Text\n");
    }
}
