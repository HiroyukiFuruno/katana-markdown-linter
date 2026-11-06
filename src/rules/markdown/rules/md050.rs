use super::emphasis_style::evaluate_emphasis_style;
use crate::rules::markdown::inline::InlineEmphasisKind;
use crate::rules::markdown::{DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta};
use crate::types::RuleConfig;
use std::path::Path;

/// MD050 / strong-style — Strong style.
pub struct StrongStyleRule;

impl MarkdownRule for StrongStyleRule {
    fn id(&self) -> &'static str {
        "MD050"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD050")
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
            InlineEmphasisKind::Strong,
            self.official_meta().expect("always Some for MD050"),
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
mod tests;
