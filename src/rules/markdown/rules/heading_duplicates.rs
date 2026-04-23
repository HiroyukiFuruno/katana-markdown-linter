use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta, RuleParityStatus,
};
use std::collections::HashSet;
use std::path::Path;

/// MD024 / no-duplicate-heading - Multiple headings with the same content.
pub struct NoDuplicateHeadingRule;

impl MarkdownRule for NoDuplicateHeadingRule {
    fn id(&self) -> &'static str {
        "MD024"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD024",
            title: "no-duplicate-heading",
            description: "Multiple headings with the same content.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md024.md",
            parity: RuleParityStatus::Official,
            is_fixable: false,
            properties: &[crate::rule_prop!(
                Boolean,
                "siblings_only",
                "Only check sibling headings",
                "false"
            )],
        })
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD024");
        let mut diagnostics = Vec::new();
        let mut seen = HashSet::new();
        let mut in_code_block = false;
        let lines: Vec<&str> = content.lines().collect();

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim_start();
            if RuleHelpers::is_fence(trimmed) {
                in_code_block = !in_code_block;
                continue;
            }
            if in_code_block {
                continue;
            }

            if let Some(heading_text) = extract_heading_text(&lines, i) {
                if !seen.insert(heading_text.clone()) {
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
        }

        diagnostics
    }
}

fn extract_heading_text(lines: &[&str], idx: usize) -> Option<String> {
    let line = lines[idx];
    let trimmed = line.trim_start();
    if RuleHelpers::is_atx_heading(trimmed) {
        return Some(trimmed.trim_start_matches('#').trim().to_string());
    }

    if idx + 1 < lines.len() && is_setext_underline(lines[idx + 1].trim()) {
        let heading = line.trim();
        if !heading.is_empty() {
            return Some(heading.to_string());
        }
    }

    None
}

fn is_setext_underline(trimmed: &str) -> bool {
    if trimmed.len() < 2 {
        return false;
    }
    trimmed.chars().all(|c| c == '=') || trimmed.chars().all(|c| c == '-')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_duplicate_headings() {
        let rule = NoDuplicateHeadingRule;
        let content = "# title\n\n## section\n\n# title";
        let diagnostics = rule.evaluate(Path::new("doc.md"), content);
        assert_eq!(diagnostics.len(), 1);
    }
}
