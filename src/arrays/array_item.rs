use crate::errors::{Result};
use crate::common::{Width,Layout};
use crate::common::ranges::ADDRESS;
use crate::common::{Item};

/// Wraps and hides the more general purpose Item
pub struct ArrayItem(Item<i32,i64>);

impl ArrayItem {

    /// Create an ArrayItem with a value
    pub fn new(value: i64) -> Self {
        Self(Item::new(ADDRESS)
        .with_value(value))
    }

    /// Read an ArrayItem from a given byte buffer with layout and width
    pub(super) fn read(layout: Layout, width: Width, bytes: &[u8]) -> Result<Self> {
        Item::new(ADDRESS)
            .with_width(width)
            .with_layout(layout)
            .parse(bytes)
            .map(|i| Self(i))
    }

    /// Write this ArrayItem to the provided byte buffer
    pub(super) fn write(&self, bytes: &mut [u8]) -> Result<()> {
        self.0.write(bytes)?;
        Ok(())
    }

    /// Set the layout for this ArrayItem
    pub(super) fn set_layout(&mut self, layout: Layout) {
        self.0.set_layout(layout);
    }

    /// Set the width of this ArrayItem
    pub(super) fn set_width(&mut self, width: Width) {
        self.0.set_width(width);
    }

    /// Get the internal ArrayItem value 
    pub fn value(&self) -> Option<i64> {
        self.0.get()
    }

    /// Set the internal ArrayItem value
    pub fn set_value(&mut self, value: i64) {
        self.0.set(value);
    }

}