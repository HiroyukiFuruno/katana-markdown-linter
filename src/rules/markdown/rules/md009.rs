use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta, RuleConfig,
};
use std::path::Path;

/// MD009 / no-trailing-spaces — Trailing spaces.
pub struct NoTrailingSpacesRule;

impl MarkdownRule for NoTrailingSpacesRule {
    fn id(&self) -> &'static str {
        "MD009"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD009")
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
        let meta = self.official_meta().expect("always Some for MD009");
        let options = NoTrailingSpacesOptions::from_config(config);
        let mut diagnostics = Vec::new();
        for (i, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(i) && !options.code_blocks {
                continue;
            }
            let trailing_spaces = line.text.len() - line.text.trim_end_matches(' ').len();
            if trailing_spaces == 0 || options.allows_trailing_spaces(trailing_spaces) {
                continue;
            }
            let trimmed_len = line.text.len() - trailing_spaces;

            diagnostics.push(MarkdownDiagnostic {
                file: ctx.file_path().to_path_buf(),
                severity: DiagnosticSeverity::Warning,
                range: DiagnosticRange {
                    start_line: line.number,
                    start_column: trimmed_len + 1,
                    end_line: line.number,
                    end_column: line.text.len() + 1,
                },
                message: meta.description.to_string(),
                rule_id: meta.code.to_string(),
                official_meta: Some(meta.clone()),
                fix_info: Some(crate::rules::markdown::types::DiagnosticFix {
                    start_line: line.number,
                    start_column: trimmed_len + 1,
                    end_line: line.number,
                    end_column: line.text.len() + 1,
                    replacement: String::new(),
                }),
            });
        }
        diagnostics
    }
}

struct NoTrailingSpacesOptions {
    br_spaces: usize,
    code_blocks: bool,
    strict: bool,
}

impl NoTrailingSpacesOptions {
    fn from_config(config: Option<&RuleConfig>) -> Self {
        Self {
            br_spaces: config
                .and_then(|config| config.properties.get("br_spaces"))
                .and_then(|value| value.parse().ok())
                .unwrap_or(2),
            code_blocks: config
                .and_then(|config| config.properties.get("code_blocks"))
                .and_then(|value| value.parse().ok())
                .unwrap_or(false),
            strict: config
                .and_then(|config| config.properties.get("strict"))
                .and_then(|value| value.parse().ok())
                .unwrap_or(false),
        }
    }

    fn allows_trailing_spaces(&self, trailing_spaces: usize) -> bool {
        !self.strict && self.br_spaces >= 2 && trailing_spaces == self.br_spaces
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_trailing_spaces() {
        let rule = NoTrailingSpacesRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "text   \nnext");

        assert_eq!(diagnostics.len(), 1);
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("trailing spaces should be fixable");
        assert_eq!(fix.start_column, 5);
        assert_eq!(fix.end_column, 8);
        assert_eq!(fix.replacement, "");
    }

    #[test]
    fn allows_two_space_break_and_code_blocks_by_default() {
        let rule = NoTrailingSpacesRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "text  \n```\ncode  \n```\n");

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn can_include_code_blocks_when_configured() {
        let rule = NoTrailingSpacesRule;
        let config = RuleConfig {
            enabled: true,
            properties: [("code_blocks".to_string(), "true".to_string())]
                .into_iter()
                .collect(),
        };
        let diagnostics = rule.evaluate_context(
            &DocumentContext::new(Path::new("doc.md"), "```\ncode   \n```\n"),
            Some(&config),
        );

        assert_eq!(diagnostics.len(), 1);
    }
}
