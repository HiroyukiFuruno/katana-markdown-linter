use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD030 / list-marker-space - Spaces after list markers.
pub struct ListMarkerSpaceRule;

impl MarkdownRule for ListMarkerSpaceRule {
    fn id(&self) -> &'static str {
        "MD030"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD030")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD030");
        let mut diagnostics = Vec::new();
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

            if let Some(marker_end) = list_marker_end(line) {
                let mut after = line[marker_end..].chars();
                let spaces = after.by_ref().take_while(|c| c.is_whitespace()).count();
                if spaces > 0 {
                    let fix_col = marker_end + 1;
                    let fix = crate::rules::markdown::types::DiagnosticFix {
                        start_line: i + 1,
                        start_column: fix_col,
                        end_line: i + 1,
                        end_column: fix_col + spaces - 1,
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

fn list_marker_end(line: &str) -> Option<usize> {
    let trimmed = line.trim_start();
    let leading = line.len() - trimmed.len();
    let mut chars = trimmed.chars();
    let first = chars.next()?;

    if matches!(first, '-' | '*' | '+') && chars.next()? == ' ' {
        return Some(leading + 2);
    }

    if first.is_ascii_digit() {
        let dot_pos = trimmed.find(". ")?;
        let prefix = &trimmed[..dot_pos];
        if prefix.chars().all(|c| c.is_ascii_digit()) {
            return Some(leading + dot_pos + 2);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_extra_space_after_list_marker() {
        let rule = ListMarkerSpaceRule;
        let content = "-  item\n1.  item";
        let diagnostics = rule.evaluate(Path::new("doc.md"), content);
        assert_eq!(diagnostics.len(), 2);
    }
}
