use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
    RuleConfig,
};
use std::path::Path;

/// MD053 / link-definitions — Link definitions.
pub struct LinkDefinitionsRule;

impl MarkdownRule for LinkDefinitionsRule {
    fn id(&self) -> &'static str {
        "MD053"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD053")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let ctx = DocumentContext::new(file_path, content);
        self.evaluate_context(&ctx, None)
    }

    fn evaluate_context(
        &self,
        ctx: &DocumentContext<'_>,
        _config: Option<&RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD053");
        let mut diagnostics = Vec::new();
        let mut seen = std::collections::HashSet::new();
        for definition in ctx.reference_definitions() {
            if !seen.insert(definition.label.to_lowercase()) {
                let line = &ctx.lines()[definition.line];
                let fix = crate::rules::markdown::types::DiagnosticFix {
                    start_line: line.number,
                    start_column: 1,
                    end_line: line.number + 1,
                    end_column: 1,
                    replacement: String::new(),
                };
                RuleHelpers::push_diag_with_fix(
                    &mut diagnostics,
                    ctx.file_path(),
                    definition.line,
                    line.text,
                    &meta,
                    DiagnosticSeverity::Warning,
                    Some(fix),
                );
            }
        }
        diagnostics
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_duplicate_link_definition_by_removing_later_definition() {
        let rule = LinkDefinitionsRule;
        let diagnostics = rule.evaluate(
            Path::new("doc.md"),
            "[one]: https://example.com/1\n[one]: https://example.com/2\nText\n",
        );

        assert_eq!(diagnostics.len(), 1);
        let fix = diagnostics[0]
            .fix_info
            .as_ref()
            .expect("duplicate link definition should be fixable");
        assert_eq!(fix.start_line, 2);
        assert_eq!(fix.end_line, 3);
        assert_eq!(fix.replacement, "");
    }
}
