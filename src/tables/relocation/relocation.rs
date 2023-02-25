use crate::errors::{Error, Result};
use crate::headers::common::constants::{Width,Layout};
use crate::headers::common::field::Field;
use crate::headers::common::ranges::*;
use crate::tables::relocation::RelocationInfo;
use crate::impl_property;

#[derive(Debug,Clone)]
pub struct RelocationValues {
    r_offset: u64, 
    r_info: RelocationInfo,
    r_addend: Option<i64>,
}

// https://docs.oracle.com/cd/E23824_01/html/819-0690/chapter6-54839.html
#[derive(Clone)]
pub struct Relocation {
    layout: Layout,
    width: Width,
    r_offset: Field<u32,u64>, 
    r_info: Field<u32,u64,RelocationInfo>,
    r_addend: Field<i32,i64>,
    values: RelocationValues,
}

impl RelocationValues {

    pub fn new() -> Self {
        Self {
            r_offset: 0, 
            r_info: RelocationInfo::empty(),
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
    impl_property!(info,r_info,RelocationInfo);
    impl_property!(addend,r_addend,Option<i64>);

}

impl std::fmt::Debug for Relocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Relocation")
         .field("offset", &self.values.r_offset)
         .field("info", &self.values.r_info)
         .field("addend", &self.values.r_addend)
         .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::headers::common::constants::{RT_SIZE_64};

    const TEST_TABLE: &[u8] = include!("../../../assets/bytes/libvpf_rela.dyn.in");

    // the starting byte of the test table
    const TEST_TABLE_OFFSET: usize = 0;

    // the length in bytes of the test table
    const TEST_TABLE_LENGTH: usize = 1200;

    // the number of elements in the test table
    const TEST_TABLE_COUNT: usize = 50;

    // the size of an element in the test table
    const TEST_TABLE_ENTITY: usize = 24;

    #[test]
    fn test_relocation_parse_offset() {
        let start = TEST_TABLE_ENTITY * 1;
        let end = start + TEST_TABLE_ENTITY;

        let bytes = &TEST_TABLE[start..end];
        let result = Relocation::parse(bytes,Layout::Little,Width::X64);

        assert!(result.is_ok());

        let relocation = result.unwrap();
        assert!(relocation.offset() == 0x46b38);
    }

    #[test]
    fn test_relocation_parse_info() {
        let start = TEST_TABLE_ENTITY * 38;
        let end = start + TEST_TABLE_ENTITY;

        let bytes = &TEST_TABLE[start..end];
        let result = Relocation::parse(bytes,Layout::Little,Width::X64);

        assert!(result.is_ok());
        let relocation = result.unwrap();

        let info = relocation.info();

        // readelf -r assets/libvpf.so.4.1 ('fieldcol' in .rela.dyn)
        assert!(info.symbol() == 0xfe000000);
        assert!(info.kind() == 0x06); // R_X86_64_GLOB_DAT
    }

    #[test]
    fn test_relocation_parse_addend_rela() {
        // calculate table offset
        let start = TEST_TABLE_ENTITY * 1;
        let end = start + TEST_TABLE_ENTITY;

        // get a constrained slice and parse
        let bytes = &TEST_TABLE[start..end];
        let result = Relocation::parse(bytes,Layout::Little,Width::X64);

        // unwrap the resulting relocation
        assert!(result.is_ok());
        let relocation = result.unwrap();

        // verify that addend is expected value
        assert!(relocation.addend() == Some(0x57b0));
    }

    #[test]
    fn test_relocation_parse_addend_rel() {
        // the length of a 64-bit relocation with no addend
        let length = 16;

        // calculate table offset (using addend lengths)
        let start = TEST_TABLE_ENTITY * 1;
        let end = start + length; // leave off the addend

        // get a constrained slice and parse
        let bytes = &TEST_TABLE[start..end];
        let result = Relocation::parse(bytes,Layout::Little,Width::X64);

        // unwrap the resulting relocation
        assert!(result.is_ok());
        let relocation = result.unwrap();

        // verify that addend is none
        assert!(relocation.addend().is_none());
    }

    #[test]
    fn test_relocation_write_no_change() {
        // calculate table offset
        let mut result = [0;RT_SIZE_64];
        let bytes = &TEST_TABLE[..RT_SIZE_64];
        
        // parse the relocation from the buffer
        let parsed = Relocation::parse(bytes,Layout::Little,Width::X64);
        assert!(parsed.is_ok());

        let relocation = parsed.unwrap();
        
        // write the relocation back to the buffer
        relocation.write(&mut result);
        assert_eq!(&result,bytes);
    }

    #[test]
    fn test_relocation_write_change() {
        let mut result = [0;RT_SIZE_64];
        let bytes = &TEST_TABLE[..RT_SIZE_64];
        
        // parse a relocation record from the relocation table
        let parsed = Relocation::parse(bytes,Layout::Little,Width::X64);
        assert!(parsed.is_ok());

        // change the addend of the relocation
        let mut relocation = parsed.unwrap();
        relocation.set_addend(Some(-20));
        
        // write the relocation to a buffer
        relocation.write(&mut result);
        assert_ne!(&result,bytes);

        // re-parse the written data
        let parsed = Relocation::parse(&result,Layout::Little,Width::X64);
        assert!(parsed.is_ok());

        // verify that the re-parsed relocation has correct addend
        let relocation = parsed.unwrap();
        assert_eq!(relocation.addend(),Some(-20));
    }
}