use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta, RuleConfig, RuleParityStatus,
};
use std::path::Path;

/* WHY: Section: Whitespace and blank-line markdownlint rule implementations
======================================================= */

/// MD012 / no-multiple-blanks — Multiple consecutive blank lines.
pub struct NoMultipleBlanksRule;

impl MarkdownRule for NoMultipleBlanksRule {
    fn id(&self) -> &'static str {
        "MD012"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD012")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD012");
        let mut diagnostics = Vec::new();
        let mut consecutive_blanks = 0;
        let ctx = DocumentContext::new(file_path, content);
        for (i, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(i) {
                consecutive_blanks = 0;
                continue;
            }
            let line = line.text;
            let trimmed = line.trim();
            if trimmed.is_empty() {
                consecutive_blanks += 1;
                if consecutive_blanks > 1 {
                    let fix = crate::rules::markdown::types::DiagnosticFix {
                        start_line: i + 1,
                        start_column: 1,
                        end_line: i + 2,
                        end_column: 1,
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
            } else {
                consecutive_blanks = 0;
            }
        }
        if content.ends_with("\n\n") && consecutive_blanks == 1 {
            diagnostics.push(MarkdownDiagnostic {
                file: file_path.to_path_buf(),
                severity: DiagnosticSeverity::Warning,
                range: DiagnosticRange {
                    start_line: ctx.lines().len() + 1,
                    start_column: 1,
                    end_line: ctx.lines().len() + 1,
                    end_column: 1,
                },
                message: meta.description.to_string(),
                rule_id: meta.code.to_string(),
                official_meta: Some(meta),
                fix_info: ctx.lines().last().map(|line| {
                    crate::rules::markdown::types::DiagnosticFix {
                        start_line: line.number,
                        start_column: 1,
                        end_line: line.number + 1,
                        end_column: 1,
                        replacement: String::new(),
                    }
                }),
            });
        }
        diagnostics
    }
}

/// MD027 / no-multiple-space-blockquote — Multiple spaces after blockquote symbol.
pub struct NoMultipleSpaceBlockquoteRule;

impl MarkdownRule for NoMultipleSpaceBlockquoteRule {
    fn id(&self) -> &'static str {
        "MD027"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD027",
            title: "no-multiple-space-blockquote",
            description: "Multiple spaces after blockquote symbol.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md027.md",
            aliases: &["no-multiple-space-blockquote"],
            parity: RuleParityStatus::Official,
            is_fixable: true,
            properties: &[crate::rule_prop!(
                Boolean,
                "list_items",
                "Include list items",
                "true"
            )],
        })
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        self.evaluate_with_list_items(file_path, content, true)
    }

    fn evaluate_configured(
        &self,
        file_path: &Path,
        content: &str,
        config: Option<&RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let include_list_items = config
            .and_then(|config| config.properties.get("list_items"))
            .map(|value| value != "false")
            .unwrap_or(true);
        self.evaluate_with_list_items(file_path, content, include_list_items)
    }
}

impl NoMultipleSpaceBlockquoteRule {
    fn evaluate_with_list_items(
        &self,
        file_path: &Path,
        content: &str,
        include_list_items: bool,
    ) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD027");
        let mut diagnostics = Vec::new();
        let ctx = DocumentContext::new(file_path, content);
        for (i, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(i) {
                continue;
            }
            let line = line.text;
            let trimmed = line.trim_start();
            let Some(after_marker) = trimmed.strip_prefix('>') else {
                continue;
            };
            if after_marker.starts_with("  ")
                && (include_list_items || !RuleHelpers::is_list_item(after_marker.trim_start()))
            {
                let gt_pos = line.find('>').unwrap();
                let spaces_start = gt_pos + 1;
                let mut spaces_end = spaces_start;
                while spaces_end < line.len() && line[spaces_end..].starts_with(' ') {
                    spaces_end += 1;
                }

                let fix = crate::rules::markdown::types::DiagnosticFix {
                    start_line: i + 1,
                    start_column: spaces_start + 1,
                    end_line: i + 1,
                    end_column: spaces_end + 1,
                    replacement: " ".to_string(),
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
        diagnostics
    }
}

/// MD047 / single-trailing-newline — Files should end with a single newline character.
pub struct SingleTrailingNewlineRule;

impl MarkdownRule for SingleTrailingNewlineRule {
    fn id(&self) -> &'static str {
        "MD047"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD047",
            title: "single-trailing-newline",
            description: "Files should end with a single newline character.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md047.md",
            aliases: &["single-trailing-newline"],
            parity: RuleParityStatus::Official,
            is_fixable: true,
            properties: &[],
        })
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD047");
        if content.is_empty() || content.ends_with('\n') {
            return Vec::new();
        }
        let line_count = content.lines().count();
        vec![MarkdownDiagnostic {
            file: file_path.to_path_buf(),
            severity: DiagnosticSeverity::Warning,
            range: DiagnosticRange {
                start_line: line_count,
                start_column: 1,
                end_line: line_count,
                end_column: 1,
            },
            message: meta.description.to_string(),
            rule_id: meta.code.to_string(),
            official_meta: Some(meta),
            fix_info: Some(crate::rules::markdown::types::DiagnosticFix {
                start_line: line_count,
                start_column: content.lines().last().map_or(1, |l| l.len() + 1),
                end_line: line_count,
                end_column: content.lines().last().map_or(1, |l| l.len() + 1),
                replacement: "\n".to_string(),
            }),
        }]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn md027_fixes_extra_spaces_after_blockquote_marker() {
        let rule = NoMultipleSpaceBlockquoteRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), ">  quote");

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0]
                .fix_info
                .as_ref()
                .expect("fix exists")
                .replacement,
            " "
        );
    }

    #[test]
    fn md027_can_ignore_list_items_when_configured() {
        let rule = NoMultipleSpaceBlockquoteRule;
        let config = RuleConfig {
            enabled: true,
            properties: HashMap::from([("list_items".to_string(), "false".to_string())]),
        };
        let diagnostics =
            rule.evaluate_configured(Path::new("doc.md"), ">  - nested item", Some(&config));

        assert!(diagnostics.is_empty());
    }
}
