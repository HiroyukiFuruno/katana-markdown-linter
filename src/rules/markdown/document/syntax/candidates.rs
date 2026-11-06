pub(super) struct ParseCandidates {
    pub(super) blocks: bool,
    pub(super) html: bool,
}

impl ParseCandidates {
    pub(super) fn scan(content: &str) -> Self {
        /* WHY: ラベル付きコンテナの構文は再実装せず、候補を共有パーサーへ渡す。 */
        let first_footnote_marker = content.find("[^");
        let could_start_block = |offset| {
            first_footnote_marker.is_some_and(|marker| marker < offset)
                || has_container_prefix(&content.as_bytes()[..offset])
        };
        let mut html = false;
        let mut html_blocks = false;
        for (offset, _) in content.match_indices('<') {
            if could_begin_html(&content.as_bytes()[offset + 1..]) {
                html = true;
                if could_start_block(offset) {
                    html_blocks = true;
                    break;
                }
            }
        }

        let fences = || {
            content
                .match_indices("```")
                .chain(content.match_indices("~~~"))
                .any(|(offset, _)| could_start_block(offset))
        };
        Self {
            blocks: html_blocks || fences(),
            html,
        }
    }
}

fn could_begin_html(after_angle: &[u8]) -> bool {
    if matches!(after_angle.first(), Some(b'!' | b'?')) {
        return true;
    }
    let tag = after_angle.strip_prefix(b"/").unwrap_or(after_angle);
    if !tag.first().is_some_and(u8::is_ascii_alphabetic) {
        return false;
    }
    tag.iter()
        .copied()
        .find(|byte| !byte.is_ascii_alphanumeric() && *byte != b'-')
        .is_none_or(|byte| byte.is_ascii_whitespace() || matches!(byte, b'/' | b'>'))
}

fn has_container_prefix(before_marker: &[u8]) -> bool {
    /* WHY: コンテナ記号の候補だけを絞る。実際のブロック境界は共有パーサーで確定する。 */
    before_marker
        .iter()
        .rev()
        .find(|&&byte| {
            !matches!(
                byte,
                b' ' | b'\t' | b'>' | b'-' | b'+' | b'*' | b'.' | b')' | b':' | b'0'
                    ..=b'9' | b'[' | b']' | b'x' | b'X'
            )
        })
        .is_none_or(|&byte| matches!(byte, b'\n' | b'\r'))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::markdown::document::syntax::COMMONMARK_OPTIONS;
    use pulldown_cmark::{CodeBlockKind, Event, Parser, Tag};

    #[test]
    fn keeps_parser_required_for_raw_html_and_nested_fences() {
        for source in [
            "A <span title=\"attribute\">word</span>.\n",
            "A <!-- comment --> word.\n",
            "A <?target data?> word.\n",
            "A <!DOCTYPE html> word.\n",
            "A <![CDATA[<]]> word.\n",
            "<script>\nraw\n",
            "<!-- comment\n",
            "<?target\n",
            "<!DOCTYPE html\n",
            "<![CDATA[\nraw\n",
            "> 1. <section>\n>    raw\n",
            "- > <custom-tag>\n",
            "> - > ~~~text\n>   > payload\n",
            "- > - ```text\n  >   payload\n",
            "[^note]: ```text\n    #Heading\n",
            "[^脚注]: <pre>\n    payload\n",
            "Term\n: ~~~text\n  #Heading\n",
        ] {
            let candidates = ParseCandidates::scan(source);
            let mut saw_relevant_event = false;
            for event in Parser::new_ext(source, COMMONMARK_OPTIONS) {
                match event {
                    Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(_))) => {
                        assert!(candidates.blocks, "{source:?}");
                        saw_relevant_event = true;
                    }
                    Event::Start(Tag::HtmlBlock) => {
                        assert!(candidates.blocks && candidates.html, "{source:?}");
                        saw_relevant_event = true;
                    }
                    Event::InlineHtml(_) | Event::Html(_) => {
                        assert!(candidates.html, "{source:?}");
                        saw_relevant_event = true;
                    }
                    _ => {}
                }
            }
            assert!(saw_relevant_event, "{source:?}");
        }
    }

    #[test]
    fn excludes_autolinks_and_keeps_inline_html_out_of_the_block_path() {
        let autolinks = ParseCandidates::scan("See <https://example.org> and <hi@example.org>.");
        assert!(!autolinks.blocks && !autolinks.html);

        let inline_html = ParseCandidates::scan("See <span>text</span>.");
        assert!(!inline_html.blocks && inline_html.html);
    }
}
