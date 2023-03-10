use crate::headers::common::constants::{Width,Layout};
use crate::errors::{Result};

use crate::tables::common::ByteDelimiter;

// stuff for String impl
use std::ffi::{CString};

// stuff for Relocation impl
use crate::tables::relocation::RelocationInfo;
use crate::common::Item;

// stuff for Symbol impl
use crate::tables::symbol::SymbolInfo;

pub trait TableItem {

    /// Create a new, empty item
    fn new() -> Self where Self: Sized;

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

#[derive(Clone)]
pub struct RelocationItem {
    r_offset: Item<u32,u64>, 
    r_info: Item<u32,u64,RelocationInfo>,
    r_addend: Item<i32,i64>,
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
pub struct StringItem {
    field: Item<CString>,
}

impl TableItem for StringItem {

    fn new() -> Self {
        unimplemented!();
    }

    // Override the default implementation
    fn delimiter(_: usize) -> ByteDelimiter {
        ByteDelimiter::Value(b'\0')
    }

    fn read(&mut self, b: &[u8]) -> Result<()> {
        self.field.read(b)?;
        Ok(())
    }

    fn write(&self, b: &mut [u8]) -> Result<()> {
        self.field.write(b)?;
        Ok(())
    }

    fn size(&self) -> usize {
        self.field
            .get()
            .map(|s| s
                .as_bytes_with_nul()
                .len())
            .unwrap_or(0)
    }

}

impl TableItem for SymbolItem {

    fn new() -> Self {
        unimplemented!();
    }

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

    fn new() -> Self {
        unimplemented!();
    }

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