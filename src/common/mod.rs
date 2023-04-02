//! Common structs and types used by all other modules
//!

pub mod ranges; // find byte slices to parse
mod bytes;  // parse bytes into values
mod field;  // read and write values
mod item;   // read and write with cache

mod iterator;
mod constants;

pub type Data = std::sync::Arc<std::sync::Mutex<Vec<u8>>>;

pub use bytes::{FromBytes,IntoBytes,Convert};
pub use ranges::Ranges;
pub use field::Field;
pub use item::Item;

pub use iterator::{
    ByteIter,
    ByteDelimiter,
};

pub use constants::{
    Width,
    Layout,
    PHType,
    SHType,
    SHFlags,
    STBind,
    STType,
};