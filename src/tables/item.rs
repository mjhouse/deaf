use crate::common::{ByteDelimiter,Width,Layout,IntoBytes,FromBytes,Item,ranges::*};
use crate::tables::info::RelocationInfo;
use crate::tables::info::SymbolInfo;
use crate::errors::Result;
use std::ffi::CString;

/// A record that can be extracted from a table section
pub trait TableItem {

    /// Make a delimiter given expected entity size
    fn delimiter(size: usize) -> ByteDelimiter {
        ByteDelimiter::Length(size)
    }

    /// Read the item from a byte array
    fn read(&mut self, b: &[u8]) -> Result<()>;

    /// Write the item to a byte array
    fn write(&self, b: &mut [u8]) -> Result<()>;

    /// Set the layout if necessary
    fn set_layout(&mut self, _: Layout){}

    /// Set the width if necessary
    fn set_width(&mut self, _: Width){}

    /// Get the calculated size of the item
    fn size(&self) -> usize;

}

/// A String item found in string tables
#[derive(Default,Clone)]
pub struct StringItem {
    value: CString,
}

/// A Symbol item found in symbol tables
#[derive(Clone)]
pub struct SymbolItem {
    st_name: Item<u32,u32>,
    st_value: Item<u32,u64>,
    st_size: Item<u32,u64>,
    st_info: Item<u8,u8,SymbolInfo>,
    st_other: Item<u8>,
    st_shndx: Item<u16,u16>,
}

/// A Relocation item found in relocation tables
#[derive(Clone)]
pub struct RelItem {
    r_offset: Item<u32,u64>, 
    r_info: Item<u32,u64,RelocationInfo>,
}

/// A Relocation item found in relocation tables
#[derive(Clone)]
pub struct RelaItem {
    r_offset: Item<u32,u64>, 
    r_info: Item<u32,u64,RelocationInfo>,
    r_addend: Item<i32,i64>,
}

impl StringItem {

    /// Get the string value of the table item
    ///
    /// This method will fail if the string is not 
    /// valid UTF-8
    pub fn string(&self) -> Result<String> {
        Ok(self.value.clone().into_string()?.into())
    }

    /// Get the string value of the table item
    ///
    /// This method will replace invalid characters
    /// with U+FFFD (REPLACEMENT CHARACTER)
    pub fn string_lossy(&self) -> String {
        self.value.to_string_lossy().into()
    }

    /// Set the string value of the table item
    ///
    /// This method will fail if the string is not
    /// valid UTF-8
    pub fn set_string(&mut self, value: String) -> Result<()> {
        self.value = CString::new(value.as_bytes())?;
        Ok(())
    }

}

impl SymbolItem {

    /// Get the 'st_name' attribute (name *index*) of the symbol
    pub fn name(&self) -> u32 {
        self.st_name.get()
    }

    /// Set the 'st_name' attribute (name *index*) of the symbol
    pub fn set_name(&mut self, value: u32) {
        self.st_name.set(value);
    }

    /// Get the 'st_value' attribute of the symbol
    pub fn value(&self) -> u64 {
        self.st_value.get()
    }

    /// Set the 'st_value' attribute of the symbol
    pub fn set_value(&mut self, value: u64) {
        self.st_value.set(value);
    }

    /// Get the 'st_size' attribute of the symbol
    pub fn size(&self) -> u64 {
        self.st_size.get()
    }

    /// Set the 'st_size' attribute of the symbol
    pub fn set_size(&mut self, value: u64) {
        self.st_size.set(value);
    }

    /// Get the 'st_info' attribute of the symbol
    pub fn info(&self) -> SymbolInfo {
        self.st_info.get()
    }

    /// Set the 'st_info' attribute of the symbol
    pub fn set_info(&mut self, value: SymbolInfo) {
        self.st_info.set(value);
    }

    /// Get the 'st_other' attribute of the symbol
    pub fn other(&self) -> u8 {
        self.st_other.get()
    }

    /// Set the 'st_other' attribute of the symbol
    pub fn set_other(&mut self, value: u8) {
        self.st_other.set(value);
    }

    /// Get the 'st_shndx' attribute of the symbol
    pub fn shndx(&self) -> u16 {
        self.st_shndx.get()
    }

    /// Set the 'st_shndx' attribute of the symbol
    pub fn set_shndx(&mut self, value: u16) {
        self.st_shndx.set(value);
    }

}

impl RelaItem {

