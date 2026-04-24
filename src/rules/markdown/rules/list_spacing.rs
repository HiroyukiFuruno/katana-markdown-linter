use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use crate::types::RuleConfig;
use std::path::Path;

/// MD030 / list-marker-space - Spaces after list markers.
pub struct ListMarkerSpaceRule;

impl MarkdownRule for ListMarkerSpaceRule {
    fn id(&self) -> &'static str {
        "MD030"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD030")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        self.evaluate_configured(file_path, content, None)
    }

    fn evaluate_configured(
        &self,
        file_path: &Path,
        content: &str,
        config: Option<&RuleConfig>,
    ) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD030");
        let mut diagnostics = Vec::new();
        let mut in_code_block = false;
        let lines = content.lines().collect::<Vec<_>>();

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim_start();
            if RuleHelpers::is_fence(trimmed) {
                in_code_block = !in_code_block;
                continue;
            }
            if in_code_block {
                continue;
            }

            if let Some(marker) = list_marker(line) {
                let mut after = line[marker.spaces_start..].chars();
                let spaces = after.by_ref().take_while(|c| c.is_whitespace()).count();
                let target = configured_target_spaces(config, marker.kind, is_multiline(&lines, i));
                if spaces != target {
                    let fix = crate::rules::markdown::types::DiagnosticFix {
                        start_line: i + 1,
                        start_column: marker.spaces_start + 1,
                        end_line: i + 1,
                        end_column: marker.spaces_start + spaces + 1,
                        replacement: " ".repeat(target),
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
        }

        diagnostics
    }
}

#[derive(Clone, Copy)]
enum MarkerKind {
    Ordered,
    Unordered,
}

struct ListMarker {
    kind: MarkerKind,
    spaces_start: usize,
}

fn list_marker(line: &str) -> Option<ListMarker> {
    let trimmed = line.trim_start();
    let leading = line.len() - trimmed.len();
    let mut chars = trimmed.chars();
    let first = chars.next()?;

    if matches!(first, '-' | '*' | '+') && chars.next()?.is_whitespace() {
        return Some(ListMarker {
            kind: MarkerKind::Unordered,
            spaces_start: leading + 1,
        });
    }

    if first.is_ascii_digit() {
        let dot_pos = trimmed.find('.')?;
        let prefix = &trimmed[..dot_pos];
        if prefix.chars().all(|c| c.is_ascii_digit())
            && trimmed[dot_pos + 1..]
                .chars()
                .next()
                .is_some_and(char::is_whitespace)
        {
            return Some(ListMarker {
                kind: MarkerKind::Ordered,
                spaces_start: leading + dot_pos + 1,
            });
        }
    }

    None
}

fn configured_target_spaces(
    config: Option<&RuleConfig>,
    kind: MarkerKind,
    is_multiline: bool,
) -> usize {
    let key = match (kind, is_multiline) {
        (MarkerKind::Ordered, true) => "ol_multi",
        (MarkerKind::Ordered, false) => "ol_single",
        (MarkerKind::Unordered, true) => "ul_multi",
        (MarkerKind::Unordered, false) => "ul_single",
    };
    config
        .and_then(|config| config.properties.get(key))
        .and_then(|value| value.parse::<usize>().ok())
        .unwrap_or(1)
}

fn is_multiline(lines: &[&str], index: usize) -> bool {
    let Some(current) = lines.get(index) else {
        return false;
    };
    let current_indent = current.len() - current.trim_start().len();
    let Some(next) = lines.get(index + 1) else {
        return false;
    };
    let next_trimmed = next.trim_start();
    if next_trimmed.is_empty() || RuleHelpers::is_list_item(next_trimmed) {
        return false;
    }
    next.len() - next_trimmed.len() > current_indent
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_extra_space_after_list_marker() {
        let rule = ListMarkerSpaceRule;
        let content = "-  item\n1.  item";
        let diagnostics = rule.evaluate(Path::new("doc.md"), content);
        assert_eq!(diagnostics.len(), 2);
        assert!(diagnostics
            .iter()
            .all(|diagnostic| diagnostic.fix_info.is_some()));
    }

    #[test]
    fn fixes_to_configured_single_line_spacing() {
        let rule = ListMarkerSpaceRule;
        let config = RuleConfig {
            enabled: true,
            properties: [("ul_single".to_string(), "2".to_string())].into(),
        };
        let diagnostics = rule.evaluate_configured(Path::new("doc.md"), "- item", Some(&config));

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0]
                .fix_info
                .as_ref()
                .expect("configured spacing should be fixable")
                .replacement,
            "  "
        );
    }
}
