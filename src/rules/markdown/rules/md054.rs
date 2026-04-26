use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta,
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
        let ctx = DocumentContext::new(file_path, content);
        self.evaluate_context(&ctx, config)
    }

    fn evaluate_context(
        &self,
        ctx: &DocumentContext<'_>,
        config: Option<&RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        if collapsed_references_allowed(config) {
            return Vec::new();
        }

        let meta = self.official_meta().expect("always Some for MD054");
        let has_inline = ctx.inline_links().iter().any(|link| link.kind.is_inline());
        let definitions = link_definitions(ctx);
        let has_reference = !definitions.is_empty()
            || ctx
                .inline_links()
                .iter()
                .any(|link| link.kind.is_reference());
        if !has_inline || !has_reference {
            return Vec::new();
        }

        let mut diagnostics = Vec::new();
        for reference in ctx
            .inline_links()
            .iter()
            .filter(|link| link.kind.is_collapsed_reference())
        {
            let Some(label) = reference.effective_label() else {
                continue;
            };
            let destination = definitions.get(&label.to_lowercase());
            let range = ctx.diagnostic_range(reference.full_range);
            diagnostics.push(MarkdownDiagnostic {
                file: ctx.file_path().to_path_buf(),
                severity: DiagnosticSeverity::Warning,
                range: DiagnosticRange {
                    start_line: range.start_line,
                    start_column: range.start_column,
                    end_line: range.end_line,
                    end_column: range.end_column,
                },
                message: meta.description.to_string(),
                rule_id: meta.code.to_string(),
                official_meta: Some(meta.clone()),
                fix_info: destination.map(|destination| {
                    crate::rules::markdown::types::DiagnosticFix {
                        start_line: range.start_line,
                        start_column: range.start_column,
                        end_line: range.end_line,
                        end_column: range.end_column,
                        replacement: format!("[{label}]({destination})"),
                    }
                }),
            });
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

fn link_definitions(ctx: &DocumentContext<'_>) -> HashMap<String, String> {
    let mut definitions = HashMap::new();
    for definition in ctx.reference_definitions() {
        if !definition.label.is_empty() && !definition.destination.is_empty() {
            definitions.insert(
                definition.label.to_lowercase(),
                definition.destination.to_string(),
            );
        }
    }
    definitions
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
