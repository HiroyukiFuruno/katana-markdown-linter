use super::types::Link;
use crate::rules::markdown::inline::InlineLink;

pub(super) fn extract_links<'a>(inline_links: &[InlineLink<'a>]) -> Vec<Link<'a>> {
    inline_links
        .iter()
        .filter(|link| link.kind.is_inline() && !link.kind.is_image())
        .filter_map(|link| {
            Some(Link {
                line: link.line,
                text: link.text?,
                destination: link.destination?,
                text_range: link.text_range?,
                destination_range: link.destination_range?,
            })
        })
        .collect()
}
