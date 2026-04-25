use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
    RuleConfig,
};
use std::path::Path;

/// MD052 / reference-links-images — Reference links and images.
pub struct ReferenceLinksImagesRule;

impl MarkdownRule for ReferenceLinksImagesRule {
    fn id(&self) -> &'static str {
        "MD052"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD052")
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
        let meta = self.official_meta().expect("always Some for MD052");
        let mut diagnostics = Vec::new();
        for (i, line) in ctx.lines().iter().enumerate() {
            if ctx.is_code_line(i) {
                continue;
            }
            if contains_reference_without_label(line.text) {
                RuleHelpers::push_diag(
                    &mut diagnostics,
                    ctx.file_path(),
                    i,
                    line.text,
                    &meta,
                    DiagnosticSeverity::Warning,
                );
            }
        }
        diagnostics
    }
}

fn contains_reference_without_label(line: &str) -> bool {
    let bytes = line.as_bytes();
    let mut cursor = 0;
    let mut in_code = false;
    while cursor < bytes.len() {
        match bytes[cursor] {
            b'`' => {
                in_code = !in_code;
                cursor += 1;
            }
            b']' if !in_code && bytes.get(cursor + 1) == Some(&b'[') => {
                if bytes.get(cursor + 2) == Some(&b']') {
                    return true;
                }
                cursor += 1;
            }
            b'!' if !in_code && bytes.get(cursor + 1) == Some(&b'[') => {
                let Some(close) = line[cursor + 2..]
                    .find(']')
                    .map(|offset| cursor + 2 + offset)
                else {
                    break;
                };
                if bytes.get(close + 1) == Some(&b'[') && bytes.get(close + 2) == Some(&b']') {
                    return true;
                }
                cursor = close + 1;
            }
            _ => cursor += 1,
        }
    }
    false
}
