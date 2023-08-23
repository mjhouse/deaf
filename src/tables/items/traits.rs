use crate::common::{ByteDelimiter,Width,Layout};
use crate::errors::Result;

/// A record that can be extracted from a table section
pub trait TableItem {

    /// Make a delimiter given expected entity size
    fn delimiter(size: usize) -> ByteDelimiter {
        ByteDelimiter::Length(size)
    }

    /// Read the item from a byte array
    fn read(&mut self, b: &[u8]) -> Result<()>;

    /// Write the item to a byte array
    fn write(&self, b: &mut [u8]) -> Result<()>;

    /// Set the layout if necessary
    fn set_layout(&mut self, _: Layout){}

    /// Set the width if necessary
    fn set_width(&mut self, _: Width){}

    /// Get the calculated size of the item
    fn size(&self) -> usize;

}