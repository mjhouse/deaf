//! Various types of tables and associated table items
//!
//! Each table type can be created (using TryFrom) from an appropriate section header
//! struct. TryFrom will fail if the section isn't the correct type (e.g. non-SHT_SYMTAB
//! section header cannot be converted into a `Table<SymbolItem>` struct).

mod relocation_info;
mod symbol_info;
mod table_item;
mod table;

pub use relocation_info::RelocationInfo;
pub use symbol_info::SymbolInfo;

pub use table_item::{
    RelocationItem,
    SymbolItem,
    StringItem
};

pub use table::{
    RelocationTable,
    SymbolTable,
    StringTable,
    Table
};