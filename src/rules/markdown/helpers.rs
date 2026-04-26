use crate::rules::markdown::{
    DiagnosticRange, DiagnosticSeverity, MarkdownDiagnostic, OfficialRuleMeta,
};
use std::path::Path;

/* WHY: Section: Shared helper utilities for markdown rule implementations
=======================================================
  Grouped into struct+impl per coding-rules §1.1 (no public free functions).
  Extracted to stay within 200-line file limits per coding-rules §2.1. */

const MAX_HEADING_LEVEL: usize = 6;

pub struct RuleHelpers;

impl RuleHelpers {
    /// Detect whether a line is a fenced code block delimiter.
    pub fn is_fence(trimmed: &str) -> bool {
        trimmed.starts_with("```") || trimmed.starts_with("~~~")
    }

    /// Detect whether a line is an ATX-style heading.
    pub fn is_atx_heading(trimmed: &str) -> bool {
        if !trimmed.starts_with('#') {
            return false;
        }
        let count = trimmed.chars().take_while(|c| *c == '#').count();
        count <= MAX_HEADING_LEVEL && trimmed[count..].starts_with(' ')
    }

    /// Detect whether a line is a list item (bullet or ordered).
    pub fn is_list_item(trimmed: &str) -> bool {
        Self::get_bullet_char(trimmed).is_some() || Self::get_ordered_number(trimmed).is_some()
    }

    /// Returns the bullet character if the line starts with one.
    pub fn get_bullet_char(trimmed: &str) -> Option<char> {
        let first = trimmed.chars().next()?;
        if (first == '-' || first == '*' || first == '+') && trimmed.get(1..2) == Some(" ") {
            return Some(first);
        }
        None
    }

    /// Returns the ordered list number prefix if present.
    pub fn get_ordered_number(trimmed: &str) -> Option<u32> {
        let dot_pos = trimmed.find(". ")?;
        let prefix = &trimmed[..dot_pos];
        prefix.parse::<u32>().ok()
    }

    /// Push a diagnostic with standard structure.
    pub fn push_diag(
        diagnostics: &mut Vec<MarkdownDiagnostic>,
        file_path: &Path,
        line_idx: usize,
        line: &str,
        meta: &OfficialRuleMeta,
        severity: DiagnosticSeverity,
    ) {
        Self::push_diag_with_fix(diagnostics, file_path, line_idx, line, meta, severity, None);
    }

    /// Push a diagnostic with fix info.
    pub fn push_diag_with_fix(
        diagnostics: &mut Vec<MarkdownDiagnostic>,
        file_path: &Path,
        line_idx: usize,
        line: &str,
        meta: &OfficialRuleMeta,
        severity: DiagnosticSeverity,
        fix_info: Option<crate::rules::markdown::types::DiagnosticFix>,
    ) {
        diagnostics.push(MarkdownDiagnostic {
            file: file_path.to_path_buf(),
            severity,
            range: DiagnosticRange {
                start_line: line_idx + 1,
                start_column: 1,
                end_line: line_idx + 1,
                end_column: line.len().max(1),
            },
            message: meta.description.to_string(),
            rule_id: meta.code.to_string(),
            official_meta: Some(meta.clone()),
            fix_info,
        });
    }

    /// Returns ATX heading level (1-6) for a line, or None if not a heading.
    pub fn get_heading_level(line: &str) -> Option<usize> {
        if !line.starts_with('#') {
            return None;
        }
        let count = line.chars().take_while(|c| *c == '#').count();
        if line[count..].starts_with(' ') {
            Some(count)
        } else {
            None
        }
    }

    /// Push a broken-link diagnostic for an internal-only rule.
    pub fn push_broken_link_violation(
        diagnostics: &mut Vec<MarkdownDiagnostic>,
        file_path: &Path,
        line_idx: usize,
        actual_start: usize,
        absolute_end: usize,
        base_dir: &Path,
        link: &str,
    ) {
        if link.starts_with("http") || link.starts_with('#') {
            return;
        }
        let target_path = base_dir.join(link);
        if target_path.exists() || target_path.with_extension("md").exists() {
            return;
        }
        diagnostics.push(MarkdownDiagnostic {
            file: file_path.to_path_buf(),
            severity: DiagnosticSeverity::Error,
            range: DiagnosticRange {
                start_line: line_idx + 1,
                start_column: actual_start + 1,
                end_line: line_idx + 1,
                end_column: absolute_end + 2,
            },
            message: format!("Broken local link: {}", link),
            rule_id: "md-broken-link".to_string(),
            official_meta: None,
            fix_info: None,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordered_numbers_and_broken_link_diagnostics_are_reported() {
        assert_eq!(RuleHelpers::get_ordered_number("12. item"), Some(12));

        let dir = std::env::temp_dir().join(format!(
            "katana-markdown-linter-helper-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("test directory should be created");
        std::fs::write(dir.join("present.md"), "").expect("target file should be created");

        let mut diagnostics = Vec::new();
        RuleHelpers::push_broken_link_violation(
            &mut diagnostics,
            Path::new("doc.md"),
            1,
            3,
            9,
            &dir,
            "https://example.com",
        );
        RuleHelpers::push_broken_link_violation(
            &mut diagnostics,
            Path::new("doc.md"),
            1,
            3,
            9,
            &dir,
            "#heading",
        );
        RuleHelpers::push_broken_link_violation(
            &mut diagnostics,
            Path::new("doc.md"),
            1,
            3,
            9,
            &dir,
            "present",
        );
        RuleHelpers::push_broken_link_violation(
            &mut diagnostics,
            Path::new("doc.md"),
            1,
            3,
            9,
            &dir,
            "missing",
        );

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_id, "md-broken-link");
        assert_eq!(diagnostics[0].range.start_line, 2);
        assert_eq!(diagnostics[0].range.start_column, 4);
        assert_eq!(diagnostics[0].range.end_column, 11);

        let _ = std::fs::remove_dir_all(dir);
    }
}
