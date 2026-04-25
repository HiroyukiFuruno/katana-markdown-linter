use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD038 / no-space-in-code — Spaces inside code span elements
pub struct NoSpaceInCodeRule;

impl MarkdownRule for NoSpaceInCodeRule {
    fn id(&self) -> &'static str {
        "MD038"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD038")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD038");
        let mut diagnostics = Vec::new();

        let ctx = DocumentContext::new(file_path, content);
        for (i, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(i) {
                continue;
            }
            let line = line.text;

            let mut chars = line.char_indices().peekable();
            let mut current_span_start: Option<usize> = None;
            let mut backtick_count = 0;

            while let Some((idx, c)) = chars.next() {
                if c == '`' {
                    let mut count = 1;
                    while let Some(&(_, next_c)) = chars.peek() {
                        if next_c == '`' {
                            count += 1;
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    if let Some(start_idx) = current_span_start {
                        if count == backtick_count {
                            let inner_start = start_idx + count;
                            let inner_end = idx;
                            if inner_start < inner_end {
                                let inner_text = &line[inner_start..inner_end];
                                if inner_text.starts_with(' ') || inner_text.ends_with(' ') {
                                    let trimmed_inner = inner_text.trim();
                                    let replacement = format!(
                                        "{}{}{}",
                                        "`".repeat(count),
                                        trimmed_inner,
                                        "`".repeat(count)
                                    );
                                    let fix = Some(crate::rules::markdown::types::DiagnosticFix {
                                        start_line: i + 1,
                                        start_column: start_idx + 1,
                                        end_line: i + 1,
                                        end_column: inner_end + count + 1,
                                        replacement,
                                    });

                                    RuleHelpers::push_diag_with_fix(
                                        &mut diagnostics,
                                        file_path,
                                        i,
                                        line,
                                        &meta,
                                        DiagnosticSeverity::Warning,
                                        fix,
                                    );
                                }
                            }
                            current_span_start = None;
                            backtick_count = 0;
                        }
                    } else {
                        current_span_start = Some(idx);
                        backtick_count = count;
                    }
                }
            }
        }

        diagnostics
    }
}
