use crate::common::{ByteDelimiter,Width,Layout};
use crate::errors::Result;
use crate::Section;

/// A record that can be extracted from a table section
pub trait TableItem: Default {

    /// Make a delimiter given expected entity size
    fn delimiter(size: usize) -> ByteDelimiter {
        ByteDelimiter::Length(size)
    }

    /// Parse the item directly from the byte array
    fn parse(b: &[u8], section: &Section) -> Result<Self> where Self: Sized {
        let mut item = Self::default();
        item.set_layout(section.layout());
        item.set_width(section.width());
        item.read(b)?;
        Ok(item)
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