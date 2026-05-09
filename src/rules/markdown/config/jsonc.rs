use crate::Error;
use serde_json::Value;
use std::iter::Peekable;
use std::str::Chars;

pub(super) fn parse_config_text(raw: &str) -> Result<Value, Error> {
    let without_comments = strip_jsonc_comments(raw);
    let normalized = strip_trailing_commas(&without_comments);
    serde_json::from_str(&normalized)
        .map_err(|err| Error::new(format!("failed to parse config: {err}")))
}

fn strip_jsonc_comments(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();
    let mut state = StringScanState::default();

    while let Some(ch) = chars.next() {
        if state.push_string_char(ch, &mut output) {
            continue;
        }

        match ch {
            '"' => {
                state.in_string = true;
                output.push(ch);
            }
            '/' if matches!(chars.peek(), Some('/')) => {
                chars.next();
                consume_line_comment(&mut chars, &mut output);
            }
            '/' if matches!(chars.peek(), Some('*')) => {
                chars.next();
                consume_block_comment(&mut chars);
            }
            _ => output.push(ch),
        }
    }

    output
}

fn strip_trailing_commas(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let chars: Vec<char> = input.chars().collect();
    let mut index = 0;
    let mut state = StringScanState::default();

    while index < chars.len() {
        let ch = chars[index];
        if state.push_string_char(ch, &mut output) {
            index += 1;
            continue;
        }

        if ch == '"' {
            state.in_string = true;
            output.push(ch);
            index += 1;
            continue;
        }

        if ch == ',' && comma_is_trailing(&chars, index) {
            index += 1;
            continue;
        }

        output.push(ch);
        index += 1;
    }

    output
}

#[derive(Default)]
struct StringScanState {
    in_string: bool,
    escape: bool,
}

impl StringScanState {
    fn push_string_char(&mut self, ch: char, output: &mut String) -> bool {
        if !self.in_string {
            return false;
        }
        output.push(ch);
        if self.escape {
            self.escape = false;
            return true;
        }
        match ch {
            '\\' => self.escape = true,
            '"' => self.in_string = false,
            _ => {}
        }
        true
    }
}

fn consume_line_comment(chars: &mut Peekable<Chars<'_>>, output: &mut String) {
    for comment_ch in chars.by_ref() {
        if comment_ch == '\n' {
            output.push('\n');
            break;
        }
        if comment_ch == '\r' {
            output.push('\r');
            consume_optional_lf(chars, output);
            break;
        }
    }
}

fn consume_optional_lf(chars: &mut Peekable<Chars<'_>>, output: &mut String) {
    if matches!(chars.peek(), Some('\n')) {
        output.push('\n');
        chars.next();
    }
}

fn consume_block_comment(chars: &mut Peekable<Chars<'_>>) {
    let mut prev = '\0';
    for comment_ch in chars.by_ref() {
        if prev == '*' && comment_ch == '/' {
            break;
        }
        prev = comment_ch;
    }
}

fn comma_is_trailing(chars: &[char], comma_index: usize) -> bool {
    let mut index = comma_index + 1;
    while let Some(next) = chars.get(index) {
        if next.is_whitespace() {
            index += 1;
            continue;
        }
        return *next == '}' || *next == ']';
    }
    false
}
