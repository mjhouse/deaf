use crate::errors::{Error, Result};
use crate::headers::common::constants::{Width,Layout};
use crate::headers::common::field::Field;
use crate::headers::common::ranges::*;
use crate::impl_property;

#[derive(Debug,Clone)]
pub struct RelocationValues {
    r_offset: u64, 
    r_info: u64,
    r_addend: Option<i64>,
}

// https://docs.oracle.com/cd/E23824_01/html/819-0690/chapter6-54839.html
#[derive(Debug,Clone)]
pub struct Relocation {
    layout: Layout,
    width: Width,
    r_offset: Field<u32,u64>, 
    r_info: Field<u32,u64>,
    r_addend: Field<i32,i64>,
    values: RelocationValues,
}

impl RelocationValues {

    pub fn new() -> Self {
        Self {
            r_offset: 0, 
            r_info: 0,
            r_addend: None,
        }
    }

}

impl Relocation {

    pub fn new(layout: Layout, width: Width) -> Self {
        Self {
            layout, width,
            r_offset: Field::new(RT_OFFSET), 
            r_info: Field::new(RT_INFO),
            r_addend: Field::new(RT_ADDEND),
            values: RelocationValues::new(),
        }
    }

    pub fn parse(b: &[u8], layout: Layout, width: Width) -> Result<Self> {
        let mut relocation = Self::new(layout,width);
        relocation.read(b)?;
        Ok(relocation)
    }

    fn set_layout(&mut self, layout: Layout) {
        self.r_offset.layout = layout;
        self.r_info.layout = layout;
        self.r_addend.layout = layout;
    }

    fn set_width(&mut self, width: Width) {
        self.r_offset.ranges.width = width;
        self.r_info.ranges.width = width;
        self.r_addend.ranges.width = width;
    }

    pub fn read(&mut self, b: &[u8]) -> Result<RelocationValues> {
        self.set_layout(self.layout);
        self.set_width(self.width);
        self.values.r_offset  = self.r_offset.get(b)?;
        self.values.r_info = self.r_info.get(b)?;

        // addend is optional, so ignore read errors
        self.values.r_addend  = self.r_addend.get(b).ok();

        Ok(self.values.clone())
    }

    pub fn write(&self, b: &mut [u8]) -> Result<()> {
        self.r_offset.set(b,self.values.r_offset)?;
        self.r_info.set(b,self.values.r_info)?;
        
        // some relocations don't have addends
        if let Some(a) = self.values.r_addend {
            self.r_addend.set(b,a)?;
        }

        Ok(())
    }

    impl_property!(offset,r_offset,u64);
    impl_property!(info,r_info,u64);
    impl_property!(addend,r_addend,Option<i64>);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relocation_parse_value() {
        // let start = ST_SIZE_64 * 1;
        // let end = start + ST_SIZE_64;

        // let bytes = &TEST_BYTES1[start..end];
        // let result = Symbol::parse(bytes,Layout::Little,Width::X64);

        // assert!(result.is_ok());

        // let symbol = result.unwrap();
        // assert!(symbol.value() == 0x4298)
    }
}