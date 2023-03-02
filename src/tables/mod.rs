//! Module that defines tables and associated items
//!
//! Each table type can be created (using TryFrom) from an appropriate section header
//! struct. TryFrom will fail if the section isn't the correct type (e.g. non-SHT_SYMTAB
//! section header cannot be converted into a SymbolTable struct).

pub mod common;

pub mod relocation;
pub mod string;
pub mod symbol;