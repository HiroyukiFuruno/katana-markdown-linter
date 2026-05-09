use super::shared::leading_spaces;
use crate::rules::markdown::{
    DiagnosticFix, DiagnosticRange, DiagnosticSeverity, DocumentContext, LineInfo,
    MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta, RuleParityStatus,
};
use crate::types::RuleConfig;
use std::path::Path;

const MIN_HR_CHARS: usize = 3;

pub struct HrStyleRule;

impl MarkdownRule for HrStyleRule {
    fn id(&self) -> &'static str {
        "MD035"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        Some(OfficialRuleMeta {
            code: "MD035",
            title: "hr-style",
            description: "Horizontal rule style.",
            docs_url: "https://github.com/DavidAnson/markdownlint/blob/main/doc/md035.md",
            aliases: &["hr-style"],
            parity: RuleParityStatus::Official,
            is_fixable: true,
            properties: &[crate::rule_prop!(
                String,
                "style",
                "Horizontal rule style",
                "consistent"
            )],
        })
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
        let meta = self.official_meta().expect("always Some for MD035");
        let mut diagnostics = Vec::new();
        let configured = configured_hr_style(config);
        let mut first_rule: Option<String> = None;
        for (index, line) in ctx.lines().iter().enumerate() {
            push_hr_line_diagnostic(
                &mut diagnostics,
                ctx,
                index,
                line,
                &configured,
                &mut first_rule,
                &meta,
            );
        }
        diagnostics
    }
}

fn push_hr_line_diagnostic(
    diagnostics: &mut Vec<MarkdownDiagnostic>,
    ctx: &DocumentContext<'_>,
    index: usize,
    line: &LineInfo<'_>,
    configured: &Option<String>,
    first_rule: &mut Option<String>,
    meta: &OfficialRuleMeta,
) {
    if ctx.is_code_line(index) || is_front_matter_line(ctx, line.content_range.start) {
        return;
    }
    let trimmed = line.text.trim();
    if !is_horizontal_rule(trimmed) {
        return;
    }
    let expected = match configured {
        Some(style) => style.as_str(),
        None => first_rule
            .get_or_insert_with(|| trimmed.to_string())
            .as_str(),
    };
    if trimmed == expected {
        return;
    }
    let replacement = is_horizontal_rule(expected).then(|| expected.to_string());
    push_hr_diagnostic(diagnostics, ctx, index, line.text, meta, replacement);
}

fn configured_hr_style(config: Option<&RuleConfig>) -> Option<String> {
    config
        .and_then(|config| config.properties.get("style"))
        .filter(|style| style.as_str() != "consistent")
        .cloned()
}

fn is_front_matter_line(ctx: &DocumentContext<'_>, offset: usize) -> bool {
    ctx.front_matter()
        .is_some_and(|range| offset >= range.start && offset < range.end)
}

fn push_hr_diagnostic(
    diagnostics: &mut Vec<MarkdownDiagnostic>,
    ctx: &DocumentContext<'_>,
    line_idx: usize,
    line: &str,
    meta: &OfficialRuleMeta,
    replacement: Option<String>,
) {
    diagnostics.push(MarkdownDiagnostic {
        file: ctx.file_path().to_path_buf(),
        severity: DiagnosticSeverity::Warning,
        range: DiagnosticRange {
            start_line: line_idx + 1,
            start_column: 1,
            end_line: line_idx + 1,
            end_column: line.len().max(1),
        },
        message: meta.description.to_string(),
        rule_id: meta.code.to_string(),
        official_meta: Some(meta.clone()),
        fix_info: replacement.map(|replacement| DiagnosticFix {
            start_line: line_idx + 1,
            start_column: 1,
            end_line: line_idx + 1,
            end_column: line.len() + 1,
            replacement: format!("{}{}", leading_spaces(line), replacement),
        }),
    });
}

fn is_horizontal_rule(trimmed: &str) -> bool {
    if trimmed.len() < MIN_HR_CHARS {
        return false;
    }
    let chars: Vec<char> = trimmed
        .chars()
        .filter(|char| !char.is_whitespace())
        .collect();
    if chars.len() < MIN_HR_CHARS {
        return false;
    }
    let first = chars[0];
    (first == '-' || first == '*' || first == '_') && chars.iter().all(|char| *char == first)
}
