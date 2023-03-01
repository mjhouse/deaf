use crate::errors::{Error, Result};
use crate::headers::common::constants::{Width,Layout};
use crate::headers::common::field::Field;
use crate::headers::common::ranges::*;
use crate::impl_property;

#[derive(Debug,Clone)]
pub struct InitializationValues {
    address: i64,
}

#[derive(Clone)]
pub struct Initialization {
    layout: Layout,
    width: Width,
    address: Field<i32,i64>,
    values: InitializationValues,
}

impl InitializationValues {

    pub fn new() -> Self {
        Self {
            address: 0,
        }
    }

}

impl Initialization {

    pub fn new(layout: Layout, width: Width) -> Self {
        Self {
            layout, width,
            address: Field::new(INIT_ADDRESS),
            values: InitializationValues::new(),
        }
    }

    pub fn parse(b: &[u8], layout: Layout, width: Width) -> Result<Self> {
        let mut initialization = Self::new(layout,width);
        initialization.read(b)?;
        Ok(initialization)
    }

    fn set_layout(&mut self, layout: Layout) {
        self.address.layout = layout;
    }

    fn set_width(&mut self, width: Width) {
        self.address.ranges.width = width;
    }

    pub fn read(&mut self, b: &[u8]) -> Result<InitializationValues> {
        self.set_layout(self.layout);
        self.set_width(self.width);

        self.values.address  = self.address.get(b)?;
        Ok(self.values.clone())
    }

    pub fn write(&self, b: &mut [u8]) -> Result<()> {
        self.address.set(b,self.values.address)?;
        Ok(())
    }

    impl_property!(address,address,i64);

}

impl std::fmt::Debug for Initialization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Initialization")
         .field("address", &self.values.address)
         .finish()
    }
}