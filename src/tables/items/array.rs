use crate::errors::Result;
use crate::common::{Width,Layout};
use crate::common::ranges::ADDRESS;
use crate::tables::TableItem;
use crate::common::Item;

/// A specialization of item for reading addresses from Arrays
#[derive(Clone,PartialEq)]
pub struct ArrayItem {
    item: Item<i32,i64>
}

impl ArrayItem {

    pub(crate) fn make(value: i64) -> Self {
        Self::new().with_value(value)
    }

    pub(crate) fn new() -> Self {
        Self { item: Item::new(ADDRESS) }
    }

    /// Builder method to add a layout to an ArrayItem
    pub(crate) fn with_layout(mut self, layout: Layout) -> Self {
        self.set_layout(layout);
        self
    }

    /// Builder method to add a width to an ArrayItem
    pub(crate) fn with_width(mut self, width: Width) -> Self {
        self.set_width(width);
        self
    }

    /// Builder method to add a value to an ArrayItem
    pub(crate) fn with_value(mut self, value: i64) -> Self {
        self.set_value(value);
        self
    }

    /// Get the internal value 
    pub fn value(&self) -> i64 {
        self.item.get()
    }

    /// Set the internal value
    pub fn set_value(&mut self, value: i64) {
        self.item.set(value);
    }

}

impl TableItem for ArrayItem {

    fn set_layout(&mut self, layout: Layout){
        self.item.set_layout(layout);
    }

    fn set_width(&mut self, width: Width){
        self.item.set_width(width);
    }

    fn read(&mut self, b: &[u8]) -> Result<()> {
        self.item.read(b).map(|_| ())
    }

    fn write(&self, b: &mut [u8]) -> Result<()> {
        self.item.write(b)
    }

    fn size(&self) -> usize {
        self.item.size()
    }

}

impl Default for ArrayItem {
    fn default() -> Self {
        Self::new()
    }
}

impl From<i64> for ArrayItem {
    fn from(value: i64) -> Self {
        Self::make(value)
    }
}