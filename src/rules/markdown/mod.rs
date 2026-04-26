mod types;
pub use types::*;
pub mod broken_link;
pub mod catalog;
pub mod document;
pub use broken_link::*;
pub use document::*;
pub mod inline;
pub use inline::*;

use crate::types::RuleConfig;
use std::path::Path;

pub trait MarkdownRule: Send + Sync {
    fn id(&self) -> &'static str;
    /// Returns official markdownlint metadata for this rule, if any.
    /// `None` means the rule is hidden (internal-only).
    fn official_meta(&self) -> Option<OfficialRuleMeta>;
    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic>;

    fn evaluate_context(
        &self,
        ctx: &DocumentContext<'_>,
        config: Option<&RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        self.evaluate_configured(ctx.file_path(), ctx.content(), config)
    }

    fn evaluate_configured(
        &self,
        file_path: &Path,
        content: &str,
        _config: Option<&RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        self.evaluate(file_path, content)
    }
}

/* WHY: Section: Official rule implementations
======================================================= */

/// MD001 / heading-increment — Heading levels should only increment by one level at a time.
pub struct HeadingIncrementRule;

impl MarkdownRule for HeadingIncrementRule {
    fn id(&self) -> &'static str {
        "MD001"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD001")
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
        let meta = self
            .official_meta()
            .expect("official_meta is always Some for MD001");
        let mut diagnostics = Vec::new();
        let mut last_level = 0;

        for heading in ctx.headings() {
            let current_level = heading.level;
            if last_level > 0 && current_level > last_level + 1 {
                let line = &ctx.lines()[heading.line];
                diagnostics.push(MarkdownDiagnostic {
                    file: ctx.file_path().to_path_buf(),
                    severity: DiagnosticSeverity::Warning,
                    range: DiagnosticRange {
                        start_line: line.number,
                        start_column: 1,
                        end_line: line.number,
                        end_column: line.text.len(),
                    },
                    message: format!(
                        "{} [Expected: h{}, Actual: h{}]",
                        meta.description,
                        last_level + 1,
                        current_level
                    ),
                    rule_id: meta.code.to_string(),
                    official_meta: Some(meta.clone()),
                    fix_info: None,
                });
            }
            last_level = current_level;
        }
        diagnostics
    }
}

/* WHY: Section: Legacy re-export for backward compatibility
=======================================================
 Downstream code (katana-ui) references HeadingStructureRule by name.
 Re-export as a public alias so callers compile until migrated to HeadingIncrementRule. */
pub use HeadingIncrementRule as HeadingStructureRule;

#[macro_use]
pub mod macros;
#[rustfmt::skip]
pub mod stubs;
pub mod stubs_regex;
pub use stubs_regex::*;

pub mod helpers;

/* WHY: Rule implementations live in rules/ subdirectory for clean separation */
pub mod rules;
pub use rules::*;

pub mod eval;
pub use eval::*;

pub mod config;
pub use config::*;
