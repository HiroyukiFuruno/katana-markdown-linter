use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticFix, DiagnosticRange, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic,
    MarkdownRule, OfficialRuleMeta, RuleParityStatus,
};
use crate::types::RuleConfig;
use std::path::Path;

/* WHY: Section: Style/emphasis markdownlint rule implementations
======================================================= */

/* WHY: Minimum marker length for bold/italic (e.g. `**x**` = 5 chars, so content must be > 4) */
const MIN_BOLD_LEN: usize = 4;
/* WHY: Minimum char count for a valid horizontal rule (---) */
const MIN_HR_CHARS: usize = 3;

/// MD036 / no-emphasis-as-heading — Emphasis used instead of a heading.
pub struct NoEmphasisAsHeadingRule;

impl MarkdownRule for NoEmphasisAsHeadingRule {
    fn id(&self) -> &'static str {
        "MD036"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD036")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD036");
        let mut diagnostics = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut in_code_block = false;
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if RuleHelpers::is_fence(trimmed) {
                in_code_block = !in_code_block;
                continue;
            }
            if in_code_block {
                continue;
            }
            if is_emphasis_heading(trimmed, &lines, i) {
                RuleHelpers::push_diag(
                    &mut diagnostics,
                    file_path,
                    i,
                    line,
                    &meta,
                    DiagnosticSeverity::Warning,
                );
            }
        }
        diagnostics
    }
}

/// MD035 / hr-style — Horizontal rule style should be consistent.
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
        for (idx, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(idx) || is_front_matter_line(ctx, line.content_range.start) {
                continue;
            }
            let trimmed = line.text.trim();
            if is_horizontal_rule(trimmed) {
                let expected = match &configured {
                    Some(style) => style.as_str(),
                    None => first_rule
                        .get_or_insert_with(|| trimmed.to_string())
                        .as_str(),
                };
                if trimmed == expected {
                    continue;
                }
                let replacement = is_horizontal_rule(expected).then(|| expected.to_string());
                push_hr_diagnostic(&mut diagnostics, ctx, idx, line.text, &meta, replacement);
            }
        }
        diagnostics
    }
}

/* WHY: Section: Private helpers
======================================================= */

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

fn leading_spaces(line: &str) -> &str {
    let count = line.len() - line.trim_start_matches(' ').len();
    &line[..count]
}

fn is_emphasis_heading(trimmed: &str, lines: &[&str], idx: usize) -> bool {
    let is_bold = (trimmed.starts_with("**")
        && trimmed.ends_with("**")
        && trimmed.len() > MIN_BOLD_LEN)
        || (trimmed.starts_with("__") && trimmed.ends_with("__") && trimmed.len() > MIN_BOLD_LEN);
    if !is_bold {
        return false;
    }
    let blank_before = idx == 0 || lines[idx - 1].trim().is_empty();
    let blank_after = idx + 1 >= lines.len() || lines[idx + 1].trim().is_empty();
    blank_before && blank_after
}

fn is_horizontal_rule(trimmed: &str) -> bool {
    if trimmed.len() < MIN_HR_CHARS {
        return false;
    }
    let chars: Vec<char> = trimmed.chars().filter(|c| !c.is_whitespace()).collect();
    if chars.len() < MIN_HR_CHARS {
        return false;
    }
    let first = chars[0];
    (first == '-' || first == '*' || first == '_') && chars.iter().all(|c| *c == first)
}

#[cfg(test)]
mod tests {
    use crate::{fix_with_results, lint, LintOptions, RuleConfig};
    use std::collections::HashMap;

    fn md035_options(style: &str) -> LintOptions {
        let mut rules = HashMap::new();
        rules.insert(
            "MD035".to_string(),
            RuleConfig {
                enabled: true,
                properties: HashMap::from([("style".to_string(), style.to_string())]),
            },
        );
        LintOptions {
            rules,
            ..LintOptions::default()
        }
    }

    #[test]
    fn fixes_consistent_horizontal_rule_style() {
        let content = "---\n\n  * * *\n";
        let results = lint(content, &LintOptions::default()).expect("lint runs");
        let md035 = results
            .iter()
            .find(|result| result.rule_id == "MD035")
            .expect("MD035 diagnostic exists");

        assert!(md035.fix.is_some());
        let fixed = fix_with_results(content, &results);
        assert_eq!(fixed.content, "---\n\n  ---\n");
    }

    #[test]
    fn fixes_configured_horizontal_rule_style() {
        let content = "---\n";
        let results = lint(content, &md035_options("***")).expect("lint runs");
        let fixed = fix_with_results(content, &results);

        assert_eq!(fixed.content, "***\n");
    }

    #[test]
    fn front_matter_delimiter_does_not_seed_horizontal_rule_style() {
        let content = "---\ntitle: Doc\n---\n\n***\n\n***\n";
        let results = lint(content, &LintOptions::default()).expect("lint runs");

        assert!(results.iter().all(|result| result.rule_id != "MD035"));
    }
}
