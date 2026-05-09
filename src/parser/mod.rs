/// Parsed Markdown document scaffold.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MarkdownAst {
    pub content: String,
}

pub struct MarkdownParser;

impl MarkdownParser {
    pub fn parse(content: &str) -> MarkdownAst {
        MarkdownAst {
            content: content.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_preserves_source_content() {
        let ast = MarkdownParser::parse("# Title\n\nParagraph\n");

        assert_eq!(ast.content, "# Title\n\nParagraph\n");
    }
}
