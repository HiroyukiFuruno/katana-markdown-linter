use crate::rules::markdown::inline::{
    extract_emphasis_spans, safe_replacement, touches_inline_code, InlineEmphasisKind,
};
use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta,
};
use crate::types::RuleConfig;
use std::path::Path;

/// MD049 / emphasis-style — Emphasis style.
pub struct EmphasisStyleRule;

impl MarkdownRule for EmphasisStyleRule {
    fn id(&self) -> &'static str {
        "MD049"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD049")
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

impl EmphasisStyleRule {
    fn evaluate_context_with_style(
        &self,
        ctx: &DocumentContext<'_>,
        style: Option<&str>,
    ) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD049");
        let mut diagnostics = Vec::new();
        let mut expected = match style {
            Some("asterisk") => Some('*'),
            Some("underscore") => Some('_'),
            _ => None,
        };

        for (i, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(i) {
                continue;
            }
            let has_inline_code_marker = line.text.contains('`');
            let line_start = line.content_range.start;
            for span in extract_emphasis_spans(ctx, line, InlineEmphasisKind::Emphasis) {
                if has_inline_code_marker && touches_inline_code(ctx, line_start, &span) {
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
                    fix_info: safe_replacement(ctx, line, &span, expected_marker).map(
                        |replacement| crate::rules::markdown::types::DiagnosticFix {
                            start_line: line.number,
                            start_column: span.start + 1,
                            end_line: line.number,
                            end_column: span.end + 1,
                            replacement,
                        },
                    ),
                });
            }
        }
        diagnostics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_emphasis_to_first_style() {
        let rule = EmphasisStyleRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "*Text* and _more_");

        assert_eq!(diagnostics.len(), 1);
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("emphasis style should be fixable");
        assert_eq!(fix.replacement, "*more*");
    }

    #[test]
    fn ignores_intraword_underscores() {
        let rule = EmphasisStyleRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "like_this_one");

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn fixes_emphasis_to_configured_style() {
        let rule = EmphasisStyleRule;
        let config = RuleConfig {
            enabled: true,
            properties: [("style".to_string(), "underscore".to_string())]
                .into_iter()
                .collect(),
        };
        let diagnostics = rule.evaluate_configured(Path::new("doc.md"), "*Text*", Some(&config));

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0].fix_info.as_ref().unwrap().replacement,
            "_Text_"
        );
    }

    #[test]
    fn ignores_emphasis_inside_fenced_code() {
        let rule = EmphasisStyleRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "```\n*one* and _two_\n```\n");

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn ignores_emphasis_inside_long_and_unclosed_code_spans() {
        let rule = EmphasisStyleRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "*one* ``_two_``\n`_three_\n");

        assert!(diagnostics.is_empty());
    }
}
