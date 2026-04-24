use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD014 / dollar-signs-before-commands — Dollar signs used before commands without spaces.
pub struct DollarSignsBeforeCommandsRule;

impl MarkdownRule for DollarSignsBeforeCommandsRule {
    fn id(&self) -> &'static str {
        "MD014"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD014")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD014");
        let mut diagnostics = Vec::new();
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim_start();
            if trimmed.starts_with('$')
                && trimmed
                    .chars()
                    .nth(1)
                    .is_some_and(|next| !next.is_whitespace())
            {
                let indent = line.len() - trimmed.len();
                let fix = crate::rules::markdown::types::DiagnosticFix {
                    start_line: i + 1,
                    start_column: indent + 1,
                    end_line: i + 1,
                    end_column: indent + 2,
                    replacement: String::new(),
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
        }
        diagnostics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_dollar_sign_before_command() {
        let rule = DollarSignsBeforeCommandsRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "$echo hi\n$ ls");

        assert_eq!(diagnostics.len(), 1);
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("dollar prompt should be fixable");
        assert_eq!(fix.replacement, "");
        assert_eq!(fix.start_column, 1);
        assert_eq!(fix.end_column, 2);
    }
}
