use crate::rules::markdown::inline::{
    extract_emphasis_spans, safe_replacement, touches_inline_code, InlineEmphasisKind,
};
use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta,
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
            for span in extract_emphasis_spans(ctx, line, InlineEmphasisKind::Strong) {
                if has_inline_code_marker && touches_inline_code(ctx, line_start, &span) {
                    continue;
                }
                let source_marker = if span.marker == '*' { "**" } else { "__" };
                let expected_marker = *expected.get_or_insert(source_marker);
                if source_marker == expected_marker {
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
                    fix_info: safe_replacement(
                        ctx,
                        line,
                        &span,
                        if expected_marker == "**" { '*' } else { '_' },
                    )
                    .map(|replacement| {
                        crate::rules::markdown::types::DiagnosticFix {
                            start_line: line.number,
                            start_column: span.start + 1,
                            end_line: line.number,
                            end_column: span.end + 1,
                            replacement,
                        }
                    }),
                });
            }
        }
        diagnostics
    }
}

#[cfg(test)]
mod tests;
