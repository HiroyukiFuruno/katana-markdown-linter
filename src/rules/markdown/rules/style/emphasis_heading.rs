use super::shared::leading_spaces;
use crate::rules::markdown::document::LineInfo;
use crate::rules::markdown::{
    DiagnosticFix, DiagnosticRange, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic,
    MarkdownRule, OfficialRuleMeta,
};
use crate::types::RuleConfig;
use std::path::Path;

pub struct NoEmphasisAsHeadingRule;

impl MarkdownRule for NoEmphasisAsHeadingRule {
    fn id(&self) -> &'static str {
        "MD036"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD036")
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
        let meta = self.official_meta().expect("always Some for MD036");
        let mut diagnostics = Vec::new();
        let punctuation = configured_md036_punctuation(config);
        let ctx_lines = ctx.lines();
        for (index, line_info) in ctx_lines.iter().enumerate() {
            if ctx.is_code_line(index) {
                continue;
            }
            push_emphasis_heading_diagnostic(
                &mut diagnostics,
                ctx,
                ctx_lines,
                index,
                line_info.text,
                &punctuation,
                &meta,
            );
        }
        diagnostics
    }
}

fn push_emphasis_heading_diagnostic(
    diagnostics: &mut Vec<MarkdownDiagnostic>,
    ctx: &DocumentContext<'_>,
    ctx_lines: &[LineInfo<'_>],
    index: usize,
    line: &str,
    punctuation: &str,
    meta: &OfficialRuleMeta,
) {
    let trimmed = line.trim();
    let Some(heading_text) = emphasis_heading_text(trimmed, ctx_lines, index, punctuation) else {
        return;
    };
    diagnostics.push(MarkdownDiagnostic {
        file: ctx.file_path().to_path_buf(),
        severity: DiagnosticSeverity::Warning,
        range: DiagnosticRange {
            start_line: index + 1,
            start_column: 1,
            end_line: index + 1,
            end_column: line.len().max(1),
        },
        message: meta.description.to_string(),
        rule_id: meta.code.to_string(),
        official_meta: Some(meta.clone()),
        fix_info: Some(DiagnosticFix {
            start_line: index + 1,
            start_column: 1,
            end_line: index + 1,
            end_column: line.len() + 1,
            replacement: format!("{}# {}", leading_spaces(line), heading_text),
        }),
    });
}

fn configured_md036_punctuation(config: Option<&RuleConfig>) -> String {
    config
        .and_then(|config| config.properties.get("punctuation"))
        .cloned()
        .unwrap_or_else(|| ".,;:!?。，；：！？".to_string())
}

fn emphasis_heading_text<'a>(
    trimmed: &'a str,
    lines: &[LineInfo<'_>],
    index: usize,
    punctuation: &str,
) -> Option<&'a str> {
    let heading_text = ["**", "__", "*", "_"].into_iter().find_map(|marker| {
        let text = trimmed.strip_prefix(marker)?.strip_suffix(marker)?;
        (!text.is_empty()).then_some(text)
    })?;
    if heading_text.trim().is_empty() {
        return None;
    }
    if heading_text
        .chars()
        .last()
        .is_some_and(|char| punctuation.contains(char))
    {
        return None;
    }
    if contains_inline_markdown_token(heading_text) {
        return None;
    }
    let blank_before = index == 0 || lines[index - 1].text.trim().is_empty();
    let blank_after = index + 1 >= lines.len() || lines[index + 1].text.trim().is_empty();
    (blank_before && blank_after).then_some(heading_text)
}

fn contains_inline_markdown_token(text: &str) -> bool {
    text.contains(['`', '[', ']', '<', '>'])
}
