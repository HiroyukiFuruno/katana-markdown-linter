use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use crate::types::RuleConfig;
use std::path::Path;

/// MD049 / emphasis-style — Emphasis style.
pub struct EmphasisStyleRule;

impl MarkdownRule for EmphasisStyleRule {
    fn id(&self) -> &'static str {
        "MD049"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD049")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        self.evaluate_with_style(file_path, content, None)
    }

    fn evaluate_configured(
        &self,
        file_path: &Path,
        content: &str,
        config: Option<&RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let style = config
            .and_then(|config| config.properties.get("style"))
            .map(String::as_str);
        self.evaluate_with_style(file_path, content, style)
    }
}

impl EmphasisStyleRule {
    fn evaluate_with_style(
        &self,
        file_path: &Path,
        content: &str,
        style: Option<&str>,
    ) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD049");
        let mut diagnostics = Vec::new();
        let mut expected = match style {
            Some("asterisk") => Some('*'),
            Some("underscore") => Some('_'),
            _ => None,
        };

        for (i, line) in content.lines().enumerate() {
            for span in emphasis_spans(line) {
                let expected_marker = *expected.get_or_insert(span.marker);
                if span.marker == expected_marker {
                    continue;
                }

                diagnostics.push(MarkdownDiagnostic {
                    file: file_path.to_path_buf(),
                    severity: DiagnosticSeverity::Warning,
                    range: DiagnosticRange {
                        start_line: i + 1,
                        start_column: span.start + 1,
                        end_line: i + 1,
                        end_column: span.end + 1,
                    },
                    message: meta.description.to_string(),
                    rule_id: meta.code.to_string(),
                    official_meta: Some(meta.clone()),
                    fix_info: Some(crate::rules::markdown::types::DiagnosticFix {
                        start_line: i + 1,
                        start_column: span.start + 1,
                        end_line: i + 1,
                        end_column: span.end + 1,
                        replacement: format!("{expected_marker}{}{expected_marker}", span.inner),
                    }),
                });
            }
        }
        diagnostics
    }
}

struct EmphasisSpan<'a> {
    marker: char,
    start: usize,
    end: usize,
    inner: &'a str,
}

fn emphasis_spans(line: &str) -> Vec<EmphasisSpan<'_>> {
    let bytes = line.as_bytes();
    let mut spans = Vec::new();
    let mut cursor = 0;
    let mut in_code = false;

    while cursor < bytes.len() {
        match bytes[cursor] {
            b'`' => {
                in_code = !in_code;
                cursor += 1;
            }
            marker @ (b'*' | b'_') if !in_code => {
                if bytes.get(cursor + 1) == Some(&marker) {
                    cursor += 2;
                    continue;
                }
                let Some(close) = find_single_marker(line, cursor + 1, marker) else {
                    cursor += 1;
                    continue;
                };
                if marker == b'_' && is_intraword(line, cursor, close) {
                    cursor = close + 1;
                    continue;
                }
                let inner = &line[cursor + 1..close];
                if !inner.trim().is_empty() {
                    spans.push(EmphasisSpan {
                        marker: marker as char,
                        start: cursor,
                        end: close + 1,
                        inner,
                    });
                }
                cursor = close + 1;
            }
            _ => cursor += 1,
        }
    }

    spans
}

fn find_single_marker(line: &str, start: usize, marker: u8) -> Option<usize> {
    let bytes = line.as_bytes();
    let mut cursor = start;
    while cursor < bytes.len() {
        if bytes[cursor] == marker
            && bytes.get(cursor + 1) != Some(&marker)
            && bytes.get(cursor.wrapping_sub(1)) != Some(&marker)
        {
            return Some(cursor);
        }
        cursor += 1;
    }
    None
}

fn is_intraword(line: &str, open: usize, close: usize) -> bool {
    let bytes = line.as_bytes();
    open > 0
        && bytes
            .get(open - 1)
            .is_some_and(|byte| byte.is_ascii_alphanumeric())
        && bytes
            .get(close + 1)
            .is_some_and(|byte| byte.is_ascii_alphanumeric())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_emphasis_to_first_style() {
        let rule = EmphasisStyleRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "*Text* and _more_");

        assert_eq!(diagnostics.len(), 1);
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("emphasis style should be fixable");
        assert_eq!(fix.replacement, "*more*");
    }

    #[test]
    fn ignores_intraword_underscores() {
        let rule = EmphasisStyleRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "like_this_one");

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn fixes_emphasis_to_configured_style() {
        let rule = EmphasisStyleRule;
        let config = RuleConfig {
            enabled: true,
            properties: [("style".to_string(), "underscore".to_string())]
                .into_iter()
                .collect(),
        };
        let diagnostics = rule.evaluate_configured(Path::new("doc.md"), "*Text*", Some(&config));

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0].fix_info.as_ref().unwrap().replacement,
            "_Text_"
        );
    }
}
