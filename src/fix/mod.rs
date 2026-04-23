use crate::types::{FixResult, LintResult};

pub fn apply(results: &[LintResult], content: &str) -> FixResult {
    let mut applied_fixes = 0;
    let had_trailing_newline = content.ends_with('\n');
    let mut lines: Vec<String> = content.lines().map(|line| line.to_string()).collect();
    for result in results {
        let Some(fix) = &result.fix else {
            continue;
        };
        if fix.range.start_line != fix.range.end_line {
            continue;
        }
        if let Some(line) = lines.get_mut(fix.range.start_line.saturating_sub(1)) {
            let start = fix.range.start_column.saturating_sub(1);
            let end = fix.range.end_column.saturating_sub(1).min(line.len());
            if start <= end && end <= line.len() {
                line.replace_range(start..end, &fix.replacement);
                applied_fixes += 1;
            }
        }
    }

    let mut content = lines.join("\n");
    if had_trailing_newline && !content.is_empty() {
        content.push('\n');
    }
    FixResult {
        content,
        applied_fixes,
    }
}
