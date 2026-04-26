use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule,
    OfficialRuleMeta, RuleConfig,
};
use std::path::Path;

/// Internal broken-link rule. Hidden from user-facing UI.
pub struct BrokenLinkRule;

impl MarkdownRule for BrokenLinkRule {
    fn id(&self) -> &'static str {
        "md-broken-link"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        None
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
        let Some(base_dir) = local_base_dir(ctx.file_path()) else {
            return Vec::new();
        };
        ctx.inline_links()
            .iter()
            .filter_map(|link| {
                let destination = link.destination?;
                let target = local_target(destination)?;
                let target_path = base_dir.join(target);
                if target_path.exists() || target_path.with_extension("md").exists() {
                    return None;
                }
                let range = ctx.diagnostic_range(
                    link.destination_range
                        .expect("inline destination should have a source range"),
                );
                Some(MarkdownDiagnostic {
                    file: ctx.file_path().to_path_buf(),
                    severity: DiagnosticSeverity::Warning,
                    range: DiagnosticRange {
                        start_line: range.start_line,
                        start_column: range.start_column,
                        end_line: range.end_line,
                        end_column: range.end_column,
                    },
                    message: format!("Broken local link: {destination}"),
                    rule_id: "md-broken-link".to_string(),
                    official_meta: None,
                    fix_info: None,
                })
            })
            .collect()
    }
}

fn local_base_dir(file_path: &Path) -> Option<&Path> {
    if file_path == Path::new("<memory>") || file_path == Path::new("<stdin>") {
        return None;
    }
    file_path.parent()
}

fn local_target(destination: &str) -> Option<&str> {
    if destination.is_empty() || destination.starts_with('#') || destination.starts_with('/') {
        return None;
    }
    if has_uri_scheme(destination) {
        return None;
    }
    let path_without_fragment = destination.split('#').next().unwrap_or(destination);
    let path_without_query = path_without_fragment
        .split('?')
        .next()
        .unwrap_or(path_without_fragment);
    (!path_without_query.is_empty()).then_some(path_without_query)
}

fn has_uri_scheme(destination: &str) -> bool {
    let Some(colon) = destination.find(':') else {
        return false;
    };
    let slash = destination.find('/').unwrap_or(destination.len());
    let hash = destination.find('#').unwrap_or(destination.len());
    let query = destination.find('?').unwrap_or(destination.len());
    colon < slash && colon < hash && colon < query
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignores_non_local_destinations_and_existing_local_files() {
        let root = std::env::temp_dir().join(format!(
            "katana-markdown-linter-broken-link-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("test directory should be created");
        std::fs::write(root.join("present.md"), "").expect("target file should be created");
        let source = root.join("doc.md");
        let content = concat!(
            "[Present](present.md)\n",
            "[Fragment](#fragment)\n",
            "[Email](mailto:test@example.com)\n",
            "[Missing](missing.md)\n",
        );

        let diagnostics = BrokenLinkRule.evaluate(&source, content);

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].message, "Broken local link: missing.md");
        let _ = std::fs::remove_dir_all(root);
    }
}
