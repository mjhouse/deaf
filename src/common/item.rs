use crate::headers::common::constants::{Width,Layout};
use crate::common::{FromBytes, IntoBytes, Convert, Ranges, Field};
use crate::errors::{Result};

use std::fmt::Debug;

#[derive(Debug,Clone)]
pub struct Item<T32 = u8, T64 = T32, Out = T64>
where
    T32: FromBytes + IntoBytes + Convert<Out>,
    T64: FromBytes + IntoBytes + Convert<Out>,
    Out: Convert<T32> + Convert<T64> + Debug + Clone,
{
    field: Field<T32,T64,Out>,
    value: Option<Out>,
}

impl<T32, T64, Out> Item<T32, T64, Out>
where
    T32: FromBytes + IntoBytes + Convert<Out>,
    T64: FromBytes + IntoBytes + Convert<Out>,
    Out: Convert<T32> + Convert<T64> + Debug + Clone,
{
    pub fn new(ranges: Ranges) -> Self {
        Self {
            field: Field::new(ranges),
            value: None
        }
    }

    pub fn with_width(mut self, width: Width) -> Self {
        self.set_width(width);
        self
    }

    pub fn with_layout(mut self, layout: Layout) -> Self {
        self.set_layout(layout);
        self
    }

    pub fn with_value(mut self, value: Out) -> Self {
        self.set(value);
        self
    }

    pub fn parse(mut self, bytes: &[u8]) -> Result<Self> {
        self.read(bytes)?;
        Ok(self)
    }

    /// Read the value if possible
    pub fn read(&mut self, bytes: &[u8]) -> Result<()> {
        self.value = Some(self.field.get(bytes)?);
        Ok(())
    }

    /// Write the value if there is one
    pub fn write(&self, bytes: &mut [u8]) -> Result<()> {
        if let Some(v) = &self.value {
            self.field.set(bytes,v.clone())?;
        }
        Ok(())
    }

    pub fn get(&self) -> Option<Out> {
        self.value.clone()
    }

    pub fn set(&mut self, value: Out) {
        self.value = Some(value)
    }

    pub fn width(&self) -> Width {
        self.field.ranges.width
    }

    pub fn set_width(&mut self, width: Width) {
        self.field.ranges.width = width;
    }

    pub fn layout(&self) -> Layout {
        self.field.layout
    }

    pub fn set_layout(&mut self, layout: Layout) {
        self.field.layout = layout;
    }

}