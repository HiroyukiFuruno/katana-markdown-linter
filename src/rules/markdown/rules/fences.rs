use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta, RuleParityStatus,
};
use std::path::Path;

/// MD031 / blanks-around-fences - Fenced code blocks should be surrounded by blank lines.
pub struct BlanksAroundFencesRule;

impl MarkdownRule for BlanksAroundFencesRule {
    fn id(&self) -> &'static str {
        "MD031"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD031",
            title: "blanks-around-fences",
            description: "Fenced code blocks should be surrounded by blank lines.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md031.md",
            parity: RuleParityStatus::Official,
            is_fixable: true,
            properties: &[crate::rule_prop!(
                Boolean,
                "list_items",
                "Include list items",
                "true"
            )],
        })
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD031");
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim_start();
            if !RuleHelpers::is_fence(trimmed) {
                continue;
            }

            let has_blank_before = i == 0 || lines[i - 1].trim().is_empty();
            let has_blank_after = i + 1 >= lines.len() || lines[i + 1].trim().is_empty();

            if !has_blank_before || !has_blank_after {
                RuleHelpers::push_diag(
                    &mut diagnostics,
                    file_path,
                    i,
                    line,
                    &meta,
                    DiagnosticSeverity::Warning,
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
    fn reports_fences_without_blank_lines_around_them() {
        let rule = BlanksAroundFencesRule;
        let content = "Paragraph\n```rust\ncode\n```\nNext";
        let diagnostics = rule.evaluate(Path::new("doc.md"), content);
        assert!(!diagnostics.is_empty());
    }
}
