use crate::common::{Width, Layout, FromBytes, IntoBytes, Convert, Ranges, Field};
use crate::errors::{Result};

use std::fmt::Debug;

/// An item in a section, table item etc that contains a field 
/// and associated value.
#[derive(Debug,Clone)]
pub struct Item<T32 = u8, T64 = T32, Out = T64>
where
    T32: FromBytes + IntoBytes + Convert<Out>,
    T64: FromBytes + IntoBytes + Convert<Out>,
    Out: Convert<T32> + Convert<T64> + Debug + Clone + Default,
{
    field: Field<T32,T64,Out>,
    value: Out,
}

impl<T32, T64, Out> Item<T32, T64, Out>
where
    T32: FromBytes + IntoBytes + Convert<Out>,
    T64: FromBytes + IntoBytes + Convert<Out>,
    Out: Convert<T32> + Convert<T64> + Debug + Clone + Default,
{
    /// Create a new item with given ranges
    pub fn new(ranges: Ranges) -> Self {
        Self {
            field: Field::new(ranges),
            value: Default::default()
        }
    }

    /// Create a new item with ranges, width and layout
    pub fn make(ranges: Ranges, width: Width, layout: Layout) -> Self {
        Self::new(ranges)
            .with_width(width)
            .with_layout(layout)
    }

    /// Builder method to set the initial width
    pub fn with_width(mut self, width: Width) -> Self {
        self.set_width(width);
        self
    }

    /// Builder method to set the initial layout
    pub fn with_layout(mut self, layout: Layout) -> Self {
        self.set_layout(layout);
        self
    }

    /// Builder method to set the initial value
    pub fn with_value(mut self, value: Out) -> Self {
        self.set(value);
        self
    }

    /// Builder method to parse a byte buffer
    pub fn parse(mut self, bytes: &[u8]) -> Result<Self> {
        self.read(bytes)?;
        Ok(self)
    }

    /// Read the value if possible
    pub fn read(&mut self, bytes: &[u8]) -> Result<Out> {
        self.value = self.field.get(bytes)?;
        Ok(self.value.clone())
    }

    /// Write the value if there is one
    pub fn write(&self, bytes: &mut [u8]) -> Result<()> {
        self.field.set(bytes,self.value.clone())
    }

    /// Get the output value of the item
    pub fn get(&self) -> Out {
        self.value.clone()
    }

    /// Set the output value of the item
    pub fn set(&mut self, value: Out) {
        self.value = value
    }

    /// Get the size of the item if there is a value
    pub fn size(&self) -> usize {
        self.field.size()
    }

    /// Get the width (32- or 64-bit) of the item
    pub fn width(&self) -> Width {
        self.field.width()
    }

    /// Set the width (32- or 64-bit) of the item
    pub fn set_width(&mut self, width: Width) {
        self.field.set_width(width);
    }

    /// Get the layout (little- or big-endian) of the item
    pub fn layout(&self) -> Layout {
        self.field.layout()
    }

    /// Set the layout (little- or big-endian) of the item
    pub fn set_layout(&mut self, layout: Layout) {
        self.field.set_layout(layout);
    }

}

impl<T32, T64, Out> PartialEq for Item<T32, T64, Out>
where
    T32: FromBytes + IntoBytes + Convert<Out>,
    T64: FromBytes + IntoBytes + Convert<Out>,
    Out: Convert<T32> + Convert<T64> + Debug + Clone + Default + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.get().eq(&other.get())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::ranges::*;

    #[test]
    fn test_read_out_of_range() {
        let bytes: &[u8] = &[ 0x00, 0x01 ];

        // build the item for testing
        let ranges = Ranges::new(0x00..0x04,0x00..0x04);
        let mut item: Item<u32,u64> = Item::new(ranges);

        // verify that small byte buffer fails
        assert!(item.read(bytes).is_err());
    }

}