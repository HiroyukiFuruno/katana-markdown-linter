use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD034 / no-bare-urls — No bare URLs.
pub struct NoBareUrlsRule;

impl MarkdownRule for NoBareUrlsRule {
    fn id(&self) -> &'static str {
        "MD034"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD034")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD034");
        let mut diagnostics = Vec::new();
        for (i, line) in content.lines().enumerate() {
            if let Some((start, end)) = bare_url_range(line) {
                let url = &line[start..end];
                diagnostics.push(MarkdownDiagnostic {
                    file: file_path.to_path_buf(),
                    severity: DiagnosticSeverity::Warning,
                    range: DiagnosticRange {
                        start_line: i + 1,
                        start_column: start + 1,
                        end_line: i + 1,
                        end_column: end + 1,
                    },
                    message: meta.description.to_string(),
                    rule_id: meta.code.to_string(),
                    official_meta: Some(meta.clone()),
                    fix_info: Some(crate::rules::markdown::types::DiagnosticFix {
                        start_line: i + 1,
                        start_column: start + 1,
                        end_line: i + 1,
                        end_column: end + 1,
                        replacement: format!("<{url}>"),
                    }),
                });
            }
        }
        diagnostics
    }
}

fn bare_url_range(line: &str) -> Option<(usize, usize)> {
    let start = match (line.find("http://"), line.find("https://")) {
        (Some(http), Some(https)) => http.min(https),
        (Some(http), None) => http,
        (None, Some(https)) => https,
        (None, None) => return None,
    };
    if start > 0 && line.as_bytes()[start - 1] == b'<' {
        return None;
    }
    if line[..start].ends_with("](") || line[..start].ends_with(")[") {
        return None;
    }

    let mut end = line.len();
    for (offset, ch) in line[start..].char_indices() {
        if ch.is_whitespace() || ch == '<' || ch == '>' {
            end = start + offset;
            break;
        }
    }
    while end > start && matches!(line.as_bytes()[end - 1], b'.' | b',' | b';' | b':') {
        end -= 1;
    }
    (end > start).then_some((start, end))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_bare_url_with_angle_brackets() {
        let rule = NoBareUrlsRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "See http://example.com now.");
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("url should be fixable");
        assert_eq!(fix.replacement, "<http://example.com>");
    }

    #[test]
    fn ignores_url_inside_inline_link_destination() {
        let rule = NoBareUrlsRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "[Example](https://example.com)");

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn ignores_url_inside_reversed_link_destination() {
        let rule = NoBareUrlsRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "(Example)[https://example.com]");

        assert!(diagnostics.is_empty());
    }
}
