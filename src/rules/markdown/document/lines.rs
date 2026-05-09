use super::types::{BlockRange, LineInfo, SourceRange};

pub(super) fn build_code_line_flags(line_count: usize, code_blocks: &[BlockRange]) -> Vec<bool> {
    let mut flags = vec![false; line_count];
    for block in code_blocks {
        if block.start_line >= line_count {
            continue;
        }
        let end_line = block.end_line.min(line_count - 1);
        for flag in &mut flags[block.start_line..=end_line] {
            *flag = true;
        }
    }
    flags
}

pub(super) fn split_lines(content: &str) -> Vec<LineInfo<'_>> {
    let mut lines = Vec::new();
    let mut start = 0;
    for (index, ch) in content.char_indices() {
        if ch != '\n' {
            continue;
        }
        let text_end = line_text_end(content, start, index);
        lines.push(LineInfo {
            number: lines.len() + 1,
            text: &content[start..text_end],
            content_range: SourceRange {
                start,
                end: text_end,
            },
            full_range: SourceRange {
                start,
                end: index + 1,
            },
        });
        start = index + 1;
    }
    if start < content.len() {
        lines.push(LineInfo {
            number: lines.len() + 1,
            text: &content[start..],
            content_range: SourceRange {
                start,
                end: content.len(),
            },
            full_range: SourceRange {
                start,
                end: content.len(),
            },
        });
    }
    lines
}

pub(super) fn extract_front_matter(lines: &[LineInfo<'_>]) -> Option<SourceRange> {
    if lines.first()?.text.trim() != "---" {
        return None;
    }
    lines
        .iter()
        .enumerate()
        .skip(1)
        .find(|(_, line)| line.text.trim() == "---")
        .map(|(_, line)| SourceRange {
            start: 0,
            end: line.full_range.end,
        })
}

fn line_text_end(content: &str, start: usize, newline_index: usize) -> usize {
    if newline_index > start && content.as_bytes()[newline_index - 1] == b'\r' {
        return newline_index - 1;
    }
    newline_index
}
