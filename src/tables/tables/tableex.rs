use std::marker::PhantomData;

use crate::errors::{Error,Result};
use crate::common::{ByteIter,SHType};
use crate::tables::TableItem;
use crate::headers::SectionHeader;
use crate::tables::{StringItem,RelaItem,RelItem,SymbolItem};
use crate::Section;

pub struct Table<'a,T>
where
    T: TableItem + Default
{
    item: PhantomData<T>,
    section: &'a Section
}

pub struct TableMut<'a,T>
where
    T: TableItem + Default
{
    item: PhantomData<T>,
    section: &'a mut Section
}

impl<'a,T> Table<'a,T>
where
    T: TableItem + Default
{

    /// Create a new table from the given section
    fn new(section: &'a Section) -> Self {
        Self {
            item: PhantomData {},
            section: section,
        }
    }

    /// Get an iterator over the data
    fn iterator(&self) -> ByteIter {
        ByteIter::new(
            self.section.data(),
            T::delimiter(
                self.item_size()))
    }

    /// Get a slice of data that represents an item
    fn item_data(&self, index: usize) -> Result<&[u8]> {
        self.iterator()
            .nth(index)
            .ok_or(Error::OutOfBoundsError)
    }

    /// Get the offset of an item from the index
    fn item_offset(&self, index: usize) -> usize {
        self.item_size() * index
    }

    /// Get the size of the table item
    fn item_size(&self) -> usize {
        self.section.entity_size()
    }

    /// Get the number of items in the table
    fn item_count(&self) -> usize {
        if self.item_size() > 0 {
            self.section.entity_count()
        } else {
            self.iterator().count()
        }
    }

    /// Read from a fitted slice, returning the item
    fn get(&self, index: usize) -> Result<T> {
        let data = self.item_data(index)?;
        let mut item = T::default();

        // set expected layout and width from table
        item.set_layout(self.section.layout());
        item.set_width(self.section.width());

        item.read(data)?;
        Ok(item)
    }

    /// Get all items from the table
    fn items(&self) -> Result<Vec<T>> {
        // TODO: make this use ByteIter instead of calling get
        (0..self.item_count())
            .into_iter()
            .map(|i| self.get(i))
            .collect()
    }

    /// Get the number of items in the table
    pub fn len(&self) -> usize {
        self.item_count()
    }
}

