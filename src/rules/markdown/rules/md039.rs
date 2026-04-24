use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD039 / no-space-in-links — Spaces inside link text.
pub struct NoSpacesInLinksRule;

impl MarkdownRule for NoSpacesInLinksRule {
    fn id(&self) -> &'static str {
        "MD039"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD039")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD039");
        let mut diagnostics = Vec::new();
        for (i, line) in content.lines().enumerate() {
            for (open, close, replacement) in link_text_space_fixes(line) {
                diagnostics.push(MarkdownDiagnostic {
                    file: file_path.to_path_buf(),
                    severity: DiagnosticSeverity::Warning,
                    range: DiagnosticRange {
                        start_line: i + 1,
                        start_column: open + 1,
                        end_line: i + 1,
                        end_column: close + 2,
                    },
                    message: meta.description.to_string(),
                    rule_id: meta.code.to_string(),
                    official_meta: Some(meta.clone()),
                    fix_info: Some(crate::rules::markdown::types::DiagnosticFix {
                        start_line: i + 1,
                        start_column: open + 2,
                        end_line: i + 1,
                        end_column: close + 1,
                        replacement,
                    }),
                });
            }
        }
        diagnostics
    }
}

fn link_text_space_fixes(line: &str) -> Vec<(usize, usize, String)> {
    let bytes = line.as_bytes();
    let mut fixes = Vec::new();
    let mut cursor = 0;
    let mut in_code = false;

    while cursor < bytes.len() {
        match bytes[cursor] {
            b'`' => {
                in_code = !in_code;
                cursor += 1;
            }
            b'[' if !in_code && !is_image_marker(bytes, cursor) => {
                let Some(close) = line[cursor + 1..]
                    .find(']')
                    .map(|offset| cursor + 1 + offset)
                else {
                    break;
                };

                if is_link_text_candidate(bytes, close) {
                    let inner = &line[cursor + 1..close];
                    let trimmed = inner.trim();
                    if !trimmed.is_empty() && trimmed != inner {
                        fixes.push((cursor, close, trimmed.to_string()));
                    }
                }
                cursor = close + 1;
            }
            _ => cursor += 1,
        }
    }

    fixes
}

fn is_image_marker(bytes: &[u8], open_bracket: usize) -> bool {
    open_bracket > 0 && bytes[open_bracket - 1] == b'!'
}

fn is_link_text_candidate(bytes: &[u8], close_bracket: usize) -> bool {
    matches!(bytes.get(close_bracket + 1), Some(b'(' | b'['))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_spaces_inside_inline_link_text() {
        let rule = NoSpacesInLinksRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "[ text ](https://example.com)");

        assert_eq!(diagnostics.len(), 1);
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("link text spaces should be fixable");
        assert_eq!(fix.start_column, 2);
        assert_eq!(fix.end_column, 8);
        assert_eq!(fix.replacement, "text");
    }

    #[test]
    fn ignores_task_checkboxes_and_image_alt_text() {
        let rule = NoSpacesInLinksRule;
        let diagnostics = rule.evaluate(
            Path::new("doc.md"),
            "- [ ] task\n- [x] done\n![ alt ](image.png)",
        );

        assert!(diagnostics.is_empty());
    }
}
