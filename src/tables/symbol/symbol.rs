use crate::errors::{Error, Result};
use crate::headers::common::constants::{Width,Layout,STType};
use crate::common::field::Field;
use crate::common::ranges::*;
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
#[derive(Clone)]
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

    pub fn is_object(&self) -> bool {
        self.info().kind() == STType::STT_OBJECT
    }

    pub fn is_function(&self) -> bool {
        self.info().kind() == STType::STT_FUNC
    }

    pub fn is_section(&self) -> bool {
        self.info().kind() == STType::STT_SECTION
    }

    impl_property!(name,st_name,u32);
    impl_property!(value,st_value,u64);
    impl_property!(size,st_size,u64);
    impl_property!(info,st_info,SymbolInfo);
    impl_property!(other,st_other,u8);
    impl_property!(shndx,st_shndx,u16);

}

impl std::fmt::Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Symbol")
         .field("name", &self.name())
         .field("value", &self.value())
         .field("size", &self.size())
         .field("info", &self.info())
         .field("other", &self.other())
         .field("shndx", &self.shndx())
         .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::headers::common::constants::{STBind,STType};
    use crate::utilities::tests::{LIBVPF_DYNSYM as TEST,read};

    #[test]
    fn test_symbol_parse_value() {
        // calculate the element size and position
        let index = 150;
        let start = TEST.entsize * index;
        let end = start + TEST.entsize;

        // parse the Symbol from the byte buffer
        let bytes = &TEST.bytes[start..end];
        let result = Symbol::parse(bytes,Layout::Little,Width::X64);

        // unwrap the resulting symbol
        assert!(result.is_ok());
        let symbol = result.unwrap();

        // verify that the value is expected (line 152)
        assert!(symbol.value() == 0x013530);
    }

    #[test]
    fn test_symbol_parse_info_object() {
        // calculate the element size and position
        let index = 99;
        let start = TEST.entsize * index;
        let end = start + TEST.entsize;

        // parse the Symbol from the byte buffer
        let bytes = &TEST.bytes[start..end];
        let result = Symbol::parse(bytes,Layout::Little,Width::X64);

        // unwrap the resulting symbol
        assert!(result.is_ok());
        let symbol = result.unwrap();
        
        // verify that the symbol info has expected values
        let info = symbol.info();
        assert_eq!(info.kind(),STType::STT_OBJECT);
        assert_eq!(info.bind(),STBind::STB_GLOBAL);
    }

    #[test]
    fn test_symbol_parse_info_symbol() {
        // calculate the element size and position
        let index = 150;
        let start = TEST.entsize * index;
        let end = start + TEST.entsize;

        // parse the Symbol from the byte buffer
        let bytes = &TEST.bytes[start..end];
        let result = Symbol::parse(bytes,Layout::Little,Width::X64);

        // unwrap the resulting symbol
        assert!(result.is_ok());
        let symbol = result.unwrap();
        
        // verify that the symbol info has expected values
        let info = symbol.info();
        assert_eq!(info.kind(),STType::STT_FUNC);
        assert_eq!(info.bind(),STBind::STB_GLOBAL);
    }

    #[test]
    fn test_symbol_write_no_change() {
        // calculate the element size and position
        let index = 150;
        let start = TEST.entsize * index;
        let end = start + TEST.entsize;

        // parse the Symbol from the byte buffer
        let mut result = [0;TEST.entsize];
        let bytes = &TEST.bytes[start..end];
        let parsed = Symbol::parse(bytes,Layout::Little,Width::X64);

        // unwrap the resulting symbol
        assert!(parsed.is_ok());
        let symbol = parsed.unwrap();
        
        // write the symbol back to the buffer
        symbol.write(&mut result);
        assert_eq!(&result,bytes);
    }

    #[test]
    fn test_symbol_write_with_change() {
        // calculate the element size and position
        let index = 150;
        let start = TEST.entsize * index;
        let end = start + TEST.entsize;

        // parse the Symbol from the byte buffer
        let mut result = [0;TEST.entsize];
        let bytes = &TEST.bytes[start..end];
        let parsed = Symbol::parse(bytes,Layout::Little,Width::X64);

        // unwrap the resulting symbol
        assert!(parsed.is_ok());
        let mut symbol = parsed.unwrap();

        // change the value of the symbol
        symbol.set_value(123);
        
        // write the symbol back to the buffer
        symbol.write(&mut result);
        assert_ne!(&result,bytes);

        // re-parse the symbol from the result buffer
        let parsed = Symbol::parse(&result,Layout::Little,Width::X64);

        // unwrap the resulting symbol
        assert!(parsed.is_ok());
        let symbol = parsed.unwrap();

        // verify that the re-parsed symbol has the set value
        assert_eq!(symbol.value(),123);
    }

}