impl<'a,T> TableMut<'a,T>
where
    T: TableItem + Default
{

    /// Create a new mutable table for the given section
    fn new(section: &'a mut Section) -> Self {
        Self {
            item: PhantomData {},
            section: section,
        }
    }

    /// Get an iterator over the data
    fn iterator(&self) -> ByteIter {
        ByteIter::new(
            self.section.data(),
            T::delimiter(
                self.item_size()))
    }

    /// Get a slice of data that represents an item
    fn item_data(&self, index: usize) -> Result<&[u8]> {
        self.iterator()
            .nth(index)
            .ok_or(Error::OutOfBoundsError)
    }

    // /// Get a slice of data that represents an item
    // fn item_data_mut(&self, index: usize) -> Result<&[u8]> {
    //     self.iterator()
    //         .nth(index)
    //         .ok_or(Error::OutOfBoundsError)
    // }

    /// Get the offset of an item from the index
    fn item_offset(&self, index: usize) -> usize {
        self.item_size() * index
    }

    /// Get the total size of the table
    fn table_size(&self) -> usize {
        self.section.body_size()
    }

    /// Get the size of the table item
    fn item_size(&self) -> usize {
        self.section.entity_size()
    }

    /// Get the number of items in the table
    fn item_count(&self) -> usize {
        if self.item_size() > 0 {
            self.section.entity_count()
        } else {
            self.iterator().count()
        }
    }

    /// Reserve bytes at the end of the buffer
    fn reserve_end(&mut self, size: usize) {
        let limit = self.table_size();

        // reserve additional space
        self.section
            .data_mut()
            .reserve(size);

        // update the section size
        self.section
            .set_body_size(limit + size);
    }

    /// Reserve bytes at the start of the buffer
    fn reserve_start(&mut self, size: usize) {
        // reserve additional space
        self.reserve_end(size);

        // rotate that space to the front
        self.section
            .data_mut()
            .rotate_right(size);
    }

    /// Read from a fitted slice, returning the item
    pub fn get(&self, index: usize) -> Result<T> {
        let data = self.item_data(index)?;
        let mut item = T::default();

        // set expected layout and width from table
        item.set_layout(self.section.layout());
        item.set_width(self.section.width());

        item.read(data)?;
        Ok(item)
    }

    /// Write to a fitted slice, returning the number of bytes written
    pub fn set(&mut self, index: usize, item: T) -> Result<usize> {
        let size  = self.item_size();
        let start = self.item_offset(index);
        let end   = start + size;
        let data  = self.section.data_mut();

        // get a constrained, mutable slice of bytes
        let buffer = &mut data[start..end];

        // write the item to the byte slice
        item.write(buffer)?;
        Ok(size)
    }

    /// Append an item to the table
    pub fn append(&mut self, item: T) -> Result<usize> {
        let size   = self.item_size();
        let offset = self.table_size();

        // reserve additional space
        self.reserve_end(size);

        // write to that position
        self.set(offset,item)
    }

    /// Prepend an item to the table
    pub fn prepend(&mut self, item: T) -> Result<usize> {
        let size   = self.item_size();
        let offset = 0;

        // reserve additional space
        self.reserve_start(size);

        // write to that position
        self.set(offset,item)
    }

    /// Remove an item from the table by index
    pub fn remove(&mut self, index: usize) -> Result<T> {
        let size  = self.item_size();
        let start = self.item_offset(index);
        
        // get the item from the buffer
        let item = self.get(index)?;

        // remove the data from the buffer
        self.section
            .data_mut()
            .drain(start..start + size);

        Ok(item)
    }

    /// Get all items from the table
    pub fn items(&self) -> Result<Vec<T>> {
        (0..self.item_count())
            .into_iter()
            .map(|i| self.get(i))
            .collect()
    }

    /// Get the number of items in the table
    pub fn len(&self) -> usize {
        self.item_count()
    }
}

impl<'a> TryFrom<&'a Section> for Table<'a, SymbolItem> 
{
    type Error = Error;

    fn try_from(section: &'a Section) -> Result<Self> {
        match section.header().kind() {
            SHType::SHT_SYMTAB => Ok(Self::new(section)),
            SHType::SHT_DYNSYM => Ok(Self::new(section)),
            _ => Err(Error::WrongSectionError)
        }
    }
}

impl<'a> TryFrom<&'a mut Section> for TableMut<'a, SymbolItem>
{
    type Error = Error;

    fn try_from(section: &'a mut Section) -> Result<Self> {
        match section.header().kind() {
            SHType::SHT_SYMTAB => Ok(Self::new(section)),
            SHType::SHT_DYNSYM => Ok(Self::new(section)),
            _ => Err(Error::WrongSectionError)
        }
    }
}

impl<'a> TryFrom<&'a Section> for Table<'a, StringItem> 
{
    type Error = Error;

    fn try_from(section: &'a Section) -> Result<Self> {
        match section.header().kind() {
            SHType::SHT_STRTAB => Ok(Self::new(section)),
            _ => Err(Error::WrongSectionError)
        }
    }
}

impl<'a> TryFrom<&'a mut Section> for TableMut<'a, StringItem>
{
    type Error = Error;

    fn try_from(section: &'a mut Section) -> Result<Self> {
        match section.header().kind() {
            SHType::SHT_STRTAB => Ok(Self::new(section)),
            _ => Err(Error::WrongSectionError)
        }
    }
}

impl<'a> TryFrom<&'a Section> for Table<'a, RelaItem> 
{
    type Error = Error;

    fn try_from(section: &'a Section) -> Result<Self> {
        match section.header().kind() {
            SHType::SHT_RELA => Ok(Self::new(section)),
            _ => Err(Error::WrongSectionError)
        }
    }
}

impl<'a> TryFrom<&'a mut Section> for TableMut<'a, RelaItem>
{
    type Error = Error;

