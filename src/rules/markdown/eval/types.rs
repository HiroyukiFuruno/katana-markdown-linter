use crate::rules::markdown::{DocumentContext, MarkdownDiagnostic, OfficialRuleMeta};

pub struct MarkdownLinterOps;

#[derive(Clone, Copy)]
pub struct RuleEntry {
    pub(super) id: &'static str,
    pub(super) official_meta: fn() -> Option<OfficialRuleMeta>,
    pub(super) evaluate_context:
        fn(&DocumentContext<'_>, Option<&crate::RuleConfig>) -> Vec<MarkdownDiagnostic>,
}

impl RuleEntry {
    pub fn id(&self) -> &'static str {
        self.id
    }

    pub fn official_meta(&self) -> Option<OfficialRuleMeta> {
        (self.official_meta)()
    }

    pub fn evaluate_context(
        &self,
        ctx: &DocumentContext<'_>,
        config: Option<&crate::RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        (self.evaluate_context)(ctx, config)
    }
}
