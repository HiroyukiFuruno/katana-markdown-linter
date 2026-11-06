use super::ScopeKind;
use pulldown_cmark::Tag;

pub(super) fn remove_open_scope(scopes: &mut Vec<usize>, scope_index: usize) {
    if let Some(position) = scopes.iter().rposition(|open| *open == scope_index) {
        scopes.remove(position);
    }
}

pub(super) fn scope_kind(tag: &Tag<'_>) -> Option<ScopeKind> {
    match tag {
        Tag::Emphasis
        | Tag::Strong
        | Tag::Strikethrough
        | Tag::Superscript
        | Tag::Subscript
        | Tag::Link { .. }
        | Tag::Image { .. } => Some(ScopeKind::Inline),
        Tag::Paragraph
        | Tag::Heading { .. }
        | Tag::BlockQuote(_)
        | Tag::CodeBlock(_)
        | Tag::HtmlBlock
        | Tag::List(_)
        | Tag::Item
        | Tag::FootnoteDefinition(_)
        | Tag::DefinitionList
        | Tag::DefinitionListTitle
        | Tag::DefinitionListDefinition
        | Tag::Table(_)
        | Tag::TableHead
        | Tag::TableRow
        | Tag::TableCell
        | Tag::MetadataBlock(_) => Some(ScopeKind::Block),
    }
}
