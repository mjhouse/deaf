use std::marker::PhantomData;

use crate::errors::{Error,Result};
use crate::common::{ByteIter,SHType};
use crate::tables::{TableItem,StringItem,RelaItem,RelItem,SymbolItem};
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
                self.section.entity_size()))
    }

    /// Get a slice of data that represents an item
    fn item_data(&self, index: usize) -> Result<&[u8]> {
        self.iterator()
            .nth(index)
            .ok_or(Error::OutOfBoundsError)
    }

    /// True if items are all the same size
    pub fn has_fixed_size(&self) -> bool {
        self.section.entity_size() > 0
    }

    /// True if items can be different sizes
    pub fn has_variable_size(&self) -> bool {
        !self.has_fixed_size()
    }

    /// Get an element from the table
    pub fn at(&self, index: usize) -> Result<T> {
        T::parse(self.item_data(index)?,&self.section)
    }

    /// Get all items from the table
    pub fn items(&self) -> Result<Vec<T>> {
        self.iterator()
            .map(|b| T::parse(b,&self.section))
            .collect()
    }

    /// Get the number of items in the table
    pub fn len(&self) -> usize {
        if self.has_fixed_size() {
            self.section.entity_count()
        } else {
            self.iterator().count()
        }
    }

    /// Get the number of bytes in the table
    pub fn size(&self) -> usize {
        self.section.body_size()
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
                self.section.entity_size()))
    }

    /// Get a slice of data that represents an item
    fn item_data(&self, index: usize) -> Result<&[u8]> {
        self.iterator()
            .nth(index)
            .ok_or(Error::OutOfBoundsError)
    }

    /// Get the offset of an item from the index
    fn item_offset(&self, index: usize) -> usize {
        if self.has_fixed_size() {
            self.section.entity_size() * index
        } else {
            self.iterator()
                .enumerate()
                .take_while(|(i,_)| i < &index)
                .fold(0,|a,(_,e)| a + e.len())
        }
    }

    /// Get the total size of the table
    fn table_size(&self) -> usize {
        self.section.body_size()
    }

    /// Get the number of items in the table
    fn item_count(&self) -> usize {
        if self.has_fixed_size() {
            self.section.entity_count()
        } else {
            self.iterator().count()
        }
    }

    /// Reserve space at an offset in the section
    fn reserve(&mut self, offset: usize, size: usize) {
        let length = self.table_size() + size;

        self.section
            .data_mut()
            .splice(offset..offset,(0..size).map(|_| 0));

        self.section
            .set_body_size(length);
    }

    /// Discard space at an offset in the section
    fn discard(&mut self, offset: usize, size: usize) {
        let length = self.table_size() - size;

        self.section
            .data_mut()
            .drain(offset..offset + size);

        self.section
            .set_body_size(length);
    }

    /// Append an item to the table
    pub fn append(&mut self, item: T) -> Result<usize> {
        self.insert(self.len(),item)
    }

    /// Prepend an item to the table
    pub fn prepend(&mut self, item: T) -> Result<usize> {
        self.insert(0,item)
    }

    /// Insert an item into the table
    pub fn insert(&mut self, index: usize, item: T) -> Result<usize> {
        let size   = item.size();
        let offset = self.item_offset(index);

        // reserve additional space
        self.reserve(offset,size);

        // get a constrained, mutable slice of bytes
        let data = self.section.slice_mut(offset,size);

        // write the item to the byte slice
        item.write(data)?;

        Ok(size)
    }

    /// Remove an item from the table by index
    pub fn remove(&mut self, index: usize) -> Result<T> {
        let data   = self.item_data(index)?;
        let offset = self.item_offset(index);
        let size   = data.len();
        
        let item = T::parse(data,&self.section)?;

        // remove the data from the buffer
        self.discard(offset,size);

        Ok(item)
    }

    /// Get an element from the table
    pub fn at(&self, index: usize) -> Result<T> {
        T::parse(self.item_data(index)?,&self.section)
    }

    /// Get all items from the table
    pub fn items(&self) -> Result<Vec<T>> {
        self.iterator()
            .map(|b| T::parse(b,&self.section))
            .collect()
    }

    /// True if items are all the same size
    pub fn has_fixed_size(&self) -> bool {
        self.section.entity_size() > 0
    }

    /// True if items can be different sizes
    pub fn has_variable_size(&self) -> bool {
        !self.has_fixed_size()
    }

    /// Get the number of items in the table
    pub fn len(&self) -> usize {
        self.item_count()
    }

    /// Get the number of bytes in the table
    pub fn size(&self) -> usize {
        self.table_size()
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
    use crate::headers::{FileHeader};
    use crate::common::{SectionType};
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

    #[test]
    fn test_write_shstrtab_prepend() {
        let data = read("assets/libvpf/libvpf.so.4.1").unwrap();

        let file_header = FileHeader::parse(&data).unwrap();

        let count = file_header.shnum();
        let offset = file_header.shoff();
        let index = file_header.shstrndx();
        let size = file_header.shentsize();
        let layout = file_header.data();
        let width = file_header.class();

        let mut sections = Section::read_all(
            &data,
            count,
            offset,
            size,
            layout,
            width
        ).unwrap();
        
        let section = &mut sections[index];

        let result = TableMut::<StringItem>::try_from(section);
        assert!(result.is_ok());

        let mut table = result.unwrap();

        assert_eq!(table.len(),STR_TEST.length);
        assert_eq!(table.size(),STR_TEST.size);

        let result = table.prepend("TEST".try_into().unwrap());
        assert!(result.is_ok());

        assert_eq!(table.len(),STR_TEST.length + 1);
        assert_eq!(table.size(),STR_TEST.size + 5);

        let result = table.at(0);
        assert!(result.is_ok());

        let item = result.unwrap();

        let result = item.string();
        assert!(result.is_ok());

        let value = result.unwrap();
        assert_eq!(value,"TEST".to_string());
    }

    #[test]
    fn test_write_shstrtab_append() {
        let data = read("assets/libvpf/libvpf.so.4.1").unwrap();

        let file_header = FileHeader::parse(&data).unwrap();

        let count = file_header.shnum();
        let offset = file_header.shoff();
        let index = file_header.shstrndx();
        let size = file_header.shentsize();
        let layout = file_header.data();
        let width = file_header.class();

        let mut sections = Section::read_all(
            &data,
            count,
            offset,
            size,
            layout,
            width
        ).unwrap();
        
        let section = &mut sections[index];

        let result = TableMut::<StringItem>::try_from(section);
        assert!(result.is_ok());

        let mut table = result.unwrap();
        
        assert_eq!(table.len(),STR_TEST.length);
        assert_eq!(table.size(),STR_TEST.size);

        let result = table.append("TEST".try_into().unwrap());
        assert!(result.is_ok());

        assert_eq!(table.len(),STR_TEST.length + 1);
        assert_eq!(table.size(),STR_TEST.size + 5);

        let result = table.at(table.len() - 1);
        assert!(result.is_ok());

        let item = result.unwrap();

        let result = item.string();
        assert!(result.is_ok());

        let string = result.unwrap();
        let value = string.as_str();
        assert_eq!(value,"TEST");
    }

    #[test]
    fn test_write_shstrtab_insert() {
        let data = read("assets/libvpf/libvpf.so.4.1").unwrap();

        let file_header = FileHeader::parse(&data).unwrap();

        let count = file_header.shnum();
        let offset = file_header.shoff();
        let index = file_header.shstrndx();
        let size = file_header.shentsize();
        let layout = file_header.data();
        let width = file_header.class();

        let mut sections = Section::read_all(
            &data,
            count,
            offset,
            size,
            layout,
            width
        ).unwrap();
        
        let section = &mut sections[index];

        let result = TableMut::<StringItem>::try_from(section);
        assert!(result.is_ok());

        let mut table = result.unwrap();
        
        assert_eq!(table.len(),STR_TEST.length);
        assert_eq!(table.size(),STR_TEST.size);

        let result = table.insert(1,"TEST".try_into().unwrap());
        assert!(result.is_ok());

        assert_eq!(table.len(),STR_TEST.length + 1);
        assert_eq!(table.size(),STR_TEST.size + 5);

        let result = table.at(1);
        assert!(result.is_ok());

        let item = result.unwrap();

        let result = item.string();
        assert!(result.is_ok());

        let string = result.unwrap();
        let value = string.as_str();
        assert_eq!(value,"TEST");
    }

    #[test]
    fn test_write_shstrtab_remove() {
        let data = read("assets/libvpf/libvpf.so.4.1").unwrap();

        let file_header = FileHeader::parse(&data).unwrap();

        let count = file_header.shnum();
        let offset = file_header.shoff();
        let index = file_header.shstrndx();
        let size = file_header.shentsize();
        let layout = file_header.data();
        let width = file_header.class();

        let mut sections = Section::read_all(
            &data,
            count,
            offset,
            size,
            layout,
            width
        ).unwrap();
        
        let section = &mut sections[index];

        let result = TableMut::<StringItem>::try_from(section);
        assert!(result.is_ok());

        let mut table = result.unwrap();

        assert_eq!(table.len(),STR_TEST.length);
        assert_eq!(table.size(),STR_TEST.size);

        let result = table.remove(1);
        assert!(result.is_ok());

        assert_eq!(table.len(),STR_TEST.length - 1);
        assert_eq!(table.size(),STR_TEST.size - 10);

        let result = table.at(1);
        assert!(result.is_ok());

        let item = result.unwrap();

        let result = item.string();
        assert!(result.is_ok());

        let string = result.unwrap();
        let value = string.as_str();
        assert_ne!(value,".shstrtab");
    }

}