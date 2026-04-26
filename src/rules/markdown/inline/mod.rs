mod code_spans;
mod html;
mod links;
mod reference_definitions;
mod scan;
mod types;

pub(crate) use code_spans::extract_inline_code_spans;
pub(crate) use html::extract_inline_html_elements;
pub(crate) use links::extract_inline_links;
pub(crate) use reference_definitions::extract_reference_definitions;
pub use types::{
    InlineCodeSpan, InlineHtmlAttribute, InlineHtmlElement, InlineLink, InlineLinkKind,
    ReferenceDefinition,
};
