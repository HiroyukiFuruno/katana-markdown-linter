use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, OfficialRuleMeta,
};

const MAX_ATX_HEADING_LEVEL: usize = 6;

pub(super) fn check_closed_atx_spacing(
    ctx: &DocumentContext<'_>,
    meta: &OfficialRuleMeta,
    has_violation: impl Fn(usize, usize) -> bool,
) -> Vec<MarkdownDiagnostic> {
    let mut diagnostics = Vec::new();
    for (index, line) in ctx.lines().iter().enumerate() {
        if ctx.is_code_line(index) {
            continue;
        }
        let Some(heading) = ClosedAtxHeading::parse(line.text) else {
            continue;
        };
        if has_violation(heading.leading_spaces, heading.trailing_spaces) {
            let fix = whole_line_fix(index, line.text, heading.normalized_line());
            RuleHelpers::push_diag_with_fix(
                &mut diagnostics,
                ctx.file_path(),
                index,
                line.text,
                meta,
                DiagnosticSeverity::Warning,
                Some(fix),
            );
        }
    }
    diagnostics
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
        let (indent_end, opening_count, closing_start, closing_end) = heading_markers(line)?;
        let body_region_start = indent_end + opening_count;
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
            closing_end,
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

fn heading_markers(line: &str) -> Option<(usize, usize, usize, usize)> {
    let trimmed_start = line.trim_start_matches(' ');
    let indent_end = line.len() - trimmed_start.len();
    let opening_count = trimmed_start
        .bytes()
        .take_while(|byte| *byte == b'#')
        .count();
    if !(1..=MAX_ATX_HEADING_LEVEL).contains(&opening_count) {
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
    let closing_start = trimmed_end.len() - closing_count;
    (closing_start > indent_end + opening_count).then_some((
        indent_end,
        opening_count,
        closing_start,
        trimmed_end.len(),
    ))
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
