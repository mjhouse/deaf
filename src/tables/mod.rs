//! Various types of tables and associated table items
//!
//! Each table type can be created (using TryFrom) from an appropriate section
//! struct. TryFrom will fail if the section isn't the correct type (e.g. non-SHT_SYMTAB
//! section header cannot be converted into a `Table<SymbolItem>` struct).

mod info;
mod items;
mod table;
mod hash;

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
    ArrayItem,
};

pub use table::{
    Table,
    TableMut,
    TableView,
    Array,
    ArrayMut,
    SymbolTable,
    SymbolTableMut,
    RelTable,
    RelTableMut,
    RelaTable,
    RelaTableMut,
    StringTable,
    StringTableMut,
};

pub use hash::{
    HashTable,
    HashTableMut,
};