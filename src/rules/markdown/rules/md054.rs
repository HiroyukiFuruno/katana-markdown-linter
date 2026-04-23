use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta, RuleParityStatus,
};
use std::path::Path;

/// MD054 / link-style — Link style.
pub struct LinkStyleRule;

impl MarkdownRule for LinkStyleRule {
    fn id(&self) -> &'static str {
        "MD054"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD054",
            title: "link-style",
            description: "Link style.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md054.md",
            parity: RuleParityStatus::Official,
            is_fixable: false,
            properties: &[
                crate::rule_prop!(Boolean, "autolink", "Allow autolinks", "true"),
                crate::rule_prop!(Boolean, "inline", "Allow inline links and images", "true"),
                crate::rule_prop!(
                    Boolean,
                    "full",
                    "Allow full reference links and images",
                    "true"
                ),
                crate::rule_prop!(
                    Boolean,
                    "collapsed",
                    "Allow collapsed reference links and images",
                    "true"
                ),
                crate::rule_prop!(
                    Boolean,
                    "shortcut",
                    "Allow shortcut reference links and images",
                    "true"
                ),
                crate::rule_prop!(Boolean, "url_inline", "Allow URLs as inline links", "true"),
            ],
        })
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD054");
        let has_inline = content.contains("](");
        let has_reference = content.contains("][") || content.contains("]:");
        if has_inline && has_reference {
            return vec![MarkdownDiagnostic {
                file: file_path.to_path_buf(),
                severity: DiagnosticSeverity::Warning,
                range: crate::rules::markdown::DiagnosticRange {
                    start_line: 1,
                    start_column: 1,
                    end_line: 1,
                    end_column: content.lines().next().unwrap_or("").len().max(1),
                },
                message: meta.description.to_string(),
                rule_id: meta.code.to_string(),
                official_meta: Some(meta),
                fix_info: None,
            }];
        }
        Vec::new()
    }
}
