use crate::rules::markdown::{
    BlockRange, DiagnosticFix, DiagnosticSeverity, DocumentContext, FenceKind, MarkdownDiagnostic,
    MarkdownRule, OfficialRuleMeta,
};
use crate::types::RuleConfig;
use std::path::Path;

const MIN_FENCE_MARKER_LENGTH: usize = 3;

/// MD048 / code-fence-style — Code fence style.
pub struct CodeFenceStyleRule;

impl MarkdownRule for CodeFenceStyleRule {
    fn id(&self) -> &'static str {
        "MD048"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        let mut meta = crate::rules::markdown::catalog::get_official_meta("MD048")?;
        meta.is_fixable = true;
        Some(meta)
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
        let meta = self.official_meta().expect("always Some for MD048");
        let Some(expected) = expected_fence_kind(ctx, config) else {
            return Vec::new();
        };

        ctx.code_blocks()
            .iter()
            .filter(|block| block.fence != expected)
            .map(|block| {
                let range = ctx.diagnostic_range(block.range);
                MarkdownDiagnostic {
                    file: ctx.file_path().to_path_buf(),
                    severity: DiagnosticSeverity::Warning,
                    range: range.clone(),
                    message: meta.description.to_string(),
                    rule_id: meta.code.to_string(),
                    official_meta: Some(meta.clone()),
                    fix_info: safe_fix(ctx, block, expected).map(|replacement| DiagnosticFix {
                        start_line: range.start_line,
                        start_column: range.start_column,
                        end_line: range.end_line,
                        end_column: range.end_column,
                        replacement,
                    }),
                }
            })
            .collect()
    }
}

fn expected_fence_kind(
    ctx: &DocumentContext<'_>,
    config: Option<&RuleConfig>,
) -> Option<FenceKind> {
    let configured = config.and_then(|config| config.properties.get("style"));
    match configured.map(String::as_str) {
        Some("backtick") => Some(FenceKind::Backtick),
        Some("tilde") => Some(FenceKind::Tilde),
        Some("consistent") | None => ctx.code_blocks().first().map(|block| block.fence),
        Some(_) => ctx.code_blocks().first().map(|block| block.fence),
    }
}

fn safe_fix(ctx: &DocumentContext<'_>, block: &BlockRange, expected: FenceKind) -> Option<String> {
    if block.start_line >= block.end_line {
        return None;
    }
    let lines = ctx.lines();
    let opening = lines.get(block.start_line)?;
    let closing = lines.get(block.end_line)?;
    if line_fence_kind(opening.text) != Some(block.fence)
        || line_fence_kind(closing.text) != Some(block.fence)
    {
        return None;
    }
    if has_inner_collision(ctx, block, expected) {
        return None;
    }

    let mut replacement = (block.start_line..=block.end_line)
        .map(|line_index| {
            let line = lines[line_index].text;
            if line_index == block.start_line || line_index == block.end_line {
                replace_fence_marker(line, expected)
            } else {
                Some(line.to_string())
            }
        })
        .collect::<Option<Vec<_>>>()?
        .join("\n");
    if ctx.content()[block.range.start..block.range.end].ends_with('\n') {
        replacement.push('\n');
    }
    Some(replacement)
}

fn has_inner_collision(ctx: &DocumentContext<'_>, block: &BlockRange, expected: FenceKind) -> bool {
    let target = match expected {
        FenceKind::Backtick => "```",
        FenceKind::Tilde => "~~~",
    };
    ctx.lines()[block.start_line + 1..block.end_line]
        .iter()
        .any(|line| line.text.trim_start().starts_with(target))
}

fn line_fence_kind(line: &str) -> Option<FenceKind> {
    let trimmed = line.trim_start();
    if trimmed.starts_with("```") {
        Some(FenceKind::Backtick)
    } else if trimmed.starts_with("~~~") {
        Some(FenceKind::Tilde)
    } else {
        None
    }
}

fn replace_fence_marker(line: &str, expected: FenceKind) -> Option<String> {
    let prefix_len = line.len() - line.trim_start().len();
    let prefix = &line[..prefix_len];
    let trimmed = &line[prefix_len..];
    let source = match line_fence_kind(line)? {
        FenceKind::Backtick => b'`',
        FenceKind::Tilde => b'~',
    };
    let marker_len = trimmed.bytes().take_while(|byte| *byte == source).count();
    if marker_len < MIN_FENCE_MARKER_LENGTH {
        return None;
    }
    let target = match expected {
        FenceKind::Backtick => '`',
        FenceKind::Tilde => '~',
    };
    Some(format!(
        "{}{}{}",
        prefix,
        std::iter::repeat_n(target, marker_len).collect::<String>(),
        &trimmed[marker_len..]
    ))
}

#[cfg(test)]
mod tests {
    use crate::{LintOptions, MarkdownLinter, RuleConfig};
    use std::collections::HashMap;

    fn md048_options(style: &str) -> LintOptions {
        let mut rules = HashMap::new();
        rules.insert(
            "MD048".to_string(),
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
    fn fixes_mixed_fence_style_to_first_style_when_consistent() {
        let content = "```rust\ncode\n```\n\n~~~text\ncode\n~~~\n";
        let results = MarkdownLinter::lint(content, &LintOptions::default()).expect("lint runs");
        let md048 = results
            .iter()
            .find(|result| result.rule_id == "MD048")
            .expect("MD048 diagnostic exists");

        assert_eq!(md048.line, 5);
        assert!(md048.fix.is_some());
        let fixed = MarkdownLinter::fix_with_results(content, &results);
        assert_eq!(fixed.content, "```rust\ncode\n```\n\n```text\ncode\n```\n");
    }

    #[test]
    fn fixes_configured_tilde_style() {
        let content = "```rust\ncode\n```\n";
        let results = MarkdownLinter::lint(content, &md048_options("tilde")).expect("lint runs");

        assert!(results
            .iter()
            .find(|result| result.rule_id == "MD048")
            .expect("MD048 diagnostic exists")
            .fix
            .is_some());
        let fixed = MarkdownLinter::fix_with_results(content, &results);
        assert_eq!(fixed.content, "~~~rust\ncode\n~~~\n");
    }

    #[test]
    fn fixes_configured_backtick_style() {
        let content = "~~~rust\ncode\n~~~\n";
        let results = MarkdownLinter::lint(content, &md048_options("backtick")).expect("lint runs");

        let fixed = MarkdownLinter::fix_with_results(content, &results);
        assert_eq!(fixed.content, "```rust\ncode\n```\n");
    }

    #[test]
    fn keeps_diagnostic_but_skips_fix_when_target_marker_collides_inside_block() {
        let content = "```rust\ncode\n```\n\n~~~~\n```text\nnested\n```\n~~~~\n";
        let results = MarkdownLinter::lint(content, &LintOptions::default()).expect("lint runs");
        let md048 = results
            .iter()
            .find(|result| result.rule_id == "MD048")
            .expect("MD048 diagnostic exists");

        assert!(md048.fix.is_none());
    }

    #[test]
    fn keeps_diagnostic_but_skips_fix_for_unclosed_fence() {
        let content = "```rust\ncode\n```\n\n~~~text\ncode\n";
        let results = MarkdownLinter::lint(content, &LintOptions::default()).expect("lint runs");
        let md048 = results
            .iter()
            .find(|result| result.rule_id == "MD048")
            .expect("MD048 diagnostic exists");

        assert!(md048.fix.is_none());
    }
}
