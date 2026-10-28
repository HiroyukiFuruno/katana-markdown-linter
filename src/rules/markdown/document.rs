mod blocks;
mod context;
mod fences;
mod headings;
mod lines;
mod links;
mod position;
mod syntax;
mod tables;
mod types;

pub(super) use fences::fence_line_marker;
pub(in crate::rules::markdown) use headings::parse_heading;
pub use types::{
    BlockRange, DocumentContext, FenceKind, Heading, LineInfo, Link, SourceRange, TableBlock,
    TableCell, TableRow,
};

#[cfg(test)]
mod tests;
