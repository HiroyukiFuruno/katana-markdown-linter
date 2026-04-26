use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta, SourceRange,
};
use crate::types::RuleConfig;
use std::path::Path;

/// MD044 / proper-names — Proper names.
pub struct ProperNamesRule;

impl MarkdownRule for ProperNamesRule {
    fn id(&self) -> &'static str {
        "MD044"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD044")
    }

    fn evaluate(&self, _file_path: &Path, _content: &str) -> Vec<MarkdownDiagnostic> {
        Vec::new()
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

    fn evaluate_context(
        &self,
        ctx: &DocumentContext<'_>,
        config: Option<&RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD044");
        let names = configured_names(config);
        if names.is_empty() {
            return Vec::new();
        }
        let include_code_blocks = include_code_blocks(config);

        let mut diagnostics = Vec::new();
        for (i, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(i) && !include_code_blocks {
                continue;
            }
            for (start, end, replacement) in
                proper_name_fixes(ctx, i, line.text, &names, !include_code_blocks)
            {
                diagnostics.push(MarkdownDiagnostic {
                    file: ctx.file_path().to_path_buf(),
                    severity: DiagnosticSeverity::Warning,
                    range: DiagnosticRange {
                        start_line: line.number,
                        start_column: start + 1,
                        end_line: line.number,
                        end_column: end + 1,
                    },
                    message: meta.description.to_string(),
                    rule_id: meta.code.to_string(),
                    official_meta: Some(meta.clone()),
                    fix_info: Some(crate::rules::markdown::types::DiagnosticFix {
                        start_line: line.number,
                        start_column: start + 1,
                        end_line: line.number,
                        end_column: end + 1,
                        replacement,
                    }),
                });
            }
        }
        diagnostics
    }
}

fn configured_names(config: Option<&RuleConfig>) -> Vec<String> {
    let Some(raw) = config.and_then(|config| config.properties.get("names")) else {
        return Vec::new();
    };
    serde_json::from_str::<Vec<String>>(raw).unwrap_or_default()
}

fn include_code_blocks(config: Option<&RuleConfig>) -> bool {
    config
        .and_then(|config| config.properties.get("code_blocks"))
        .and_then(|value| value.parse::<bool>().ok())
        .unwrap_or(true)
}

fn proper_name_fixes(
    ctx: &DocumentContext<'_>,
    line_index: usize,
    line: &str,
    names: &[String],
    skip_inline_code: bool,
) -> Vec<(usize, usize, String)> {
    let mut fixes = Vec::new();
    let line_start = ctx.lines()[line_index].content_range.start;
    for correct in names {
        let incorrect = correct.to_lowercase();
        for (start, matched) in line.match_indices(&incorrect) {
            if matched == correct {
                continue;
            }
            let end = start + matched.len();
            let range = SourceRange {
                start: line_start + start,
                end: line_start + end,
            };
            if is_word_boundary(line, start, end)
                && !(skip_inline_code && line.contains('`') && ctx.is_inside_inline_code(range))
            {
                fixes.push((start, end, correct.clone()));
            }
        }
    }
    fixes
}

fn is_word_boundary(line: &str, start: usize, end: usize) -> bool {
    let bytes = line.as_bytes();
    let before = start
        .checked_sub(1)
        .and_then(|idx| bytes.get(idx))
        .is_some_and(|byte| byte.is_ascii_alphanumeric() || *byte == b'_');
    let after = bytes
        .get(end)
        .is_some_and(|byte| byte.is_ascii_alphanumeric() || *byte == b'_');
    !before && !after
}
