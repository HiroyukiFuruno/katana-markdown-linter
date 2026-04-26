use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta, SourceRange,
};
use crate::types::RuleConfig;
use std::path::Path;

/// MD050 / strong-style — Strong style.
pub struct StrongStyleRule;

impl MarkdownRule for StrongStyleRule {
    fn id(&self) -> &'static str {
        "MD050"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD050")
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
        let style = config
            .and_then(|config| config.properties.get("style"))
            .map(String::as_str);
        self.evaluate_context_with_style(ctx, style)
    }

    fn evaluate_configured(
        &self,
        file_path: &Path,
        content: &str,
        config: Option<&RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let ctx = DocumentContext::new(file_path, content);
        self.evaluate_context(&ctx, config)
    }
}

impl StrongStyleRule {
    fn evaluate_context_with_style(
        &self,
        ctx: &DocumentContext<'_>,
        style: Option<&str>,
    ) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD050");
        let mut diagnostics = Vec::new();
        let mut expected = match style {
            Some("asterisk") => Some("**"),
            Some("underscore") => Some("__"),
            _ => None,
        };

        for (i, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(i) {
                continue;
            }
            let has_inline_code_marker = line.text.contains('`');
            let line_start = line.content_range.start;
            for span in strong_spans(line.text) {
                if has_inline_code_marker
                    && ctx.is_inside_inline_code(SourceRange {
                        start: line_start + span.start,
                        end: line_start + span.end,
                    })
                {
                    continue;
                }
                let expected_marker = *expected.get_or_insert(span.marker);
                if span.marker == expected_marker {
                    continue;
                }

                diagnostics.push(MarkdownDiagnostic {
                    file: ctx.file_path().to_path_buf(),
                    severity: DiagnosticSeverity::Warning,
                    range: DiagnosticRange {
                        start_line: line.number,
                        start_column: span.start + 1,
                        end_line: line.number,
                        end_column: span.end + 1,
                    },
                    message: meta.description.to_string(),
                    rule_id: meta.code.to_string(),
                    official_meta: Some(meta.clone()),
                    fix_info: Some(crate::rules::markdown::types::DiagnosticFix {
                        start_line: line.number,
                        start_column: span.start + 1,
                        end_line: line.number,
                        end_column: span.end + 1,
                        replacement: format!("{expected_marker}{}{expected_marker}", span.inner),
                    }),
                });
            }
        }
        diagnostics
    }
}

struct StrongSpan<'a> {
    marker: &'static str,
    start: usize,
    end: usize,
    inner: &'a str,
}

fn strong_spans(line: &str) -> Vec<StrongSpan<'_>> {
    let bytes = line.as_bytes();
    let mut spans = Vec::new();
    let mut cursor = 0;

    while cursor + 1 < bytes.len() {
        match bytes[cursor] {
            marker @ (b'*' | b'_') if bytes.get(cursor + 1) == Some(&marker) => {
                let Some(close) = find_double_marker(line, cursor + 2, marker) else {
                    cursor += 2;
                    continue;
                };
                if marker == b'_' && is_intraword(line, cursor, close + 1) {
                    cursor = close + 2;
                    continue;
                }
                let inner = &line[cursor + 2..close];
                if !inner.trim().is_empty() {
                    spans.push(StrongSpan {
                        marker: if marker == b'*' { "**" } else { "__" },
                        start: cursor,
                        end: close + 2,
                        inner,
                    });
                }
                cursor = close + 2;
            }
            _ => cursor += 1,
        }
    }

    spans
}

fn find_double_marker(line: &str, start: usize, marker: u8) -> Option<usize> {
    let bytes = line.as_bytes();
    let mut cursor = start;
    while cursor + 1 < bytes.len() {
        if bytes[cursor] == marker && bytes[cursor + 1] == marker {
            return Some(cursor);
        }
        cursor += 1;
    }
    None
}

fn is_intraword(line: &str, open: usize, close: usize) -> bool {
    let bytes = line.as_bytes();
    open > 0
        && bytes
            .get(open - 1)
            .is_some_and(|byte| byte.is_ascii_alphanumeric())
        && bytes
            .get(close + 1)
            .is_some_and(|byte| byte.is_ascii_alphanumeric())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_strong_to_first_style() {
        let rule = StrongStyleRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "**Text** and __more__");

        assert_eq!(diagnostics.len(), 1);
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("strong style should be fixable");
        assert_eq!(fix.replacement, "**more**");
    }

    #[test]
    fn ignores_intraword_underscores() {
        let rule = StrongStyleRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "like__this__one");

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn fixes_strong_to_configured_style() {
        let rule = StrongStyleRule;
        let config = RuleConfig {
            enabled: true,
            properties: [("style".to_string(), "underscore".to_string())]
                .into_iter()
                .collect(),
        };
        let diagnostics = rule.evaluate_configured(Path::new("doc.md"), "**Text**", Some(&config));

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0].fix_info.as_ref().unwrap().replacement,
            "__Text__"
        );
    }

    #[test]
    fn ignores_strong_inside_fenced_code() {
        let rule = StrongStyleRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "```\n**one** and __two__\n```\n");

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn ignores_strong_inside_long_and_unclosed_code_spans() {
        let rule = StrongStyleRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "**one** ``__two__``\n`__three__\n");

        assert!(diagnostics.is_empty());
    }
}
