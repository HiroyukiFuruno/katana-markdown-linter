mod indented_code;
mod shortcut;

use self::indented_code::indented_code_line_indexes;
use self::shortcut::shortcut_references;
use crate::rules::markdown::document::SourceRange;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
    RuleConfig,
};
use std::collections::HashSet;
use std::path::Path;

/// MD052 / reference-links-images — Reference links and images.
pub struct ReferenceLinksImagesRule;

impl MarkdownRule for ReferenceLinksImagesRule {
    fn id(&self) -> &'static str {
        "MD052"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD052")
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
        let meta = self.official_meta().expect("always Some for MD052");
        let definitions = reference_definition_labels(ctx);
        let ignored_labels = ignored_labels(_config);
        let include_shortcut = shortcut_syntax(_config);
        let indented_code_lines = indented_code_line_indexes(ctx);
        let mut diagnostics = Vec::new();
        for link in ctx.inline_links() {
            if !link.kind.is_reference() {
                continue;
            }
            if indented_code_lines.contains(&link.line) {
                continue;
            }
            let Some(label) = link.effective_label() else {
                continue;
            };
            if should_report_label(label, &definitions, &ignored_labels) {
                diagnostics.push(diagnostic_for_range(ctx, &meta, link.full_range));
            }
        }
        if include_shortcut {
            for shortcut in shortcut_references(ctx, &indented_code_lines) {
                if should_report_label(shortcut.label, &definitions, &ignored_labels) {
                    diagnostics.push(diagnostic_for_range(ctx, &meta, shortcut.range));
                }
            }
        }
        diagnostics
    }
}

fn reference_definition_labels(ctx: &DocumentContext<'_>) -> HashSet<String> {
    ctx.reference_definitions()
        .iter()
        .map(|definition| normalize_label(definition.label))
        .collect()
}

fn ignored_labels(config: Option<&RuleConfig>) -> HashSet<String> {
    config
        .and_then(|config| config.properties.get("ignored_labels"))
        .and_then(|raw| serde_json::from_str::<Vec<String>>(raw).ok())
        .unwrap_or_else(|| vec!["x".to_string()])
        .into_iter()
        .map(|label| normalize_label(&label))
        .collect()
}

fn shortcut_syntax(config: Option<&RuleConfig>) -> bool {
    config
        .and_then(|config| config.properties.get("shortcut_syntax"))
        .and_then(|value| value.parse::<bool>().ok())
        .unwrap_or(false)
}

fn should_report_label(
    label: &str,
    definitions: &HashSet<String>,
    ignored_labels: &HashSet<String>,
) -> bool {
    let normalized = normalize_label(label);
    !normalized.is_empty()
        && !definitions.contains(&normalized)
        && !ignored_labels.contains(&normalized)
}

fn normalize_label(label: &str) -> String {
    label
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

fn diagnostic_for_range(
    ctx: &DocumentContext<'_>,
    meta: &OfficialRuleMeta,
    source_range: SourceRange,
) -> MarkdownDiagnostic {
    MarkdownDiagnostic {
        file: ctx.file_path().to_path_buf(),
        severity: DiagnosticSeverity::Warning,
        range: ctx.diagnostic_range(source_range),
        message: meta.description.to_string(),
        rule_id: meta.code.to_string(),
        official_meta: Some(meta.clone()),
        fix_info: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_collapsed_reference_links_and_images() {
        let rule = ReferenceLinksImagesRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "[ref][] and ![alt][]\n");

        assert_eq!(diagnostics.len(), 2);
        assert!(diagnostics
            .iter()
            .all(|diagnostic| diagnostic.rule_id == "MD052"));
    }

    #[test]
    fn ignores_full_references_and_code_spans() {
        let rule = ReferenceLinksImagesRule;
        let content = concat!(
            "[ref][label] and ![alt][image]\n",
            "`[ref][]`\n",
            "[label]: https://example.com\n",
            "[image]: https://example.org/image.png\n",
        );
        let diagnostics = rule.evaluate(Path::new("doc.md"), content);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn reports_missing_full_references() {
        let rule = ReferenceLinksImagesRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "[ref][missing]\n");

        assert_eq!(diagnostics.len(), 1);
        assert!(diagnostics[0].fix_info.is_none());
    }

    #[test]
    fn ignores_defined_collapsed_reference() {
        let rule = ReferenceLinksImagesRule;
        let diagnostics = rule.evaluate(
            Path::new("doc.md"),
            "[ref][]\n\n[ref]: https://example.com\n",
        );

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn is_not_fixable_in_catalog() {
        let rule = ReferenceLinksImagesRule;
        let meta = rule.official_meta().expect("meta must be Some");
        assert!(!meta.is_fixable, "MD052 must not advertise a safe fix");
    }
}