    fn try_from(section: &'a mut Section) -> Result<Self> {
        match section.header().kind() {
            SHType::SHT_RELA => Ok(Self::new(section)),
            _ => Err(Error::WrongSectionError)
        }
    }
}

impl<'a> TryFrom<&'a Section> for Table<'a, RelItem> 
{
    type Error = Error;

    fn try_from(section: &'a Section) -> Result<Self> {
        match section.header().kind() {
            SHType::SHT_REL => Ok(Self::new(section)),
            _ => Err(Error::WrongSectionError)
        }
    }
}

impl<'a> TryFrom<&'a mut Section> for TableMut<'a, RelItem>
{
    type Error = Error;

    fn try_from(section: &'a mut Section) -> Result<Self> {
        match section.header().kind() {
            SHType::SHT_REL => Ok(Self::new(section)),
            _ => Err(Error::WrongSectionError)
        }
    }
}

impl<'a,T> From<TableMut<'a,T>> for Table<'a,T>
where
    T: TableItem + Default
{
    fn from(table: TableMut<'a,T>) -> Self {
        Self::new(table.section)
    }
}

impl<'a,T> From<TableMut<'a,T>> for &'a mut Section
where
    T: TableItem + Default
{
    fn from(table: TableMut<'a,T>) -> Self {
        table.section
    }
}

impl<'a,T> From<Table<'a,T>> for &'a Section
where
    T: TableItem + Default
{
    fn from(table: Table<'a,T>) -> Self {
        table.section
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::headers::{FileHeader,SectionHeader,SectionHeaderData};
    use crate::common::{Width,Layout,SHType,SectionType};
    use crate::utilities::read;

    use crate::utilities::tests::{
        LIBJPEG_DYNSYM as SYM_TEST,
        LIBVPF_SHSTRTAB as STR_TEST,
        LIBVPF_RELA_DYN as RELA_TEST,
    };

    #[test]
    fn test_symtab_section_as_table() {
        let data = read("assets/libjpeg/libjpeg.so.9").unwrap();

        let file_header = FileHeader::parse(&data).unwrap();

        let count = file_header.shnum();
        let offset = file_header.shoff();
        let size = file_header.shentsize();
        let layout = file_header.data();
        let width = file_header.class();

        let sections = Section::read_all(
            &data,
            count,
            offset,
            size,
            layout,
            width
        ).unwrap();
        
        let section = sections
            .iter()
            .find(|&h| h.kind() == SectionType::DynamicSymbols)
            .unwrap();

        let result = Table::<SymbolItem>::try_from(section);
        assert!(result.is_ok());

        let table = result.unwrap();
        assert_eq!(table.len(),SYM_TEST.length);
    }

    #[test]
    fn test_shstrtab_section_as_table() {
        let data = read("assets/libvpf/libvpf.so.4.1").unwrap();

        let file_header = FileHeader::parse(&data).unwrap();

        let count = file_header.shnum();
        let offset = file_header.shoff();
        let index = file_header.shstrndx();
        let size = file_header.shentsize();
        let layout = file_header.data();
        let width = file_header.class();

        let sections = Section::read_all(
            &data,
            count,
            offset,
            size,
            layout,
            width
        ).unwrap();
        
        let section = &sections[index];

        let result = Table::<StringItem>::try_from(section);
        assert!(result.is_ok());

        let table = result.unwrap();
        assert_eq!(table.len(),STR_TEST.length);
    }

    #[test]
    fn test_rela_section_as_table() {
        let data = read("assets/libvpf/libvpf.so.4.1").unwrap();

        let file_header = FileHeader::parse(&data).unwrap();

        let count = file_header.shnum();
        let offset = file_header.shoff();
        let size = file_header.shentsize();
        let layout = file_header.data();
        let width = file_header.class();

        let sections = Section::read_all(
            &data,
            count,
            offset,
            size,
            layout,
            width
        ).unwrap();

        let section = sections
            .iter()
            .find(|&h| h.kind() == SectionType::RelocationsAddend)
            .unwrap();

        let result = Table::<RelaItem>::try_from(section);
        assert!(result.is_ok());

        let table = result.unwrap();
        assert_eq!(table.len(),RELA_TEST.length);
    }

}