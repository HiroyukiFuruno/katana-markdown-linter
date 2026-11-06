use crate::{
    fix::{FixEngine, LineOffsetIndex, DEFAULT_MAX_FIX_PASSES},
    rules::markdown::document::DocumentContext,
    Error, LintOptions, MarkdownLinter, RuleConfig,
};
use serde::Serialize;
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::{ops::Range, path::Path};

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
            max_passes: DEFAULT_MAX_FIX_PASSES,
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
        Self::format_scoped(content, 0..content.len(), options, lint_options)
    }

    pub(crate) fn format_markdown_range(
        content: &str,
        selected: Range<usize>,
        options: &FormatOptions,
        lint_options: &LintOptions,
    ) -> Result<String, Error> {
        Ok(Self::format_scoped(content, selected, options, lint_options)?.content)
    }

    fn format_scoped(
        content: &str,
        selected: Range<usize>,
        options: &FormatOptions,
        lint_options: &LintOptions,
    ) -> Result<FormatResult, Error> {
        let prefix = normalize_line_endings(&content[..selected.start]);
        let suffix = normalize_line_endings(&content[selected.end..]);
        let selection = normalize_line_endings(&content[selected]);
        let scope = FormatScope {
            prefix_len: prefix.len(),
            suffix_len: suffix.len(),
        };
        let mut applied_operations = usize::from(matches!(selection, Cow::Owned(_)));
        let mut content = [prefix.as_ref(), selection.as_ref(), suffix.as_ref()].concat();
        applied_operations += usize::from(scope.normalize_terminal_newline(&mut content));
        let check = |current: &str| scope.check(current, lint_options);
        let fixed = FixEngine::run(&content, check(&content)?, options.max_passes, false, check)?;
        applied_operations += fixed.applied_fixes;
        content = fixed.content;
        applied_operations += usize::from(scope.normalize_terminal_newline(&mut content));
        content.truncate(content.len() - scope.suffix_len);
        content.drain(..scope.prefix_len);

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

fn normalize_line_endings(content: &str) -> Cow<'_, str> {
    if !content.contains('\r') {
        return Cow::Borrowed(content);
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
    Cow::Owned(normalized)
}

struct FormatScope {
    prefix_len: usize,
    suffix_len: usize,
}

impl FormatScope {
    fn check(&self, content: &str, options: &LintOptions) -> Result<Vec<crate::LintResult>, Error> {
        let mut diagnostics = MarkdownLinter::lint(content, options)?;
        if self.prefix_len == 0 && self.suffix_len == 0 {
            return Ok(diagnostics);
        }

        let end = content.len() - self.suffix_len;
        let index = LineOffsetIndex::new(content);
        /* WHY: 選択外の接頭部と接尾部を固定し、各パスの最新座標で範囲を判定する。 */
        diagnostics.retain(|diagnostic| {
            diagnostic
                .fix
                .as_ref()
                .and_then(|fix| index.offsets_for_range(&fix.range))
                .is_some_and(|edit| {
                    edit.start >= self.prefix_len
                        && edit.end <= end
                        && (edit.start < end || self.suffix_len == 0)
                })
        });
        Ok(diagnostics)
    }

    fn normalize_terminal_newline(&self, content: &mut String) -> bool {
        if self.suffix_len != 0 || content.is_empty() {
            return false;
        }
        let trimmed = content.trim_end_matches('\n').len();
        let keep_end = (trimmed + 1).max(self.prefix_len);
        if keep_end == content.len() {
            return false;
        }
        let ctx = DocumentContext::new(Path::new(""), content);
        if ctx.literal_content_at_eof() {
            return false;
        }
        if keep_end > content.len() {
            content.push('\n');
        } else {
            content.truncate(keep_end);
        }
        true
    }
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
