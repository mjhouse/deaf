use crate::errors::{Error, Result};
use crate::headers::common::constants::{Width,Layout};
use crate::headers::common::field::Field;
use crate::headers::common::ranges::*;
use crate::tables::symbol::info::SymbolInfo;
use crate::impl_property;

#[derive(Debug,Clone)]
pub struct SymbolValues {
    st_name: u32, 
    st_value: u64,
    st_size: u64, 
    st_info: SymbolInfo,  
    st_other: u8, 
    st_shndx: u16,
}

// https://docs.oracle.com/cd/E23824_01/html/819-0690/chapter6-79797.html
#[derive(Debug)]
pub struct Symbol {
    layout: Layout,
    width: Width,
    st_name: Field<u32,u32>,
    st_value: Field<u32,u64>,
    st_size: Field<u32,u64>,
    st_info: Field<u8,u8,SymbolInfo>,
    st_other: Field<u8>,
    st_shndx: Field<u16,u16>,
    values: SymbolValues,
}

impl SymbolValues {

    pub fn new() -> Self {
        Self {
            st_name: 0,
            st_value: 0,
            st_size: 0,
            st_info: SymbolInfo::empty(),
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

    pub fn write(&self, b: &mut [u8]) -> Result<()> {
        self.st_name.set(b,self.values.st_name)?;
        self.st_value.set(b,self.values.st_value)?;
        self.st_size.set(b,self.values.st_size)?;
        self.st_info.set(b,self.values.st_info)?;
        self.st_other.set(b,self.values.st_other)?;
        self.st_shndx.set(b,self.values.st_shndx)?;
        Ok(())
    }

    impl_property!(name,st_name,u32);
    impl_property!(value,st_value,u64);
    impl_property!(size,st_size,u64);
    impl_property!(info,st_info,SymbolInfo);
    impl_property!(other,st_other,u8);
    impl_property!(shndx,st_shndx,u16);

}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use std::io::Read;

    use crate::headers::common::constants::{ST_SIZE_32,ST_SIZE_64};
    use crate::headers::file::header::FileHeader;
    use crate::headers::section::header::SectionHeader;
    use crate::headers::common::constants::{STBind,STType};

    const TEST_BYTES1: &[u8] = &[
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x0a, 0x00, 0x98, 0x42, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 
        0x6a, 0x0c, 0x00, 0x00, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    const TEST_BYTES2: &[u8] = &[
        0xd4, 0x00, 0x00, 0x00, 0x12, 0x00, 0x0c, 0x00, 0x80, 0x4a, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x6a, 0x08, 0x00, 0x00, 0x12, 0x00, 0x0c, 0x00, 0x40, 0x7f, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x2f, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xa7, 0x06, 0x00, 0x00, 0x12, 0x00, 0x0c, 0x00, 0x80, 0x14, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0xee, 0x08, 0x00, 0x00, 0x12, 0x00, 0x0c, 0x00, 0x80, 0xb5, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x78, 0x08, 0x00, 0x00, 0x12, 0x00, 0x0c, 0x00, 0x70, 0xc4, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0xc4, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    #[test]
    fn test_symbol_parse_value() {
        let start = ST_SIZE_64 * 1;
        let end = start + ST_SIZE_64;

        let bytes = &TEST_BYTES1[start..end];
        let result = Symbol::parse(bytes,Layout::Little,Width::X64);

        assert!(result.is_ok());

        let symbol = result.unwrap();
        assert!(symbol.value() == 0x4298)
    }

    #[test]
    fn test_symbol_parse_info_section() {
        let start = ST_SIZE_64 * 1;
        let end = start + ST_SIZE_64;

        let bytes = &TEST_BYTES1[start..end];
        let result = Symbol::parse(bytes,Layout::Little,Width::X64);

        assert!(result.is_ok());
        let symbol = result.unwrap();
        
        let info = symbol.info();
        assert_eq!(info.typing(),STType::STT_SECTION);
        assert_eq!(info.binding(),STBind::STB_LOCAL);
    }

    #[test]
    fn test_symbol_parse_info_symbol() {
        let start = ST_SIZE_64 * 0;
        let end = start + ST_SIZE_64;

        let bytes = &TEST_BYTES2[start..end];
        let result = Symbol::parse(bytes,Layout::Little,Width::X64);

        assert!(result.is_ok());
        let symbol = result.unwrap();
        
        let info = symbol.info();

        assert_eq!(info.typing(),STType::STT_FUNC);
        assert_eq!(info.binding(),STBind::STB_GLOBAL);
    }

    #[test]
    fn test_symbol_write() {
        let mut result = [0;ST_SIZE_64];
        let bytes = &TEST_BYTES2[..ST_SIZE_64];
        let parsed = Symbol::parse(bytes,Layout::Little,Width::X64);

        assert!(parsed.is_ok());
        let symbol = parsed.unwrap();
        
        symbol.write(&mut result);
        assert_eq!(result,bytes);
    }

}