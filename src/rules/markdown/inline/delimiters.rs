use regex::Regex;
use std::sync::OnceLock;

static UNICODE_CHARACTER_CLASSES: OnceLock<UnicodeCharacterClasses> = OnceLock::new();

struct Flanking {
    left: bool,
    right: bool,
    before_punctuation: bool,
    after_punctuation: bool,
}

struct UnicodeCharacterClasses {
    whitespace: Regex,
    punctuation: Regex,
}

pub(super) fn can_open_with_neighbors(
    before: Option<&str>,
    after: Option<&str>,
    marker: char,
) -> bool {
    let flanking = flanking(before, after);
    flanking.left && (marker == '*' || !flanking.right || flanking.before_punctuation)
}

pub(super) fn can_close_with_neighbors(
    before: Option<&str>,
    after: Option<&str>,
    marker: char,
) -> bool {
    let flanking = flanking(before, after);
    flanking.right && (marker == '*' || !flanking.left || flanking.after_punctuation)
}

pub(super) fn target_delimiters_can_wrap(
    line: &str,
    start: usize,
    end: usize,
    marker_len: usize,
    marker: char,
) -> bool {
    let inner_start = start + marker_len;
    let inner_end = end.saturating_sub(marker_len);
    if inner_start >= inner_end {
        return false;
    }
    can_open_with_neighbors(
        character_before(line, start),
        character_after(line, inner_start),
        marker,
    ) && can_close_with_neighbors(
        character_before(line, inner_end),
        character_after(line, end),
        marker,
    )
}

pub(in crate::rules::markdown) fn isolated_outer_delimiter_scope(
    source: &str,
    range: crate::rules::markdown::document::SourceRange,
    original_scope: &str,
    changed_scope: &str,
) -> bool {
    isolated_scope_boundary(source, range.start, range.end, original_scope)
        && isolated_scope_boundary(source, range.start, range.end, changed_scope)
}

fn isolated_scope_boundary(source: &str, start: usize, end: usize, scope: &str) -> bool {
    let bytes = scope.as_bytes();
    let Some(&marker) = bytes.first() else {
        return false;
    };
    if !matches!(marker, b'*' | b'_') {
        return false;
    }
    let opening_len = bytes.iter().take_while(|byte| **byte == marker).count();
    let closing_start = bytes
        .iter()
        .rposition(|byte| *byte != marker)
        .map_or(0, |index| index + 1);
    let closing_len = bytes.len().saturating_sub(closing_start);
    if opening_len == 0 || closing_len == 0 || closing_start <= opening_len {
        return false;
    }
    let Some(before) = source.get(..start) else {
        return false;
    };
    let Some(after) = source.get(end..) else {
        return false;
    };
    if before
        .as_bytes()
        .last()
        .is_some_and(|byte| matches!(byte, b'*' | b'_'))
        || after
            .as_bytes()
            .first()
            .is_some_and(|byte| matches!(byte, b'*' | b'_'))
    {
        return false;
    }
    let opening_before = before
        .char_indices()
        .next_back()
        .map(|(index, _)| &before[index..]);
    let opening_after = scope
        .get(opening_len..)
        .and_then(|value| value.chars().next())
        .map(|ch| {
            scope
                .get(opening_len..opening_len + ch.len_utf8())
                .unwrap_or("")
        });
    let closing_before = scope
        .get(..closing_start)
        .and_then(|value| value.chars().next_back())
        .map(|ch| &scope[closing_start - ch.len_utf8()..closing_start]);
    let closing_after = after
        .chars()
        .next()
        .map(|ch| after.get(..ch.len_utf8()).unwrap_or(""));
    let marker = marker as char;
    let opener_before = opening_before;
    let opener_after = opening_after;
    let closer_before = closing_before;
    let closer_after = closing_after;
    can_open_with_neighbors(opener_before, opener_after, marker)
        && !can_close_with_neighbors(opener_before, opener_after, marker)
        && can_close_with_neighbors(closer_before, closer_after, marker)
        && !can_open_with_neighbors(closer_before, closer_after, marker)
}

fn flanking(before: Option<&str>, after: Option<&str>) -> Flanking {
    let before_whitespace = is_commonmark_whitespace(before);
    let after_whitespace = is_commonmark_whitespace(after);
    let before_punctuation = is_commonmark_punctuation(before);
    let after_punctuation = is_commonmark_punctuation(after);
    Flanking {
        left: !after_whitespace && (!after_punctuation || before_whitespace || before_punctuation),
        right: !before_whitespace && (!before_punctuation || after_whitespace || after_punctuation),
        before_punctuation,
        after_punctuation,
    }
}

fn character_before(line: &str, byte_offset: usize) -> Option<&str> {
    line[..byte_offset]
        .char_indices()
        .next_back()
        .map(|(start, _)| &line[start..byte_offset])
}

fn character_after(line: &str, byte_offset: usize) -> Option<&str> {
    line[byte_offset..]
        .char_indices()
        .next()
        .map(|(offset, character)| {
            let start = byte_offset + offset;
            &line[start..start + character.len_utf8()]
        })
}

fn is_commonmark_whitespace(character: Option<&str>) -> bool {
    let Some(character) = character else {
        return true;
    };
    if character.is_ascii() {
        let byte = character.as_bytes()[0];
        return byte.is_ascii_whitespace() && byte != b'\x0B';
    }
    unicode_character_classes().whitespace.is_match(character)
}

fn is_commonmark_punctuation(character: Option<&str>) -> bool {
    let Some(character) = character else {
        return false;
    };
    if character.is_ascii() {
        return character.as_bytes()[0].is_ascii_punctuation();
    }
    unicode_character_classes().punctuation.is_match(character)
}

fn unicode_character_classes() -> &'static UnicodeCharacterClasses {
    UNICODE_CHARACTER_CLASSES.get_or_init(|| {
        /* WHY: CommonMarkは空白にZs、句読点にP/Sを使い、Rustの分類APIとは範囲が異なる。 */
        UnicodeCharacterClasses {
            whitespace: Regex::new(r"^\p{Zs}$").expect("Unicode whitespace pattern is valid"),
            punctuation: Regex::new(r"^[\p{P}\p{S}]$")
                .expect("Unicode punctuation pattern is valid"),
        }
    })
}
