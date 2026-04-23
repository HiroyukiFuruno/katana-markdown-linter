pub mod blockquote;
pub mod code;
pub mod emphasis;
pub mod heading;
pub mod html;
pub mod line;
pub mod link;
pub mod list;
pub mod structure;
pub mod whitespace;

use crate::parser::MarkdownAst;
use crate::types::{LintResult, RuleMeta};

pub trait Rule: Send + Sync {
    fn meta(&self) -> RuleMeta;

    fn check(&self, content: &str, ast: &MarkdownAst) -> Vec<LintResult>;

    fn fix(&self, _content: &str, _ast: &MarkdownAst) -> Option<String> {
        None
    }
}
