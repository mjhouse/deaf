//! Various types of tables and associated table items
//!
//! Each table type can be created (using TryFrom) from an appropriate section header
//! struct. TryFrom will fail if the section isn't the correct type (e.g. non-SHT_SYMTAB
//! section header cannot be converted into a `Table<SymbolItem>` struct).

mod info;
mod item;
mod table;

pub use info::{
    RelocationInfo,
    SymbolInfo
};

pub use item::{
    RelaItem,
    SymbolItem,
    StringItem,
    TableItem
};

pub use table::{
    RelaTable,
    SymbolTable,
    StringTable,
    Table
};