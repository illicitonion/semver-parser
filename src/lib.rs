use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "semver.pest"]
struct SemverParser;

mod range_set;
pub use crate::range_set::Compat;
pub use crate::range_set::RangeSet;

mod range;
pub use crate::range::Range;

// from old lib:
pub mod lexer;
pub mod parser;
// pub mod range;
pub mod version;
