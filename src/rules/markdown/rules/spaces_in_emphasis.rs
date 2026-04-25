use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD037 / no-space-in-emphasis — Spaces inside emphasis markers
pub struct SpacesInEmphasisRule;

impl MarkdownRule for SpacesInEmphasisRule {
    fn id(&self) -> &'static str {
        "MD037"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD037")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD037");
        let mut diagnostics = Vec::new();

        let ctx = DocumentContext::new(file_path, content);
        for (i, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(i) {
                continue;
            }
            let line = line.text;

            let mut markers = Vec::new();
            let mut chars = line.char_indices().peekable();
            while let Some((idx, c)) = chars.next() {
                if c == '*' || c == '_' {
                    let mut count = 1;
                    while let Some(&(_, next_c)) = chars.peek() {
                        if next_c == c {
                            count += 1;
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    if count <= 2 {
                        markers.push((idx, count, c));
                    }
                }
            }

            for m in 0..markers.len() {
                let (start_idx, len, kind) = markers[m];
                let after_marker_idx = start_idx + len;

                if line[after_marker_idx..].starts_with(' ') {
                    let valid_start = start_idx == 0
                        || line[..start_idx]
                            .ends_with(|c: char| c.is_whitespace() || "([{\"'.!?,;:".contains(c));
                    if !valid_start {
                        continue;
                    }

                    for &(end_start_idx, end_len, end_kind) in markers.iter().skip(m + 1) {
                        if end_kind == kind && end_len == len {
                            if line[..end_start_idx].ends_with(' ') {
                                let inner_text = &line[after_marker_idx..end_start_idx];
                                if !inner_text.contains('`')
                                    && inner_text.chars().any(|c| !c.is_whitespace())
                                {
                                    let trimmed_inner = inner_text.trim();
                                    let marker_str: String =
                                        std::iter::repeat_n(kind, len).collect();
                                    let replacement =
                                        format!("{}{}{}", marker_str, trimmed_inner, marker_str);

                                    let fix = crate::rules::markdown::types::DiagnosticFix {
                                        start_line: i + 1,
                                        start_column: start_idx + 1,
                                        end_line: i + 1,
                                        end_column: end_start_idx + end_len + 1,
                                        replacement,
                                    };

                                    RuleHelpers::push_diag_with_fix(
                                        &mut diagnostics,
                                        file_path,
                                        i,
                                        line,
                                        &meta,
                                        DiagnosticSeverity::Warning,
                                        Some(fix),
                                    );
                                }
                            }
                            break;
                        }
                    }
                }
            }
        }

        diagnostics
    }
}
