use crate::rules::markdown::document::SourceRange;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InlineCodeSpan {
    pub line: usize,
    pub marker_len: usize,
    pub content_range: SourceRange,
    pub full_range: SourceRange,
    pub closed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineHtmlAttribute<'a> {
    pub name: &'a str,
    pub value: Option<&'a str>,
    pub name_range: SourceRange,
    pub value_range: Option<SourceRange>,
    pub full_range: SourceRange,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineHtmlElement<'a> {
    pub line: usize,
    pub name: &'a str,
    pub name_range: SourceRange,
    pub attributes: Vec<InlineHtmlAttribute<'a>>,
    pub full_range: SourceRange,
    pub closing: bool,
}

impl<'a> InlineHtmlElement<'a> {
    pub fn attribute_value(&self, name: &str) -> Option<&'a str> {
        self.attributes
            .iter()
            .filter(|attribute| attribute.name.eq_ignore_ascii_case(name))
            .find_map(|attribute| attribute.value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InlineLinkKind {
    Inline,
    ImageInline,
    ReferenceFull,
    ImageReferenceFull,
    ReferenceCollapsed,
    ImageReferenceCollapsed,
    AutoLink,
}

impl InlineLinkKind {
    pub fn is_image(self) -> bool {
        matches!(
            self,
            Self::ImageInline | Self::ImageReferenceFull | Self::ImageReferenceCollapsed
        )
    }

    pub fn is_inline(self) -> bool {
        matches!(self, Self::Inline | Self::ImageInline)
    }

    pub fn is_reference(self) -> bool {
        matches!(
            self,
            Self::ReferenceFull
                | Self::ImageReferenceFull
                | Self::ReferenceCollapsed
                | Self::ImageReferenceCollapsed
        )
    }

    pub fn is_collapsed_reference(self) -> bool {
        matches!(
            self,
            Self::ReferenceCollapsed | Self::ImageReferenceCollapsed
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineLink<'a> {
    pub line: usize,
    pub kind: InlineLinkKind,
    pub text: Option<&'a str>,
    pub label: Option<&'a str>,
    pub destination: Option<&'a str>,
    pub text_range: Option<SourceRange>,
    pub label_range: Option<SourceRange>,
    pub destination_range: Option<SourceRange>,
    pub full_range: SourceRange,
}

impl<'a> InlineLink<'a> {
    pub fn effective_label(&self) -> Option<&'a str> {
        self.label.or(self.text)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceDefinition<'a> {
    pub line: usize,
    pub label: &'a str,
    pub destination: &'a str,
    pub label_range: SourceRange,
    pub destination_range: SourceRange,
    pub full_range: SourceRange,
}
