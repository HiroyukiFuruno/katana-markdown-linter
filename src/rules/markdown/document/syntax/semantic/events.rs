use pulldown_cmark::{Event, Tag, TagEnd};

#[derive(Clone, PartialEq, Eq)]
pub(super) enum SemanticEvent {
    Text(String),
    Structural(String),
    ParagraphStart,
    ParagraphEnd,
}

impl SemanticEvent {
    pub(super) fn from_parser(event: &Event<'_>) -> Self {
        match event {
            Event::Text(text) => Self::Text(text.to_string()),
            Event::Start(Tag::Paragraph) => Self::ParagraphStart,
            Event::End(TagEnd::Paragraph) => Self::ParagraphEnd,
            _ => Self::Structural(format!("{event:?}")),
        }
    }
}

pub(super) fn normalized_signature(events: &[SemanticEvent]) -> Vec<SemanticEvent> {
    let mut signature = Vec::new();
    let mut pending_text = String::new();
    for event in events {
        match event {
            SemanticEvent::Text(text) => pending_text.push_str(text),
            SemanticEvent::Structural(value) => {
                if !pending_text.is_empty() {
                    signature.push(SemanticEvent::Text(std::mem::take(&mut pending_text)));
                }
                signature.push(SemanticEvent::Structural(value.clone()));
            }
            SemanticEvent::ParagraphStart | SemanticEvent::ParagraphEnd => {
                if !pending_text.is_empty() {
                    signature.push(SemanticEvent::Text(std::mem::take(&mut pending_text)));
                }
                signature.push(event.clone());
            }
        }
    }
    if !pending_text.is_empty() {
        signature.push(SemanticEvent::Text(pending_text));
    }
    signature
}
