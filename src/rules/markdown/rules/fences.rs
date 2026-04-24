use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD031 / blanks-around-fences - Fenced code blocks should be surrounded by blank lines.
pub struct BlanksAroundFencesRule;

impl MarkdownRule for BlanksAroundFencesRule {
    fn id(&self) -> &'static str {
        "MD031"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD031")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD031");
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        let mut in_code_block = false;
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim_start();
            if !RuleHelpers::is_fence(trimmed) {
                continue;
            }

            if !in_code_block {
                if i > 0 && !lines[i - 1].trim().is_empty() {
                    diagnostics.push(fence_blank_fix(
                        file_path,
                        i,
                        line,
                        &meta,
                        FenceBlankFix::Before,
                    ));
                }
                in_code_block = true;
            } else {
                if i + 1 < lines.len() && !lines[i + 1].trim().is_empty() {
                    diagnostics.push(fence_blank_fix(
                        file_path,
                        i,
                        line,
                        &meta,
                        FenceBlankFix::After,
                    ));
                }
                in_code_block = false;
            }
        }

        diagnostics
    }
}

enum FenceBlankFix {
    Before,
    After,
}

fn fence_blank_fix(
    file_path: &Path,
    line_idx: usize,
    line: &str,
    meta: &OfficialRuleMeta,
    kind: FenceBlankFix,
) -> MarkdownDiagnostic {
    let (start_column, replacement) = match kind {
        FenceBlankFix::Before => (1, "\n".to_string()),
        FenceBlankFix::After => (line.len() + 1, "\n".to_string()),
    };
    MarkdownDiagnostic {
        file: file_path.to_path_buf(),
        severity: DiagnosticSeverity::Warning,
        range: DiagnosticRange {
            start_line: line_idx + 1,
            start_column,
            end_line: line_idx + 1,
            end_column: start_column,
        },
        message: meta.description.to_string(),
        rule_id: meta.code.to_string(),
        official_meta: Some(meta.clone()),
        fix_info: Some(crate::rules::markdown::types::DiagnosticFix {
            start_line: line_idx + 1,
            start_column,
            end_line: line_idx + 1,
            end_column: start_column,
            replacement,
        }),
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
        assert_eq!(diagnostics.len(), 2);
        assert!(diagnostics
            .iter()
            .all(|diagnostic| diagnostic.fix_info.is_some()));
    }
}
