use crate::headers::common::constants::{Width,Layout};
use crate::common::bytes::{IntoBytes,FromBytes};
use crate::tables::common::ByteDelimiter;
use crate::tables::RelocationInfo;
use crate::tables::SymbolInfo;
use crate::common::ranges::*;
use crate::errors::Result;
use crate::common::Item;
use std::ffi::CString;

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

#[derive(Default,Clone)]
pub struct StringItem {
    value: CString,
}

#[derive(Clone)]
pub struct SymbolItem {
    st_name: Item<u32,u32>,
    st_value: Item<u32,u64>,
    st_size: Item<u32,u64>,
    st_info: Item<u8,u8,SymbolInfo>,
    st_other: Item<u8>,
    st_shndx: Item<u16,u16>,
}

#[derive(Clone)]
pub struct RelocationItem {
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

    pub fn name(&self) -> Option<u32> {
        self.st_name.get()
    }

    pub fn name_unchecked(&self) -> u32 {
        self.st_name.get().expect("No name")
    }

    pub fn set_name(&mut self, value: u32) {
        self.st_name.set(value);
    }

    pub fn value(&self) -> Option<u64> {
        self.st_value.get()
    }

    pub fn value_unchecked(&self) -> u64 {
        self.st_value.get().expect("No value")
    }

    pub fn set_value(&mut self, value: u64) {
        self.st_value.set(value);
    }

    pub fn size(&self) -> Option<u64> {
        self.st_size.get()
    }

    pub fn size_unchecked(&self) -> u64 {
        self.st_size.get().expect("No size")
    }

    pub fn set_size(&mut self, value: u64) {
        self.st_size.set(value);
    }

    pub fn info(&self) -> Option<SymbolInfo> {
        self.st_info.get()
    }

    pub fn info_unchecked(&self) -> SymbolInfo {
        self.st_info.get().expect("No info")
    }

    pub fn set_info(&mut self, value: SymbolInfo) {
        self.st_info.set(value);
    }

    pub fn other(&self) -> Option<u8> {
        self.st_other.get()
    }

    pub fn other_unchecked(&self) -> u8 {
        self.st_other.get().expect("No other")
    }

    pub fn set_other(&mut self, value: u8) {
        self.st_other.set(value);
    }

    pub fn shndx(&self) -> Option<u16> {
        self.st_shndx.get()
    }

    pub fn shndx_unchecked(&self) -> u16 {
        self.st_shndx.get().expect("No shndx")
    }

    pub fn set_shndx(&mut self, value: u16) {
        self.st_shndx.set(value);
    }

}

impl RelocationItem {

    pub fn offset(&self) -> Option<u64> {
        self.r_offset.get()
    }

    pub fn set_offset(&mut self, value: u64) {
        self.r_offset.set(value);
    }

    pub fn offset_unchecked(&self) -> u64 {
        self.r_offset.get().expect("No offset")
    }

    pub fn info(&self) -> Option<RelocationInfo> {
        self.r_info.get()
    }

    pub fn set_info(&mut self, value: RelocationInfo) {
        self.r_info.set(value);
    }

    pub fn info_unchecked(&self) -> RelocationInfo {
        self.r_info.get().expect("No info")
    }

    pub fn addend(&self) -> Option<i64> {
        self.r_addend.get()
    }

    pub fn set_addend(&mut self, value: i64) {
        self.r_addend.set(value);
    }

    pub fn addend_unchecked(&self) -> i64 {
        self.r_addend.get().expect("No addend")
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

impl TableItem for RelocationItem {

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
        self.r_addend.read(b).ok(); // ignore failure
        Ok(())
    }

    fn write(&self, b: &mut [u8]) -> Result<()> {
        self.r_offset.write(b)?;
        self.r_info.write(b)?;
        self.r_addend.write(b).ok(); // ignore failure
        Ok(())
    }

    fn size(&self) -> usize {
        self.r_offset.size() +
        self.r_info.size() +
        self.r_addend.size() // size is 0 if None
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

impl Default for RelocationItem {
    fn default() -> Self {
        Self {
            r_offset: Item::new(RT_OFFSET), 
            r_info: Item::new(RT_INFO),
            r_addend: Item::new(RT_ADDEND),
        }
    }
}
