//! Module that defines headers and associated items
//!
//! Also includes a lot of common functionality that is used everywhere 
//! else and should probably be moved out into it's own module (like Field 
//! and Ranges structs).

pub mod common;

pub(crate) mod file;
pub(crate) mod program;
pub(crate) mod section;

pub use file::header::FileHeader;
pub use program::header::ProgramHeader;
pub use section::header::SectionHeader;