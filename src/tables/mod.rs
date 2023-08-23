//! Various types of tables and associated table items
//!
//! Each table type can be created (using TryFrom) from an appropriate section header
//! struct. TryFrom will fail if the section isn't the correct type (e.g. non-SHT_SYMTAB
//! section header cannot be converted into a `Table<SymbolItem>` struct).

mod info;
mod items;
mod tables;

pub use info::{
    SymbolInfo,
    RelocationInfo,
};

pub use items::{
    TableItem,
    RelItem,
    RelaItem,
    StringItem,
    SymbolItem,
};

pub use tables::{
    Table,
    RelTable,
    RelaTable,
    SymbolTable,
    StringTable,
};