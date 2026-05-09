mod blocks;
mod context;
mod fences;
mod headings;
mod lines;
mod links;
mod position;
mod tables;
mod types;

pub(super) use fences::fence_line_marker;
pub use types::{
    BlockRange, DocumentContext, FenceKind, Heading, LineInfo, Link, SourceRange, TableBlock,
    TableCell, TableRow,
};

#[cfg(test)]
mod tests;
