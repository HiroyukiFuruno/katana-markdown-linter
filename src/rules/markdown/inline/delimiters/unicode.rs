use regex::Regex;
use std::sync::OnceLock;

static UNICODE_CHARACTER_CLASSES: OnceLock<UnicodeCharacterClasses> = OnceLock::new();

struct UnicodeCharacterClasses {
    whitespace: Regex,
    punctuation: Regex,
}

pub(super) fn is_commonmark_whitespace(character: Option<&str>) -> bool {
    let Some(character) = character else {
        return true;
    };
    if character.is_ascii() {
        let byte = character.as_bytes()[0];
        return byte.is_ascii_whitespace() && byte != b'\x0B';
    }
    unicode_character_classes().whitespace.is_match(character)
}

pub(super) fn is_commonmark_punctuation(character: Option<&str>) -> bool {
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
