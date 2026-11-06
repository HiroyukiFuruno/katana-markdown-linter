pub(super) fn bare_url_ranges(line: &str) -> BareUrlRanges<'_> {
    BareUrlRanges {
        line,
        search_start: 0,
    }
}

pub(super) struct BareUrlRanges<'a> {
    line: &'a str,
    search_start: usize,
}

impl Iterator for BareUrlRanges<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(start) = next_url_start(self.line, self.search_start) {
            self.search_start = start + 1;
            if let Some((_, end)) = url_range(self.line, start) {
                self.search_start = end;
                return Some((start, end));
            }
        }

        None
    }
}

fn next_url_start(line: &str, search_start: usize) -> Option<usize> {
    const SCHEMES: &[&str] = &["http://", "https://", "ftp://", "ftps://", "mailto:"];
    let remainder = &line[search_start..];
    SCHEMES
        .iter()
        .filter_map(|scheme| remainder.find(scheme))
        .min()
        .map(|offset| search_start + offset)
}

fn url_range(line: &str, start: usize) -> Option<(usize, usize)> {
    let mut end = line.len();
    for (offset, ch) in line[start..].char_indices() {
        if ch.is_whitespace() || ch == '<' || ch == '>' {
            end = start + offset;
            break;
        }
    }
    loop {
        if end <= start {
            break;
        }
        match line.as_bytes()[end - 1] {
            b'.' | b',' | b';' | b':' => end -= 1,
            b')' if has_unmatched_closing_delimiter(&line[start..end], b'(', b')') => end -= 1,
            b']' if has_unmatched_closing_delimiter(&line[start..end], b'[', b']') => end -= 1,
            _ => break,
        }
    }
    (end > start).then_some((start, end))
}

fn has_unmatched_closing_delimiter(text: &str, open: u8, close: u8) -> bool {
    let open_count = text.as_bytes().iter().filter(|byte| **byte == open).count();
    let close_count = text
        .as_bytes()
        .iter()
        .filter(|byte| **byte == close)
        .count();
    close_count > open_count
}
