use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use crate::types::RuleConfig;
use std::collections::HashMap;
use std::path::Path;

/// MD054 / link-style — Link style.
pub struct LinkStyleRule;

impl MarkdownRule for LinkStyleRule {
    fn id(&self) -> &'static str {
        "MD054"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD054")
    }

    fn evaluate(&self, _file_path: &Path, _content: &str) -> Vec<MarkdownDiagnostic> {
        Vec::new()
    }

    fn evaluate_configured(
        &self,
        file_path: &Path,
        content: &str,
        config: Option<&RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        if collapsed_references_allowed(config) {
            return Vec::new();
        }

        let meta = self.official_meta().expect("always Some for MD054");
        let has_inline = content.contains("](");
        let has_reference = content.contains("][") || content.contains("]:");
        if !has_inline || !has_reference {
            return Vec::new();
        }

        let definitions = link_definitions(content);
        let mut diagnostics = Vec::new();
        for (i, line) in content.lines().enumerate() {
            for reference in collapsed_references(line) {
                let destination = definitions.get(&reference.label.to_lowercase());
                diagnostics.push(MarkdownDiagnostic {
                    file: file_path.to_path_buf(),
                    severity: DiagnosticSeverity::Warning,
                    range: DiagnosticRange {
                        start_line: i + 1,
                        start_column: reference.start + 1,
                        end_line: i + 1,
                        end_column: reference.end + 1,
                    },
                    message: meta.description.to_string(),
                    rule_id: meta.code.to_string(),
                    official_meta: Some(meta.clone()),
                    fix_info: destination.map(|destination| {
                        crate::rules::markdown::types::DiagnosticFix {
                            start_line: i + 1,
                            start_column: reference.start + 1,
                            end_line: i + 1,
                            end_column: reference.end + 1,
                            replacement: format!("[{}]({destination})", reference.label),
                        }
                    }),
                });
            }
        }

        diagnostics
    }
}

fn collapsed_references_allowed(config: Option<&RuleConfig>) -> bool {
    config
        .and_then(|config| config.properties.get("collapsed"))
        .and_then(|value| value.parse::<bool>().ok())
        .unwrap_or(true)
}

struct CollapsedReference<'a> {
    start: usize,
    end: usize,
    label: &'a str,
}

fn collapsed_references(line: &str) -> Vec<CollapsedReference<'_>> {
    let bytes = line.as_bytes();
    let mut references = Vec::new();
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
                if bytes.get(close + 1) == Some(&b'[') && bytes.get(close + 2) == Some(&b']') {
                    let label = &line[cursor + 1..close];
                    if !label.trim().is_empty() {
                        references.push(CollapsedReference {
                            start: cursor,
                            end: close + 3,
                            label,
                        });
                    }
                    cursor = close + 3;
                } else {
                    cursor = close + 1;
                }
            }
            _ => cursor += 1,
        }
    }

    references
}

fn link_definitions(content: &str) -> HashMap<String, String> {
    let mut definitions = HashMap::new();
    for line in content.lines() {
        let trimmed = line.trim_start();
        let Some((label, rest)) = trimmed
            .strip_prefix('[')
            .and_then(|rest| rest.split_once("]:"))
        else {
            continue;
        };
        let destination = rest.split_whitespace().next().unwrap_or("");
        if !label.is_empty() && !destination.is_empty() {
            definitions.insert(label.to_lowercase(), destination.to_string());
        }
    }
    definitions
}

fn is_image_marker(bytes: &[u8], open_bracket: usize) -> bool {
    open_bracket > 0 && bytes[open_bracket - 1] == b'!'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_collapsed_reference_to_inline_when_definition_exists() {
        let rule = LinkStyleRule;
        let config = RuleConfig {
            enabled: true,
            properties: [("collapsed".to_string(), "false".to_string())]
                .into_iter()
                .collect(),
        };
        let diagnostics = rule.evaluate_configured(
            Path::new("doc.md"),
            "[inline](target)\n[ref][]\n[ref]: https://example.com\n",
            Some(&config),
        );

        assert_eq!(diagnostics.len(), 1);
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("collapsed reference should be fixable");
        assert_eq!(fix.replacement, "[ref](https://example.com)");
    }

    #[test]
    fn default_config_allows_collapsed_references() {
        let rule = LinkStyleRule;
        let diagnostics = rule.evaluate(
            Path::new("doc.md"),
            "[inline](target)\n[ref][]\n[ref]: https://example.com\n",
        );

        assert!(diagnostics.is_empty());
    }
}
