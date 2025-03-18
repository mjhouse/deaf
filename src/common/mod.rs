//! Common structs and types used by all other modules
//!

pub mod ranges; // find byte slices to parse
mod bytes;      // parse bytes into values
mod field;      // read and write values
mod item;       // read and write with cache
mod enums;      
mod update;
mod item_array;

mod iterator;
mod constants;

pub use bytes::{FromBytes,IntoBytes,Convert};
pub use ranges::Ranges;
pub use field::Field;

pub use item::{
    Item,
    T32Value,
    T64Value,
    TOutValue
};

pub use iterator::{
    ByteIter,
    ByteDelimiter,
};

pub use constants::{
    Width,
    Layout,
    OFType,
    PHType,
    SHType,
    SHFlags,
    SHIndex,
    STBind,
    STType,
    STVisibility,
};

pub use update::{
    Update, 
    Updateable,
    All
};

pub use enums::SectionType;
pub use item_array::ItemArray;