mod blockquote;
mod multiple_blanks;
mod trailing_newline;

pub use blockquote::NoMultipleSpaceBlockquoteRule;
pub use multiple_blanks::NoMultipleBlanksRule;
pub use trailing_newline::SingleTrailingNewlineRule;

#[cfg(test)]
mod tests;
