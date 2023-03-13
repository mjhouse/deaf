
pub mod macros;
pub mod constants;
pub mod ranges; // find byte slices to parse
pub mod bytes;  // parse bytes into values
pub mod field;  // read and write values
pub mod item;   // read and write with cache

pub use bytes::{FromBytes,IntoBytes,Convert};
pub use ranges::Ranges;
pub use field::Field;
pub use item::Item;

pub use constants::{
    Width,
    Layout,
    PHType,
    SHType,
    SHFlags,
    STBind,
    STType,
};