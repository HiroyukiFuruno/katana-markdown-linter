use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD011 / no-reversed-links — Reversed link syntax.
pub struct NoReversedLinksRule;

impl MarkdownRule for NoReversedLinksRule {
    fn id(&self) -> &'static str {
        "MD011"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD011")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD011");
        let mut diagnostics = Vec::new();
        for (i, line) in content.lines().enumerate() {
            for reversed in reversed_link_ranges(line) {
                diagnostics.push(MarkdownDiagnostic {
                    file: file_path.to_path_buf(),
                    severity: DiagnosticSeverity::Warning,
                    range: DiagnosticRange {
                        start_line: i + 1,
                        start_column: reversed.start + 1,
                        end_line: i + 1,
                        end_column: reversed.end + 1,
                    },
                    message: meta.description.to_string(),
                    rule_id: meta.code.to_string(),
                    official_meta: Some(meta.clone()),
                    fix_info: Some(crate::rules::markdown::types::DiagnosticFix {
                        start_line: i + 1,
                        start_column: reversed.start + 1,
                        end_line: i + 1,
                        end_column: reversed.end + 1,
                        replacement: format!("[{}]({})", reversed.text, reversed.destination),
                    }),
                });
            }
        }
        diagnostics
    }
}

struct ReversedLink<'a> {
    start: usize,
    end: usize,
    text: &'a str,
    destination: &'a str,
}

fn reversed_link_ranges(line: &str) -> Vec<ReversedLink<'_>> {
    let bytes = line.as_bytes();
    let mut ranges = Vec::new();
    let mut cursor = 0;
    let mut in_code = false;

    while cursor < bytes.len() {
        match bytes[cursor] {
            b'`' => {
                in_code = !in_code;
                cursor += 1;
            }
            b'(' if !in_code => {
                let Some(close_paren) = line[cursor + 1..]
                    .find(')')
                    .map(|offset| cursor + 1 + offset)
                else {
                    break;
                };
                if bytes.get(close_paren + 1) != Some(&b'[') {
                    cursor += 1;
                    continue;
                }
                let Some(close_bracket) = line[close_paren + 2..]
                    .find(']')
                    .map(|offset| close_paren + 2 + offset)
                else {
                    break;
                };

                let text = &line[cursor + 1..close_paren];
                let destination = &line[close_paren + 2..close_bracket];
                if !text.trim().is_empty() && !destination.starts_with('^') {
                    ranges.push(ReversedLink {
                        start: cursor,
                        end: close_bracket + 1,
                        text,
                        destination,
                    });
                }
                cursor = close_bracket + 1;
            }
            _ => cursor += 1,
        }
    }

    ranges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_reversed_link_syntax() {
        let rule = NoReversedLinksRule;
        let diagnostics = rule.evaluate(
            Path::new("doc.md"),
            "(Example)[https://www.example.com/] and text",
        );

        assert_eq!(diagnostics.len(), 1);
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("reversed link should be fixable");
        assert_eq!(fix.replacement, "[Example](https://www.example.com/)");
    }

    #[test]
    fn ignores_markdown_extra_footnotes() {
        let rule = NoReversedLinksRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "For (example)[^1]");

        assert!(diagnostics.is_empty());
    }
}
