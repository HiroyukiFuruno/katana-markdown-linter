use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta, RuleConfig,
};
use std::path::Path;

/// MD010 / no-hard-tabs — Hard tabs.
pub struct NoHardTabsRule;

impl MarkdownRule for NoHardTabsRule {
    fn id(&self) -> &'static str {
        "MD010"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD010")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let ctx = DocumentContext::new(file_path, content);
        self.evaluate_context(&ctx, None)
    }

    fn evaluate_context(
        &self,
        ctx: &DocumentContext<'_>,
        config: Option<&RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD010");
        let options = NoHardTabsOptions::from_config(config);
        let mut diagnostics = Vec::new();
        for (i, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(i) && !options.code_blocks {
                continue;
            }
            for column in hard_tab_columns(line.text, !options.code_blocks) {
                diagnostics.push(MarkdownDiagnostic {
                    file: ctx.file_path().to_path_buf(),
                    severity: DiagnosticSeverity::Warning,
                    range: DiagnosticRange {
                        start_line: line.number,
                        start_column: column + 1,
                        end_line: line.number,
                        end_column: column + 2,
                    },
                    message: meta.description.to_string(),
                    rule_id: meta.code.to_string(),
                    official_meta: Some(meta.clone()),
                    fix_info: Some(crate::rules::markdown::types::DiagnosticFix {
                        start_line: line.number,
                        start_column: column + 1,
                        end_line: line.number,
                        end_column: column + 2,
                        replacement: " ".repeat(options.spaces_per_tab),
                    }),
                });
            }
        }
        diagnostics
    }
}

struct NoHardTabsOptions {
    code_blocks: bool,
    spaces_per_tab: usize,
}

impl NoHardTabsOptions {
    fn from_config(config: Option<&RuleConfig>) -> Self {
        Self {
            code_blocks: config
                .and_then(|config| config.properties.get("code_blocks"))
                .and_then(|value| value.parse().ok())
                .unwrap_or(true),
            spaces_per_tab: config
                .and_then(|config| config.properties.get("spaces_per_tab"))
                .and_then(|value| value.parse().ok())
                .unwrap_or(1),
        }
    }
}

fn hard_tab_columns(line: &str, skip_code_spans: bool) -> Vec<usize> {
    let mut columns = Vec::new();
    let mut in_code = false;
    for (idx, byte) in line.bytes().enumerate() {
        match byte {
            b'`' if skip_code_spans => in_code = !in_code,
            b'\t' if !in_code => columns.push(idx),
            _ => {}
        }
    }
    columns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_hard_tabs_with_spaces() {
        let rule = NoHardTabsRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "a\tb\tc");

        assert_eq!(diagnostics.len(), 2);
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("hard tab should be fixable");
        assert_eq!(fix.start_column, 2);
        assert_eq!(fix.end_column, 3);
        assert_eq!(fix.replacement, " ");
    }

    #[test]
    fn can_ignore_code_blocks_and_code_spans() {
        let rule = NoHardTabsRule;
        let config = RuleConfig {
            enabled: true,
            properties: [("code_blocks".to_string(), "false".to_string())]
                .into_iter()
                .collect(),
        };
        let diagnostics = rule.evaluate_context(
            &DocumentContext::new(Path::new("doc.md"), "```\na\tb\n```\n`a\tb`\n"),
            Some(&config),
        );

        assert!(diagnostics.is_empty());
    }
}
