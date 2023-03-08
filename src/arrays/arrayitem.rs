use crate::headers::common::constants::{Width,Layout};
use crate::common::{FromBytes, IntoBytes, Field, ranges::ADDRESS};
use crate::errors::{Error, Result};

/// A trait for array items
pub trait ArrayItem {
    
    /// Read from a byte array
    fn read(bytes: &[u8], width: Width, layout: Layout) -> Result<Self> where Self: Sized;
    
    /// Write to a mutable byte array
    fn write(&self, bytes: &mut [u8], width: Width, layout: Layout) -> Result<()>;

}

impl ArrayItem for i64 {

    fn read(bytes: &[u8], width: Width, layout: Layout) -> Result<Self> {
        Ok(match width {
            Width::X32 => i32::from_bytes(bytes,layout)? as i64,
            Width::X64 => i64::from_bytes(bytes,layout)?,
        })
    }

    fn write(&self, bytes: &mut [u8], width: Width, layout: Layout) -> Result<()> {
        Ok(match width {
            Width::X32 => (*self as i32).to_bytes(bytes,layout)?,
            Width::X64 => (*self as i64).to_bytes(bytes,layout)?,
        })
    }

}