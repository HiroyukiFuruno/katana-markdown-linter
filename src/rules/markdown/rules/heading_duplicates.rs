use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
    RuleConfig,
};
use std::collections::HashSet;
use std::path::Path;

/// MD024 / no-duplicate-heading - Multiple headings with the same content.
pub struct NoDuplicateHeadingRule;

impl MarkdownRule for NoDuplicateHeadingRule {
    fn id(&self) -> &'static str {
        "MD024"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD024")
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
        let meta = self.official_meta().expect("always Some for MD024");
        let mut diagnostics = Vec::new();
        let mut seen = HashSet::new();
        let ctx_lines = ctx.lines();

        for (i, line_info) in ctx_lines.iter().enumerate() {
            if ctx.is_code_line(i) {
                continue;
            }

            if let Some(heading_text) = extract_heading_text(ctx, i) {
                if !seen.insert(heading_text.clone()) {
                    RuleHelpers::push_diag(
                        &mut diagnostics,
                        ctx.file_path(),
                        i,
                        line_info.text,
                        &meta,
                        DiagnosticSeverity::Warning,
                    );
                }
            }
        }

        diagnostics
    }
}

fn extract_heading_text(ctx: &DocumentContext<'_>, idx: usize) -> Option<String> {
    let lines = ctx.lines();
    let line = lines[idx].text;
    let heading = ctx
        .headings()
        .binary_search_by_key(&idx, |heading| heading.line)
        .ok()
        .map(|index| &ctx.headings()[index]);
    if let Some(heading) = heading {
        return Some(heading.text.trim().to_owned());
    }

    if idx + 1 < lines.len() && is_setext_underline(lines[idx + 1].text.trim()) {
        let heading = line.trim();
        if !heading.is_empty() {
            return Some(heading.to_string());
        }
    }

    None
}

fn is_setext_underline(trimmed: &str) -> bool {
    if trimmed.len() < 2 {
        return false;
    }
    trimmed.chars().all(|c| c == '=') || trimmed.chars().all(|c| c == '-')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_duplicate_headings() {
        let rule = NoDuplicateHeadingRule;
        let content = "# title\n\n## section\n\n# title";
        let diagnostics = rule.evaluate(Path::new("doc.md"), content);
        assert_eq!(diagnostics.len(), 1);
    }
}
