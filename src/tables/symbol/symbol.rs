use crate::errors::{Error, Result};
use crate::headers::common::constants::{Width,Layout};
use crate::headers::common::field::Field;
use crate::headers::common::ranges::*;
use crate::impl_property;

#[derive(Debug,Clone)]
pub struct SymbolValues {
    st_name: u32,  // st_name
    st_value: u64, // st_value
    st_size: u64,  // st_size
    st_info: u8,   // st_info
    st_other: u8,  // st_other
    st_shndx: u16, // st_shndx
}

// https://docs.oracle.com/cd/E23824_01/html/819-0690/chapter6-79797.html
#[derive(Debug)]
pub struct Symbol {
    layout: Layout,
    width: Width,
    st_name: Field<u32,u32>,  // st_name
    st_value: Field<u32,u64>, // st_value
    st_size: Field<u32,u64>,  // st_size
    st_info: Field<u8>,       // st_info
    st_other: Field<u8>,      // st_other
    st_shndx: Field<u16,u16>, // st_shndx
    values: SymbolValues,
}

impl SymbolValues {

    pub fn new() -> Self {
        Self {
            st_name: 0,
            st_value: 0,
            st_size: 0,
            st_info: 0,
            st_other: 0,
            st_shndx: 0,
        }
    }

}

impl Symbol {

    pub fn new(layout: Layout, width: Width) -> Self {
        Self {
            layout, width,
            st_name: Field::new(ST_NAME), 
            st_value: Field::new(ST_VALUE),
            st_size: Field::new(ST_SIZE), 
            st_info: Field::new(ST_INFO),    
            st_other: Field::new(ST_OTHER),     
            st_shndx: Field::new(ST_SHNDX),
            values: SymbolValues::new(),
        }
    }

    pub fn parse(b: &[u8], layout: Layout, width: Width) -> Result<Self> {
        let mut symbol = Self::new(layout,width);
        symbol.read(b)?;
        Ok(symbol)
    }

    fn set_layout(&mut self, layout: Layout) {
        self.st_name.layout = layout;
        self.st_value.layout = layout;
        self.st_size.layout = layout;
        self.st_info.layout = layout;
        self.st_other.layout = layout;
        self.st_shndx.layout = layout;
    }

    fn set_width(&mut self, width: Width) {
        self.st_name.ranges.width = width;
        self.st_value.ranges.width = width;
        self.st_size.ranges.width = width;
        self.st_info.ranges.width = width;
        self.st_other.ranges.width = width;
        self.st_shndx.ranges.width = width;
    }

    pub fn read(&mut self, b: &[u8]) -> Result<SymbolValues> {
        self.set_layout(self.layout);
        self.set_width(self.width);

        self.values.st_name  = self.st_name.get(b)?;
        self.values.st_value = self.st_value.get(b)?;
        self.values.st_size  = self.st_size.get(b)?;
        self.values.st_info  = self.st_info.get(b)?;
        self.values.st_other = self.st_other.get(b)?;
        self.values.st_shndx = self.st_shndx.get(b)?;
        Ok(self.values.clone())
    }

    impl_property!(name,st_name,u32);
    impl_property!(value,st_value,u64);
    impl_property!(size,st_size,u64);
    impl_property!(info,st_info,u8);
    impl_property!(other,st_other,u8);
    impl_property!(shndx,st_shndx,u16);

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::headers::common::constants::{ST_SIZE_32,ST_SIZE_64,SHType};
    use std::fs::File;
    use std::io::Read;
    use crate::headers::file::header::FileHeader;
    use crate::headers::section::header::SectionHeader;

    const TEST_BYTES: &[u8] = &[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x0a, 0x00, 0x98, 0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x6a, 0x0c, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0xb9, 0x07, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0xcc, 0x07, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0xf1, 0x07, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0xf7, 0x03, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0xae, 0x01, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x41, 0x05, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x01, 0x00, 0x00, 0x00, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0xb2, 0x07, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    #[test]
    fn test_symbol_parse() {
        let start = ST_SIZE_64 * 1;
        let end = start + ST_SIZE_64;

        let bytes = &TEST_BYTES[start..end];
        let result = Symbol::parse(bytes,Layout::Little,Width::X64);

        assert!(result.is_ok());
        let symbol = result.unwrap();
        
        dbg!(symbol.values.st_value);
        assert!(symbol.values.st_value == 0x4298)
    }

}