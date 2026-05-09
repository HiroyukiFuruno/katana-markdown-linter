mod code_spans;
mod html;
mod links;
mod reference_definitions;
mod scan;
mod types;

pub(in crate::rules::markdown) use code_spans::extract_inline_code_spans;
pub(in crate::rules::markdown) use html::extract_inline_html_elements;
pub(in crate::rules::markdown) use links::extract_inline_links;
pub(in crate::rules::markdown) use reference_definitions::extract_reference_definitions;
pub use types::{
    InlineCodeSpan, InlineHtmlAttribute, InlineHtmlElement, InlineLink, InlineLinkKind,
    ReferenceDefinition,
};
