use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
    RuleConfig, RuleParityStatus,
};
use std::path::Path;

pub struct NoMultipleSpaceBlockquoteRule;

impl MarkdownRule for NoMultipleSpaceBlockquoteRule {
    fn id(&self) -> &'static str {
        "MD027"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD027",
            title: "no-multiple-space-blockquote",
            description: "Multiple spaces after blockquote symbol.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md027.md",
            aliases: &["no-multiple-space-blockquote"],
            parity: RuleParityStatus::Official,
            is_fixable: true,
            properties: &[crate::rule_prop!(
                Boolean,
                "list_items",
                "Include list items",
                "true"
            )],
        })
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        self.evaluate_with_list_items(file_path, content, true)
    }

    fn evaluate_configured(
        &self,
        file_path: &Path,
        content: &str,
        config: Option<&RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let include_list_items = config
            .and_then(|config| config.properties.get("list_items"))
            .map(|value| value != "false")
            .unwrap_or(true);
        self.evaluate_with_list_items(file_path, content, include_list_items)
    }
}

impl NoMultipleSpaceBlockquoteRule {
    fn evaluate_with_list_items(
        &self,
        file_path: &Path,
        content: &str,
        include_list_items: bool,
    ) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD027");
        let mut diagnostics = Vec::new();
        let ctx = DocumentContext::new(file_path, content);
        for (index, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(index) {
                continue;
            }
            push_blockquote_diagnostic(
                &mut diagnostics,
                file_path,
                index,
                line.text,
                &meta,
                include_list_items,
            );
        }
        diagnostics
    }
}

fn push_blockquote_diagnostic(
    diagnostics: &mut Vec<MarkdownDiagnostic>,
    file_path: &Path,
    index: usize,
    line: &str,
    meta: &OfficialRuleMeta,
    include_list_items: bool,
) {
    let Some(after_marker) = line.trim_start().strip_prefix('>') else {
        return;
    };
    if !should_report(after_marker, include_list_items) {
        return;
    }
    let gt_pos = line.find('>').expect("blockquote marker exists");
    let spaces_start = gt_pos + 1;
    let mut spaces_end = spaces_start;
    while spaces_end < line.len() && line[spaces_end..].starts_with(' ') {
        spaces_end += 1;
    }
    let fix = crate::rules::markdown::types::DiagnosticFix {
        start_line: index + 1,
        start_column: spaces_start + 1,
        end_line: index + 1,
        end_column: spaces_end + 1,
        replacement: " ".to_string(),
    };
    RuleHelpers::push_diag_with_fix(
        diagnostics,
        file_path,
        index,
        line,
        meta,
        DiagnosticSeverity::Warning,
        Some(fix),
    );
}

fn should_report(after_marker: &str, include_list_items: bool) -> bool {
    after_marker.starts_with("  ")
        && (include_list_items || !RuleHelpers::is_list_item(after_marker.trim_start()))
}
