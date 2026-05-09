mod evaluate;
mod format;
mod matching;
mod options;

use crate::rules::markdown::{DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta};
use crate::types::RuleConfig;
use options::TableStyleOptions;
use std::path::Path;

pub struct TableColumnStyleRule;

impl MarkdownRule for TableColumnStyleRule {
    fn id(&self) -> &'static str {
        "MD060"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD060")
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
        let meta = self.official_meta().expect("always Some for MD060");
        let options = TableStyleOptions::from_config(config);
        ctx.tables()
            .iter()
            .filter_map(|table| evaluate::evaluate_table(ctx, &meta, &options, table))
            .collect()
    }
}

#[cfg(test)]
mod tests;
