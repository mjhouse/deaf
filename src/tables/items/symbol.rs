use crate::common::{
    Width,
    Layout,
    Item,
    ranges::*
};
use crate::tables::{
    SymbolInfo,
    TableItem
};
use crate::errors::Result;

/// A Symbol item found in symbol tables
#[derive(Clone,Debug)]
pub struct SymbolItem {
    st_name: Item<u32,u32>,
    st_value: Item<u32,u64>,
    st_size: Item<u32,u64>,
    st_info: Item<u8,u8,SymbolInfo>,
    st_other: Item<u8>,
    st_shndx: Item<u16,u16>,
}

#[derive(Clone,Debug)]
pub struct SymbolItemData {
    st_name: u32,
    st_value: u64,
    st_size: u64,
    st_info: SymbolInfo,
    st_other: u8,
    st_shndx: u16,
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