use crate::common::{Width, Layout, FromBytes, IntoBytes, Convert, Ranges, Field};
use crate::common::{ByteIter,Item,T32Value,T64Value,TOutValue};
use crate::errors::{Result,Error};

#[derive(Clone)]
pub struct ItemArray<T32, T64 = T32, Out = T64>
where
    T32: T32Value<Out>,
    T64: T64Value<Out>,
    Out: TOutValue<T32,T64>,
{
    offsets: Vec<usize>,
    item: Item<T32,T64,Out>
}

impl<T32,T64,Out> ItemArray<T32, T64, Out>
where
    T32: T32Value<Out>,
    T64: T64Value<Out>,
    Out: TOutValue<T32,T64>,
{

    /// Create a new item array with given ranges
    pub fn new(ranges: Ranges) -> Self {
        Self { 
            offsets: Vec::new(),
            item: Item::new(ranges) 
        }
    }

    /// Create a new item array with given ranges, layout, and width
    pub fn make(ranges: Ranges, width: Width, layout: Layout) -> Self {
        Self::new(ranges)
            .with_layout(layout)
            .with_width(width)
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

    /// Builder method to add an array offset
    pub fn with_offset(mut self, offset: usize) -> Self {
        self.add_offset(offset);
        self
    }

    /// Builder method to set the last offset for the array
    pub fn with_last_offset(mut self, offset: usize) -> Self {
        self.offsets.pop();
        self.add_offset(offset);
        self
    }

    /// Get an iterator over slices of the data
    pub fn iterator<'a>(&'a self, bytes: &'a [u8]) -> ByteIter {
        ByteIter::length(bytes,self.item_size())
    }

    /// Read a value at a particular index in the data
    pub fn read(&self, bytes: &[u8], index: usize) -> Result<Out> {
        self.item
            .clone()
            .with_layout(self.layout())
            .with_width(self.width())
            .with_offset(self.offset())
            .with_index(index)
            .parse(bytes)
            .map(|i| i.get())
    }

    /// Get the offset of a particular index in the array
    pub fn item_offset(&self, index: usize) -> usize {
        self.item_size() * index
    }

    /// Get the size of the item for the current width
    pub fn item_size(&self) -> usize {
        self.item.size()
    }

    /// Get the length of the array, given the item count
    pub fn length(&self, count: usize) -> usize {
        self.item_offset(count)
    }

    /// Get the width (32- or 64-bit) of the item
    pub fn width(&self) -> Width {
        self.item.width()
    }

    /// Set the width (32- or 64-bit) of the item
    pub fn set_width(&mut self, width: Width) {
        self.item.set_width(width);
    }

    /// Get the layout (little- or big-endian) of the item
    pub fn layout(&self) -> Layout {
        self.item.layout()
    }

    /// Set the layout (little- or big-endian) of the item
    pub fn set_layout(&mut self, layout: Layout) {
        self.item.set_layout(layout);
    }

    /// Get the total offset at which the array will read
    pub fn offset(&self) -> usize {
        self.offsets.iter().sum()
    }

    /// Set the offset of the array in the parsed data
    pub fn add_offset(&mut self, offset: usize) {
        self.offsets.push(offset);
    }

}