use super::closed_atx::check_closed_atx_spacing;
use crate::rules::markdown::{
    DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta, RuleConfig,
};
use std::path::Path;

/// MD021 / no-multiple-space-closed-atx — Multiple spaces inside hashes on closed atx heading.
pub struct NoMultipleSpaceClosedAtxRule;

impl MarkdownRule for NoMultipleSpaceClosedAtxRule {
    fn id(&self) -> &'static str {
        "MD021"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD021")
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
        let meta = self.official_meta().expect("always Some for MD021");
        check_closed_atx_spacing(ctx, &meta, |leading, trailing| leading > 1 || trailing > 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_multiple_spaces_inside_closed_atx_hashes() {
        let rule = NoMultipleSpaceClosedAtxRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "#  Heading  #\n##  Heading ##");

        assert_eq!(diagnostics.len(), 2);
        assert!(diagnostics
            .iter()
            .all(|diagnostic| diagnostic.fix_info.is_some()));
        assert_eq!(
            diagnostics[0]
                .fix_info
                .as_ref()
                .expect("fix exists")
                .replacement,
            "# Heading #"
        );
        assert_eq!(
            diagnostics[1]
                .fix_info
                .as_ref()
                .expect("fix exists")
                .replacement,
            "## Heading ##"
        );
    }

    #[test]
    fn accepts_closed_atx_with_single_spaces() {
        let rule = NoMultipleSpaceClosedAtxRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "# Heading #");

        assert!(diagnostics.is_empty());
    }
}
