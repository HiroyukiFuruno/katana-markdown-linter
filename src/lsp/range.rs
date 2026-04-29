use serde_json::{json, Value};

pub(crate) struct SelectedLineRange {
    start_line: usize,
    end_line_exclusive: usize,
    pub(crate) start_offset: usize,
    pub(crate) end_offset: usize,
}

impl SelectedLineRange {
    pub(crate) fn from_lsp_range(content: &str, range: &Value) -> Option<Self> {
        let start = range.get("start")?;
        let end = range.get("end")?;
        let start_line = lsp_line(start)?;
        let end_line = lsp_line(end)?;
        let end_character = end.get("character")?.as_u64()? as usize;
        let end_line_exclusive = end_line + usize::from(end_character > 0);
        if end_line_exclusive < start_line {
            return None;
        }
        Some(Self {
            start_line,
            end_line_exclusive,
            start_offset: line_start_offset(content, start_line)?,
            end_offset: line_start_offset(content, end_line_exclusive)?,
        })
    }

    pub(crate) fn lsp_range(&self) -> Value {
        json!({
            "start": { "line": self.start_line, "character": 0 },
            "end": { "line": self.end_line_exclusive, "character": 0 }
        })
    }
}

fn lsp_line(position: &Value) -> Option<usize> {
    position.get("line")?.as_u64().map(|line| line as usize)
}

fn line_start_offset(content: &str, line: usize) -> Option<usize> {
    if line == 0 {
        return Some(0);
    }

    let mut current_line = 0;
    for (index, byte) in content.bytes().enumerate() {
        if byte == b'\n' {
            current_line += 1;
            if current_line == line {
                return Some(index + 1);
            }
        }
    }

    if line == current_line + 1 {
        Some(content.len())
    } else {
        None
    }
}
