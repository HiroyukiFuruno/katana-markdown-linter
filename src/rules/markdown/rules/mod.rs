/* WHY: Section: Rule submodule aggregator
=======================================================
  All markdownlint rule implementations live under this directory.
  Each file is capped at 200 lines per coding-rules §2.1. */

pub mod blockquote;
pub use blockquote::*;

pub mod fences;
pub use fences::*;

pub mod content;
pub use content::*;

pub mod content_ext;
pub use content_ext::*;

pub mod heading;
pub use heading::*;

pub mod heading_ext;
pub use heading_ext::*;

pub mod heading_duplicates;
pub use heading_duplicates::*;

pub mod heading_style;
pub use heading_style::*;

pub mod image;
pub use image::*;

pub mod md011;
pub use md011::*;

pub mod md013;
pub use md013::*;

pub mod md014;
pub use md014::*;

pub mod md018;
pub use md018::*;

pub mod md019;
pub use md019::*;

pub mod md020;
pub use md020::*;

pub mod md021;
pub use md021::*;

pub mod md034;
pub use md034::*;

pub mod md039;
pub use md039::*;

pub mod list_indent;
pub use list_indent::*;

pub mod list;
pub use list::*;

pub mod list_ext;
pub use list_ext::*;

pub mod list_spacing;
pub use list_spacing::*;

pub mod spaces_in_code;
pub use spaces_in_code::*;

pub mod spaces_in_emphasis;
pub use spaces_in_emphasis::*;

pub mod style;
pub use style::*;

pub mod whitespace;
pub use whitespace::*;
