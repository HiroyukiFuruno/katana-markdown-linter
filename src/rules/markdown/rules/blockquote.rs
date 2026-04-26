use crate::rules::markdown::helpers::RuleHelpers;
use crate::rules::markdown::{
    DiagnosticSeverity, DocumentContext, MarkdownDiagnostic, MarkdownRule, OfficialRuleMeta,
};
use std::path::Path;

/* WHY: Section: Blockquote-related markdownlint rule implementations
=======================================================
  Split from whitespace.rs to stay within 200-line limit. */

/// MD028 / no-blanks-blockquote — Blank line inside blockquote.
pub struct NoBlanksBlockquoteRule;

const GFM_ALERT_MARKERS: [&str; 5] = [
    "[!NOTE]",
    "[!TIP]",
    "[!IMPORTANT]",
    "[!WARNING]",
    "[!CAUTION]",
];

impl MarkdownRule for NoBlanksBlockquoteRule {
    fn id(&self) -> &'static str {
        "MD028"
    }

    fn official_meta(&self) -> Option<OfficialRuleMeta> {
        crate::rules::markdown::catalog::get_official_meta("MD028")
    }

    fn evaluate(&self, file_path: &Path, content: &str) -> Vec<MarkdownDiagnostic> {
        let meta = self.official_meta().expect("always Some for MD028");
        let mut diagnostics = Vec::new();
        let ctx = DocumentContext::new(file_path, content);
        for (i, line) in ctx.lines().iter().enumerate() {
            let trimmed = line.text.trim();
            if ctx.is_code_line(i) || !trimmed.is_empty() {
                continue;
            }
            /* WHY: A blank between two blockquote lines creates separated blockquotes */
            let has_bq_before = i > 0 && is_blockquote_line(ctx.lines()[i - 1].text);
            let has_bq_after =
                i + 1 < ctx.lines().len() && is_blockquote_line(ctx.lines()[i + 1].text);
            if has_bq_before && has_bq_after && !is_between_gfm_alert_blocks(&ctx, i) {
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

fn is_between_gfm_alert_blocks(ctx: &DocumentContext, blank_line_index: usize) -> bool {
    let Some(previous_start) = previous_blockquote_start(ctx, blank_line_index) else {
        return false;
    };
    let Some(next_start) = next_blockquote_start(ctx, blank_line_index) else {
        return false;
    };

    is_gfm_alert_marker(ctx.lines()[previous_start].text)
        && is_gfm_alert_marker(ctx.lines()[next_start].text)
}

fn previous_blockquote_start(ctx: &DocumentContext, blank_line_index: usize) -> Option<usize> {
    if blank_line_index == 0 {
        return None;
    }

    let mut line_index = blank_line_index - 1;
    loop {
        if line_index == 0 {
            return Some(line_index);
        }

        let previous_index = line_index - 1;
        if !is_blockquote_line(ctx.lines()[previous_index].text) {
            return Some(line_index);
        }
        line_index = previous_index;
    }
}

fn next_blockquote_start(ctx: &DocumentContext, blank_line_index: usize) -> Option<usize> {
    let next_index = blank_line_index + 1;
    ctx.lines()
        .get(next_index)
        .filter(|line| is_blockquote_line(line.text))
        .map(|_| next_index)
}

fn is_blockquote_line(line: &str) -> bool {
    line.trim_start().starts_with('>')
}

fn is_gfm_alert_marker(line: &str) -> bool {
    let Some(content) = line.trim_start().strip_prefix('>') else {
        return false;
    };
    content
        .split_whitespace()
        .next()
        .is_some_and(|marker| GFM_ALERT_MARKERS.contains(&marker))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn md028_allows_adjacent_gfm_alert_blocks() {
        let rule = NoBlanksBlockquoteRule;
        let content = concat!(
            "> [!NOTE]\n",
            "> Highlights information.\n",
            "\n",
            "> [!TIP]\n",
            "> Optional information.\n",
            "\n",
            "> [!IMPORTANT]\n",
            "> Crucial information.\n",
            "\n",
            "> [!WARNING]\n",
            "> Critical content.\n",
            "\n",
            "> [!CAUTION]\n",
            "> Negative consequences.\n",
        );

        let diagnostics = rule.evaluate(Path::new("doc.md"), content);

        assert!(diagnostics.is_empty());
    }

    #[test]
    fn md028_still_reports_regular_blockquote_blank_lines() {
        let rule = NoBlanksBlockquoteRule;
        let diagnostics = rule.evaluate(Path::new("doc.md"), "> quote\n\n> more\n");

        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].rule_id, "MD028");
    }
}
