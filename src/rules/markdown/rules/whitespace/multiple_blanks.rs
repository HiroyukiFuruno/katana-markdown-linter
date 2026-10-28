use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta,
};
use std::path::Path;

pub struct NoMultipleBlanksRule;

impl MarkdownRule for NoMultipleBlanksRule {
    fn id(&self) -> &'static str {
        "MD012"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD012")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let ctx = DocumentContext::new(file_path, content);
        self.evaluate_context(&ctx, None)
    }

    fn evaluate_configured(
        &self,
        file_path: &Path,
        content: &str,
        config: Option<&crate::types::RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let ctx = DocumentContext::new(file_path, content);
        self.evaluate_context(&ctx, config)
    }

    fn evaluate_context(
        &self,
        ctx: &DocumentContext<'_>,
        config: Option<&crate::types::RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD012");
        let maximum = config
            .and_then(|config| config.properties.get("maximum"))
            .and_then(|value| value.parse::<usize>().ok())
            .unwrap_or(1);
        let mut diagnostics = Vec::new();
        let mut consecutive_blanks = 0;
        let file_path = ctx.file_path();
        for (index, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(index) {
                consecutive_blanks = 0;
                continue;
            }
            consecutive_blanks = evaluate_blank_line(
                &mut diagnostics,
                file_path,
                index,
                line.text,
                &meta,
                consecutive_blanks,
                maximum,
            );
        }
        push_trailing_blank_diagnostic(
            &mut diagnostics,
            file_path,
            ctx.content(),
            ctx,
            consecutive_blanks,
            maximum,
            meta,
        );
        diagnostics
    }
}

fn evaluate_blank_line(
    diagnostics: &mut Vec<MarkdownDiagnostic>,
    file_path: &Path,
    index: usize,
    line: &str,
    meta: &OfficialRuleMeta,
    consecutive_blanks: usize,
    maximum: usize,
) -> usize {
    let trimmed = line.trim();
    if !trimmed.is_empty() {
        return 0;
    }
    let current_blanks = consecutive_blanks + 1;
    if current_blanks <= maximum {
        return current_blanks;
    }
    let fix = crate::rules::markdown::types::DiagnosticFix {
        start_line: index + 1,
        start_column: 1,
        end_line: index + 2,
        end_column: 1,
        replacement: String::new(),
    };
    RuleHelpers::push_diag_with_fix(
        diagnostics,
        file_path,
        index,
        line,
        meta,
        DiagnosticSeverity::Warning,
        Some(fix),
    );
    current_blanks
}

fn push_trailing_blank_diagnostic(
    diagnostics: &mut Vec<MarkdownDiagnostic>,
    file_path: &Path,
    content: &str,
    ctx: &DocumentContext<'_>,
    consecutive_blanks: usize,
    maximum: usize,
    meta: OfficialRuleMeta,
) {
    if !content.ends_with("\n\n") || consecutive_blanks != 1 || consecutive_blanks > maximum {
        return;
    }
    diagnostics.push(MarkdownDiagnostic {
        file: file_path.to_path_buf(),
        severity: DiagnosticSeverity::Warning,
        range: DiagnosticRange {
            start_line: ctx.lines().len() + 1,
            start_column: 1,
            end_line: ctx.lines().len() + 1,
            end_column: 1,
        },
        message: meta.description.to_string(),
        rule_id: meta.code.to_string(),
        official_meta: Some(meta),
        fix_info: ctx
            .lines()
            .last()
            .map(|line| crate::rules::markdown::types::DiagnosticFix {
                start_line: line.number,
                start_column: 1,
                end_line: line.number + 1,
                end_column: 1,
                replacement: String::new(),
            }),
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::RuleConfig;

    fn configured(maximum: usize) -> RuleConfig {
        RuleConfig {
            enabled: true,
            properties: [("maximum".to_string(), maximum.to_string())].into(),
        }
    }

    #[test]
    fn maximum_zero_reports_a_single_blank_once_at_eof() {
        let diagnostics = NoMultipleBlanksRule.evaluate_configured(
            Path::new("doc.md"),
            "paragraph\n\n",
            Some(&configured(0)),
        );

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].range.start_line, 2);
        assert!(diagnostics[0].fix_info.is_some());
    }

    #[test]
    fn maximum_two_allows_two_and_reports_the_third_consecutive_blank() {
        let diagnostics = NoMultipleBlanksRule.evaluate_configured(
            Path::new("doc.md"),
            "first\n\n\n\nsecond\n",
            Some(&configured(2)),
        );

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].range.start_line, 4);
        assert_eq!(diagnostics[0].fix_info.as_ref().unwrap().start_line, 4);
    }

    #[test]
    fn default_maximum_retains_trailing_blank_diagnostic() {
        let diagnostics = NoMultipleBlanksRule.evaluate(Path::new("doc.md"), "paragraph\n\n");

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].fix_info.is_some());
    }
}
