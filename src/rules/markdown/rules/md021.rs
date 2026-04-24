use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/// MD021 / no-multiple-space-closed-atx — Multiple spaces inside hashes on closed atx heading.
pub struct NoMultipleSpaceClosedAtxRule;

impl MarkdownRule for NoMultipleSpaceClosedAtxRule {
    fn id(&self) -> &'static str {
        "MD021"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD021")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD021");
        let mut diagnostics = Vec::new();
        let mut in_code_block = false;
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim_start();
            if RuleHelpers::is_fence(trimmed) {
                in_code_block = !in_code_block;
                continue;
            }
            if in_code_block {
                continue;
            }
            let Some(heading) = ClosedAtxHeading::parse(line) else {
                continue;
            };
            if heading.leading_spaces > 1 || heading.trailing_spaces > 1 {
                let fix = whole_line_fix(i, line, heading.normalized_line());
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
        diagnostics
    }
}

struct ClosedAtxHeading<'a> {
    line: &'a str,
    indent_end: usize,
    opening_count: usize,
    body_start: usize,
    body_end: usize,
    closing_start: usize,
    closing_end: usize,
    leading_spaces: usize,
    trailing_spaces: usize,
}

impl<'a> ClosedAtxHeading<'a> {
    fn parse(line: &'a str) -> Option<Self> {
        let trimmed_start = line.trim_start_matches(' ');
        let indent_end = line.len() - trimmed_start.len();
        let opening_count = trimmed_start
            .bytes()
            .take_while(|byte| *byte == b'#')
            .count();
        if !(1..=6).contains(&opening_count) {
            return None;
        }

        let trimmed_end = line.trim_end_matches(' ');
        let closing_count = trimmed_end
            .bytes()
            .rev()
            .take_while(|byte| *byte == b'#')
            .count();
        if closing_count == 0 || trimmed_end.len() <= indent_end + opening_count {
            return None;
        }

        let body_region_start = indent_end + opening_count;
        let closing_start = trimmed_end.len() - closing_count;
        if closing_start <= body_region_start {
            return None;
        }

        let body_region = &line[body_region_start..closing_start];
        let leading_spaces = body_region.bytes().take_while(|byte| *byte == b' ').count();
        let trailing_spaces = body_region
            .bytes()
            .rev()
            .take_while(|byte| *byte == b' ')
            .count();
        if leading_spaces + trailing_spaces >= body_region.len() {
            return None;
        }

        Some(Self {
            line,
            indent_end,
            opening_count,
            body_start: body_region_start + leading_spaces,
            body_end: closing_start - trailing_spaces,
            closing_start,
            closing_end: trimmed_end.len(),
            leading_spaces,
            trailing_spaces,
        })
    }

    fn normalized_line(&self) -> String {
        format!(
            "{}{} {} {}{}",
            &self.line[..self.indent_end],
            &self.line[self.indent_end..self.indent_end + self.opening_count],
            &self.line[self.body_start..self.body_end],
            &self.line[self.closing_start..self.closing_end],
            &self.line[self.closing_end..]
        )
    }
}

fn whole_line_fix(
    line_idx: usize,
    line: &str,
    replacement: String,
) -> crate::rules::markdown::types::DiagnosticFix {
    crate::rules::markdown::types::DiagnosticFix {
        start_line: line_idx + 1,
        start_column: 1,
        end_line: line_idx + 1,
        end_column: line.len().max(1) + 1,
        replacement,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixes_multiple_spaces_inside_closed_atx_hashes() {
        let rule = NoMultipleSpaceClosedAtxRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "#  Heading  #\n##  Heading ##");

        assert_eq!(diagnostics.len(), 2);
        assert!(diagnostics
            .iter()
            .all(|diagnostic| diagnostic.fix_info.is_some()));
        assert_eq!(
            diagnostics[0]
                .fix_info
                .as_ref()
                .expect("fix exists")
                .replacement,
            "# Heading #"
        );
        assert_eq!(
            diagnostics[1]
                .fix_info
                .as_ref()
                .expect("fix exists")
                .replacement,
            "## Heading ##"
        );
    }

    #[test]
    fn accepts_closed_atx_with_single_spaces() {
        let rule = NoMultipleSpaceClosedAtxRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "# Heading #");

        assert!(diagnostics.is_empty());
    }
}
