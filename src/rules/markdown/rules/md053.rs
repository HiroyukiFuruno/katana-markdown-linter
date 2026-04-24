use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD053 / link-definitions — Link definitions.
pub struct LinkDefinitionsRule;

impl MarkdownRule for LinkDefinitionsRule {
    fn id(&self) -> &'static str {
        "MD053"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD053")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD053");
        let mut diagnostics = Vec::new();
        let mut seen = std::collections::HashSet::new();
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim_start();
            if let Some(label) = trimmed
                .strip_prefix('[')
                .and_then(|rest| rest.split_once("]:"))
            {
                if !seen.insert(label.0.to_lowercase()) {
                    let fix = crate::rules::markdown::types::DiagnosticFix {
                        start_line: i + 1,
                        start_column: 1,
                        end_line: i + 2,
                        end_column: 1,
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
        }
        diagnostics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_duplicate_link_definition_by_removing_later_definition() {
        let rule = LinkDefinitionsRule;
        let diagnostics = rule.evaluate(
            Path::new("doc.md"),
            "[one]: https://example.com/1\n[one]: https://example.com/2\nText\n",
        );

        assert_eq!(diagnostics.len(), 1);
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("duplicate link definition should be fixable");
        assert_eq!(fix.start_line, 2);
        assert_eq!(fix.end_line, 3);
        assert_eq!(fix.replacement, "");
    }
}
