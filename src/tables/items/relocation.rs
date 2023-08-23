use crate::common::{Width,Layout,Item,ranges::*};
use crate::tables::info::RelocationInfo;
use crate::tables::TableItem;
use crate::errors::Result;

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

impl RelItem {

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

impl TableItem for RelItem {

    fn set_layout(&mut self, layout: Layout){
        self.r_offset.set_layout(layout);
        self.r_info.set_layout(layout);
    }

    fn set_width(&mut self, width: Width){
        self.r_offset.set_width(width);
        self.r_info.set_width(width);
    }

    fn read(&mut self, b: &[u8]) -> Result<()> {
        self.r_offset.read(b)?;
        self.r_info.read(b)?;
        Ok(())
    }

    fn write(&self, b: &mut [u8]) -> Result<()> {
        self.r_offset.write(b)?;
        self.r_info.write(b)?;
        Ok(())
    }

    fn size(&self) -> usize {
        self.r_offset.size() +
        self.r_info.size()
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

impl Default for RelItem {
    fn default() -> Self {
        Self {
            r_offset: Item::new(RT_OFFSET), 
            r_info: Item::new(RT_INFO),
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