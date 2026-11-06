use super::emphasis_style::evaluate_emphasis_style;
use crate::rules::markdown::inline::InlineEmphasisKind;
use crate::rules::markdown::{DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta};
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
        let ctx = DocumentContext::new(file_path, content);
        self.evaluate_context(&ctx, None)
    }

    fn evaluate_context(
        &self,
        ctx: &DocumentContext<'_>,
        config: Option<&RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        evaluate_emphasis_style(
            ctx,
            config,
            InlineEmphasisKind::Emphasis,
            self.official_meta().expect("always Some for MD049"),
        )
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

    #[test]
    fn ignores_emphasis_inside_fenced_code() {
        let rule = EmphasisStyleRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "```\n*one* and _two_\n```\n");

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn ignores_emphasis_inside_long_and_unclosed_code_spans() {
        let rule = EmphasisStyleRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "*one* ``_two_``\n`_three_\n");

        assert!(diagnostics.is_empty());
    }
}
