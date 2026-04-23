/// Parsed Markdown document scaffold.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MarkdownAst {
    pub content: String,
}

pub fn parse(content: &str) -> MarkdownAst {
    MarkdownAst {
        content: content.to_owned(),
    }
}
