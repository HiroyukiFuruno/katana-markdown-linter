use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use crate::types::RuleConfig;
use std::path::Path;

/// MD044 / proper-names — Proper names.
pub struct ProperNamesRule;

impl MarkdownRule for ProperNamesRule {
    fn id(&self) -> &'static str {
        "MD044"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD044")
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
        let meta = self.official_meta().expect("always Some for MD044");
        let names = configured_names(config);
        if names.is_empty() {
            return Vec::new();
        }

        let mut diagnostics = Vec::new();
        for (i, line) in content.lines().enumerate() {
            for (start, end, replacement) in proper_name_fixes(line, &names) {
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
                        replacement,
                    }),
                });
            }
        }
        diagnostics
    }
}

fn configured_names(config: Option<&RuleConfig>) -> Vec<String> {
    let Some(raw) = config.and_then(|config| config.properties.get("names")) else {
        return Vec::new();
    };
    serde_json::from_str::<Vec<String>>(raw).unwrap_or_default()
}

fn proper_name_fixes(line: &str, names: &[String]) -> Vec<(usize, usize, String)> {
    let mut fixes = Vec::new();
    for correct in names {
        let incorrect = correct.to_lowercase();
        for (start, matched) in line.match_indices(&incorrect) {
            if matched == correct {
                continue;
            }
            let end = start + matched.len();
            if is_word_boundary(line, start, end) {
                fixes.push((start, end, correct.clone()));
            }
        }
    }
    fixes
}

fn is_word_boundary(line: &str, start: usize, end: usize) -> bool {
    let bytes = line.as_bytes();
    let before = start
        .checked_sub(1)
        .and_then(|idx| bytes.get(idx))
        .is_some_and(|byte| byte.is_ascii_alphanumeric() || *byte == b'_');
    let after = bytes
        .get(end)
        .is_some_and(|byte| byte.is_ascii_alphanumeric() || *byte == b'_');
    !before && !after
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_known_proper_name_capitalization() {
        let rule = ProperNamesRule;
        let config = RuleConfig {
            enabled: true,
            properties: [(
                "names".to_string(),
                "[\"Markdownlint\",\"GitHub\"]".to_string(),
            )]
            .into_iter()
            .collect(),
        };
        let diagnostics = rule.evaluate_configured(
            Path::new("doc.md"),
            "markdownlint and github",
            Some(&config),
        );

        assert_eq!(diagnostics.len(), 2);
        let replacements = diagnostics
            .iter()
            .filter_map(|diagnostic| diagnostic.fix_info.as_ref())
            .map(|fix| fix.replacement.as_str())
            .collect::<Vec<_>>();
        assert_eq!(replacements, ["Markdownlint", "GitHub"]);
    }

    #[test]
    fn default_config_does_not_guess_proper_names() {
        let rule = ProperNamesRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "markdownlint and github");

        assert!(diagnostics.is_empty());
    }
}
