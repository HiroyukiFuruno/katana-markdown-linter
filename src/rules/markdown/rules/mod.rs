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

pub mod md043;
pub use md043::*;

pub mod md044;
pub use md044::*;

pub mod md046;
pub use md046::*;

pub mod md048;
pub use md048::*;

pub mod md049;
pub use md049::*;

pub mod md050;
pub use md050::*;

pub mod md051;
pub use md051::*;

pub mod md052;
pub use md052::*;

pub mod md053;
pub use md053::*;

pub mod md054;
pub use md054::*;

pub mod md055;
pub use md055::*;

pub mod md011;
pub use md011::*;

pub mod md009;
pub use md009::*;

pub mod md010;
pub use md010::*;

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

pub mod md033;
pub use md033::*;

pub mod md034;
pub use md034::*;

pub mod md039;
pub use md039::*;

pub mod md005;
pub use md005::*;

pub mod md056;
pub use md056::*;

pub mod md058;
pub use md058::*;

pub mod md059;
pub use md059::*;

pub mod md060;
pub use md060::*;

pub mod list_indent;
pub use list_indent::*;

pub mod list;
pub use list::*;

pub mod list_ext;
pub use list_ext::*;

pub(crate) mod list_context;

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
