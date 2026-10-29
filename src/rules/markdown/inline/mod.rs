mod code_spans;
mod delimiters;
mod emphasis;
mod html;
mod links;
mod reference_definitions;
mod reversed_links;
mod scan;
mod types;

pub(in crate::rules::markdown) use code_spans::extract_inline_code_spans;
pub(in crate::rules::markdown) use code_spans::inline_code_spans_on_line;
pub(in crate::rules::markdown) use delimiters::isolated_outer_delimiter_scope;
pub(in crate::rules::markdown) use emphasis::{
    extract_emphasis_spans, safe_replacement, touches_inline_code, InlineEmphasisKind,
};
pub(in crate::rules::markdown) use html::extract_inline_html_elements;
pub(in crate::rules::markdown) use links::extract_inline_links;
pub(in crate::rules::markdown) use reference_definitions::extract_reference_definitions;
pub(in crate::rules::markdown) use reversed_links::reversed_links_on_line;
pub use types::{
    InlineCodeSpan, InlineHtmlAttribute, InlineHtmlElement, InlineLink, InlineLinkKind,
    ReferenceDefinition,
};
