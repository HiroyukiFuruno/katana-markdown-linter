use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

pub struct UlStyleRule;

impl MarkdownRule for UlStyleRule {
    fn id(&self) -> &'static str {
        "MD004"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD004")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD004");
        let mut diagnostics = Vec::new();
        let mut first_bullet: Option<char> = None;
        let ctx = DocumentContext::new(file_path, content);
        for (index, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(index) {
                continue;
            }
            let line_text = line.text;
            let trimmed = line_text.trim_start();
            if let Some(bullet) = RuleHelpers::get_bullet_char(trimmed) {
                push_mismatch(
                    &mut diagnostics,
                    &mut first_bullet,
                    file_path,
                    index,
                    line_text,
                    &meta,
                    bullet,
                );
            }
        }
        diagnostics
    }
}

fn push_mismatch(
    diagnostics: &mut Vec<MarkdownDiagnostic>,
    first_bullet: &mut Option<char>,
    file_path: &Path,
    index: usize,
    line_text: &str,
    meta: &OfficialRuleMeta,
    bullet: char,
) {
    let Some(expected) = first_bullet else {
        *first_bullet = Some(bullet);
        return;
    };
    if bullet == *expected {
        return;
    }
    let bullet_pos = line_text
        .find(bullet)
        .expect("bullet exists in source line");
    let fix = crate::rules::markdown::types::DiagnosticFix {
        start_line: index + 1,
        start_column: bullet_pos + 1,
        end_line: index + 1,
        end_column: bullet_pos + 2,
        replacement: expected.to_string(),
    };
    RuleHelpers::push_diag_with_fix(
        diagnostics,
        file_path,
        index,
        line_text,
        meta,
        DiagnosticSeverity::Warning,
        Some(fix),
    );
}