    /// Get the 'r_offset' attribute of the relocation
    pub fn offset(&self) -> u64 {
        self.r_offset.get()
    }

    /// Set the 'r_offset' attribute of the relocation
    pub fn set_offset(&mut self, value: u64) {
        self.r_offset.set(value);
    }

    /// Get the 'r_info' attribute of the relocation
    pub fn info(&self) -> RelocationInfo {
        self.r_info.get()
    }

    /// Set the 'r_info' attribute of the relocation
    pub fn set_info(&mut self, value: RelocationInfo) {
        self.r_info.set(value);
    }

    /// Get the 'r_addend' attribute of the relocation
    pub fn addend(&self) -> i64 {
        self.r_addend.get()
    }

    /// Set the 'r_addend' attribute of the relocation
    pub fn set_addend(&mut self, value: i64) {
        self.r_addend.set(value);
    }

}

impl TableItem for StringItem {

    // Override the default implementation
    fn delimiter(_: usize) -> ByteDelimiter {
        ByteDelimiter::Value(b'\0')
    }

    fn read(&mut self, b: &[u8]) -> Result<()> {
        self.value = CString::from_bytes(b,Layout::Little)?;
        Ok(())
    }

    fn write(&self, b: &mut [u8]) -> Result<()> {
        self.value.to_bytes(b, Layout::Little)
    }

    fn size(&self) -> usize {
        self.value.as_bytes_with_nul().len()
    }

}

impl TableItem for SymbolItem {

    fn set_layout(&mut self, layout: Layout){
        self.st_name.set_layout(layout);
        self.st_value.set_layout(layout);
        self.st_size.set_layout(layout);
        self.st_info.set_layout(layout);
        self.st_other.set_layout(layout);
        self.st_shndx.set_layout(layout);
    }

    fn set_width(&mut self, width: Width){
        self.st_name.set_width(width);
        self.st_value.set_width(width);
        self.st_size.set_width(width);
        self.st_info.set_width(width);
        self.st_other.set_width(width);
        self.st_shndx.set_width(width);
    }

    fn read(&mut self, b: &[u8]) -> Result<()> {
        self.st_name.read(b)?;
        self.st_value.read(b)?;
        self.st_size.read(b)?;
        self.st_info.read(b)?;
        self.st_other.read(b)?;
        self.st_shndx.read(b)?;
        Ok(())
    }

    fn write(&self, b: &mut [u8]) -> Result<()> {
        self.st_name.write(b)?;
        self.st_value.write(b)?;
        self.st_size.write(b)?;
        self.st_info.write(b)?;
        self.st_other.write(b)?;
        self.st_shndx.write(b)?;
        Ok(())
    }

    fn size(&self) -> usize {
        self.st_name.size() +
        self.st_value.size() +
        self.st_size.size() +
        self.st_info.size() +
        self.st_other.size() +
        self.st_shndx.size()
    }

}

impl TableItem for RelaItem {

    fn set_layout(&mut self, layout: Layout){
        self.r_offset.set_layout(layout);
        self.r_info.set_layout(layout);
        self.r_addend.set_layout(layout);
    }

    fn set_width(&mut self, width: Width){
        self.r_offset.set_width(width);
        self.r_info.set_width(width);
        self.r_addend.set_width(width);
    }

    fn read(&mut self, b: &[u8]) -> Result<()> {
        self.r_offset.read(b)?;
        self.r_info.read(b)?;
        self.r_addend.read(b)?;
        Ok(())
    }

    fn write(&self, b: &mut [u8]) -> Result<()> {
        self.r_offset.write(b)?;
        self.r_info.write(b)?;
        self.r_addend.write(b)?;
        Ok(())
    }

    fn size(&self) -> usize {
        self.r_offset.size() +
        self.r_info.size() +
        self.r_addend.size()
    }

}

impl Default for SymbolItem {
    fn default() -> Self {
        Self {
            st_name: Item::new(ST_NAME), 
            st_value: Item::new(ST_VALUE),
            st_size: Item::new(ST_SIZE), 
            st_info: Item::new(ST_INFO),    
            st_other: Item::new(ST_OTHER),     
            st_shndx: Item::new(ST_SHNDX),
        }
    }
}

impl Default for RelaItem {
    fn default() -> Self {
        Self {
            r_offset: Item::new(RT_OFFSET), 
            r_info: Item::new(RT_INFO),
            r_addend: Item::new(RT_ADDEND),
        }
    }
}
