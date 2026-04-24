use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
    RuleParityStatus,
};
use std::path::Path;

/* WHY: Section: Heading-related markdownlint rules (part 2)
=======================================================
  Split from rules_heading.rs to stay within 200-line limit. */

/// MD025 / single-title/single-h1 — Only one top-level heading in the document.
pub struct SingleH1Rule;

impl MarkdownRule for SingleH1Rule {
    fn id(&self) -> &'static str {
        "MD025"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD025")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD025");
        let mut diagnostics = Vec::new();
        let mut h1_count = 0;
        let mut in_code_block = false;
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim_start();
            if RuleHelpers::is_fence(trimmed) {
                in_code_block = !in_code_block;
                continue;
            }
            if in_code_block {
                continue;
            }
            if trimmed.starts_with("# ") && !trimmed.starts_with("## ") {
                h1_count += 1;
                if h1_count > 1 {
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

/// MD026 / no-trailing-punctuation — Trailing punctuation in heading.
pub struct NoTrailingPunctuationRule;

impl MarkdownRule for NoTrailingPunctuationRule {
    fn id(&self) -> &'static str {
        "MD026"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD026",
            title: "no-trailing-punctuation",
            description: "Trailing punctuation in heading.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md026.md",
            parity: RuleParityStatus::Official,
            is_fixable: true,
            properties: &[crate::rule_prop!(
                String,
                "punctuation",
                "Punctuation characters",
                ".,;:!。，；：！"
            )],
        })
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD026");
        let mut diagnostics = Vec::new();
        let mut in_code_block = false;
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim_start();
            if RuleHelpers::is_fence(trimmed) {
                in_code_block = !in_code_block;
                continue;
            }
            if in_code_block || !RuleHelpers::is_atx_heading(trimmed) {
                continue;
            }
            let heading_text = trimmed.trim_start_matches('#').trim();
            let Some(punctuation) = heading_text.chars().last() else {
                continue;
            };
            if !".,;:!".contains(punctuation) {
                continue;
            }

            let trimmed_end = line.trim_end();
            let start = trimmed_end.len() - punctuation.len_utf8();
            diagnostics.push(MarkdownDiagnostic {
                file: file_path.to_path_buf(),
                severity: DiagnosticSeverity::Warning,
                range: DiagnosticRange {
                    start_line: i + 1,
                    start_column: start + 1,
                    end_line: i + 1,
                    end_column: trimmed_end.len() + 1,
                },
                message: meta.description.to_string(),
                rule_id: meta.code.to_string(),
                official_meta: Some(meta.clone()),
                fix_info: Some(crate::rules::markdown::types::DiagnosticFix {
                    start_line: i + 1,
                    start_column: start + 1,
                    end_line: i + 1,
                    end_column: trimmed_end.len() + 1,
                    replacement: String::new(),
                }),
            });
        }
        diagnostics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_trailing_punctuation_in_heading() {
        let rule = NoTrailingPunctuationRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "# Heading!\n\nText");

        assert_eq!(diagnostics.len(), 1);
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("heading punctuation should be fixable");
        assert_eq!(fix.start_column, 10);
        assert_eq!(fix.end_column, 11);
        assert_eq!(fix.replacement, "");
    }
}
