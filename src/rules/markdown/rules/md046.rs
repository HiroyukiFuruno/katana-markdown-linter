use crate::rules::markdown::{
    DiagnosticFix, DiagnosticRange, DiagnosticSeverity, DocumentContext, MarkdownDiagnostic,
    MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD046 / code-block-style — Code block style.
pub struct CodeBlockStyleRule;

impl MarkdownRule for CodeBlockStyleRule {
    fn id(&self) -> &'static str {
        "MD046"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD046")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD046");
        let ctx = DocumentContext::new(file_path, content);

        if ctx.code_blocks().is_empty() {
            return Vec::new();
        }

        indented_code_block_groups(&ctx)
            .into_iter()
            .map(|(start_line, end_line, last_line_len, block_lines)| {
                let replacement = build_fenced_replacement(&block_lines);
                MarkdownDiagnostic {
                    file: file_path.to_path_buf(),
                    severity: DiagnosticSeverity::Warning,
                    range: DiagnosticRange {
                        start_line,
                        start_column: 1,
                        end_line,
                        end_column: last_line_len + 1,
                    },
                    message: meta.description.to_string(),
                    rule_id: meta.code.to_string(),
                    official_meta: Some(meta.clone()),
                    fix_info: Some(DiagnosticFix {
                        start_line,
                        start_column: 1,
                        end_line,
                        end_column: last_line_len + 1,
                        replacement,
                    }),
                }
            })
            .collect()
    }
}

/// Returns groups of consecutive indented code block lines as
/// `(start_line_1based, end_line_1based, last_line_byte_len, line_texts)`.
/// Blank lines split blocks; list continuations are excluded.
fn indented_code_block_groups<'a>(
    ctx: &'a DocumentContext<'_>,
) -> Vec<(usize, usize, usize, Vec<&'a str>)> {
    let mut groups: Vec<(usize, usize, usize, Vec<&str>)> = Vec::new();
    let mut current: Option<(usize, usize, usize, Vec<&str>)> = None;

    for (idx, line) in ctx.lines().iter().enumerate() {
        if is_indented_code_line(ctx, idx, line.text) {
            if let Some(ref mut grp) = current {
                grp.1 = line.number;
                grp.2 = line.text.len();
                grp.3.push(line.text);
            } else {
                current = Some((line.number, line.number, line.text.len(), vec![line.text]));
            }
        } else if let Some(grp) = current.take() {
            groups.push(grp);
        }
    }
    if let Some(grp) = current.take() {
        groups.push(grp);
    }
    groups
}

fn build_fenced_replacement(lines: &[&str]) -> String {
    let content_len: usize = lines.iter().map(|l| l.len().saturating_sub(4) + 1).sum();
    let mut result = String::with_capacity(4 + content_len + 3);
    result.push_str("```");
    for line in lines {
        result.push('\n');
        result.push_str(line.strip_prefix("    ").unwrap_or(line));
    }
    result.push('\n');
    result.push_str("```");
    result
}

fn is_indented_code_line(ctx: &DocumentContext<'_>, line_index: usize, line: &str) -> bool {
    if ctx.is_code_line(line_index) || !line.starts_with("    ") || line.trim().is_empty() {
        return false;
    }
    !is_list_marker_line(&line[4..])
}

fn is_list_marker_line(s: &str) -> bool {
    let s = s.trim_start();
    if s.starts_with("- ")
        || s.starts_with("* ")
        || s.starts_with("+ ")
        || s == "-"
        || s == "*"
        || s == "+"
    {
        return true;
    }
    let num_end = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(0);
    num_end > 0 && (s[num_end..].starts_with(". ") || s[num_end..].starts_with(") "))
}

#[cfg(test)]
mod tests {
    use crate::{fix, lint, LintOptions};

    #[test]
    fn ignores_indented_lines_inside_fenced_diagrams() {
        let content = "# Title\n\n```mermaid\ngraph TD\n    A --> B\n```\n";
        let results = lint(content, &LintOptions::default()).expect("lint runs");

        assert!(results.iter().all(|result| result.rule_id != "MD046"));
    }

    #[test]
    fn ignores_indented_list_items_as_code_block() {
        let content = "```rust\nlet x = 1;\n```\n\n- item\n    - nested\n    - nested2\n";
        let results = lint(content, &LintOptions::default()).expect("lint runs");

        assert!(results.iter().all(|result| result.rule_id != "MD046"));
    }

    #[test]
    fn emits_per_block_diagnostic_with_fix() {
        let content = "```rust\nlet x = 1;\n```\n\n    indented code\n";
        let results = lint(content, &LintOptions::default()).expect("lint runs");
        let md046: Vec<_> = results.iter().filter(|r| r.rule_id == "MD046").collect();

        assert_eq!(md046.len(), 1);
        assert!(md046[0].fix.is_some(), "should have a fix");
    }

    #[test]
    fn fix_converts_indented_block_to_fenced() {
        let content = "```rust\nlet x = 1;\n```\n\n    hello\n    world\n";
        let result = fix(content, &LintOptions::default()).expect("fix runs");

        assert!(
            result.content.contains("hello\nworld\n```"),
            "expected stripped lines inside fenced block, got:\n{}",
            result.content
        );
        assert!(
            !result.content.contains("    hello"),
            "expected indented block to be removed, got:\n{}",
            result.content
        );
        assert!(result.applied_fixes >= 1);
    }

    #[test]
    fn two_separate_indented_blocks_produce_two_diagnostics() {
        let content = "```rust\nfn x() {}\n```\n\n    block one\n\n    block two\n";
        let results = lint(content, &LintOptions::default()).expect("lint runs");
        let md046: Vec<_> = results.iter().filter(|r| r.rule_id == "MD046").collect();

        assert_eq!(md046.len(), 2);
        assert!(md046[0].fix.is_some());
        assert!(md046[1].fix.is_some());
    }

    #[test]
    fn pure_indented_only_document_emits_no_diagnostic() {
        let content = "    only indented\n    no fenced blocks\n";
        let results = lint(content, &LintOptions::default()).expect("lint runs");

        assert!(results.iter().all(|r| r.rule_id != "MD046"));
    }
}
