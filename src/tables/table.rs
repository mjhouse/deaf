use std::marker::PhantomData;
use crate::errors::{Error,Result};
use crate::common::{ByteIter,SHType,Layout,Width};
use crate::tables::{TableItem,StringItem,RelaItem,RelItem,ArrayItem};
use crate::symbols::Symbol;
use crate::Section;

pub type Array<'a> = Table<'a,ArrayItem>;
pub type ArrayMut<'a> = TableMut<'a,ArrayItem>;

pub type SymbolTable<'a> = Table<'a,Symbol>;
pub type SymbolTableMut<'a> = TableMut<'a,Symbol>;

pub type RelTable<'a> = Table<'a,RelItem>;
pub type RelTableMut<'a> = TableMut<'a,RelItem>;

pub type RelaTable<'a> = Table<'a,RelaItem>;
pub type RelaTableMut<'a> = TableMut<'a,RelaItem>;

pub type StringTable<'a> = Table<'a,StringItem>;
pub type StringTableMut<'a> = TableMut<'a,StringItem>;

/// Shared table interface between Table and TableMut
pub trait TableView<T>
where
    T: TableItem + Default
{
    /// Get an immutable reference to the internal section
    fn section(&self) -> &Section;

    /// Get the name index of the internal section
    fn name_index(&self) -> usize {
        self.section().name_index()
    }

    /// Get an iterator over each item's binary data
    fn iterator(&self) -> ByteIter {
        ByteIter::new(
            self.section().data(),
            T::delimiter(
                self.section().entity_size()))
    }

    /// Get a slice of data that represents an item
    fn data(&self, index: usize) -> Result<&[u8]> {
        self.iterator()
            .nth(index)
            .ok_or(Error::OutOfBoundsError)
    }

    /// Get the offset of an item from the index
    fn offset(&self, index: usize) -> usize {
        if self.has_fixed_size() {
            self.section().entity_size() * index
        } else {
            self.iterator()
                .enumerate()
                .take_while(|(i,_)| i < &index)
                .fold(0,|a,(_,e)| a + e.len())
        }
    }

    /// Get an element from the table
    fn at(&self, index: usize) -> Result<T> {
        T::parse(self.data(index)?,self.section())
    }

    /// Get an element from the table at a byte offset
    fn at_offset(&self, offset: usize) -> Result<T> {
        self.iterator()
            .offset(offset)
            .next()
            .ok_or(Error::OutOfBoundsError)
            .and_then(|d| T::parse(d,self.section()))
    }

    /// Get all items from the table
    fn items(&self) -> Result<Vec<T>> {
        self.iterator()
            .map(|b| T::parse(b,self.section()))
            .collect()
    }

    /// Get the number of items in the table
    fn len(&self) -> usize {
        if self.has_fixed_size() {
            self.section().entity_count()
        } else {
            self.iterator().count()
        }
    }

    /// Get the number of bytes in the table
    fn size(&self) -> usize {
        self.section().body_size()
    }

    /// Get the layout being used by this table
    fn layout(&self) -> Layout {
        self.section().layout()
    }

    /// Get the width being used by this table
    fn width(&self) -> Width {
        self.section().width()
    }

    /// True if items are all the same size
    fn has_fixed_size(&self) -> bool {
        self.section().entity_size() > 0
    }

    /// True if items can be different sizes
    fn has_variable_size(&self) -> bool {
        !self.has_fixed_size()
    }

}

/// A Section represented as an immutable Table
pub struct Table<'a,T>
where
    T: TableItem + Default
{
    item: PhantomData<T>,
    section: &'a Section
}

/// A Section represented as a mutable Table
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
            section,
        }
    }
}

impl<'a,T> TableView<T> for Table<'a,T>
where
    T: TableItem + Default
{
    fn section(&self) -> &Section {
        self.section
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

    /// Reserve space at an offset in the section
    fn reserve(&mut self, offset: usize, size: usize) {
        let length = self.size() + size;

        self.section
            .data_mut()
            .splice(offset..offset,(0..size).map(|_| 0));

        self.section
            .set_body_size(length);
    }

    /// Discard space at an offset in the section
    fn discard(&mut self, offset: usize, size: usize) {
        let length = self.size() - size;

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
    pub fn insert(&mut self, index: usize, mut item: T) -> Result<usize> {
        item.set_layout(self.layout());
        item.set_width(self.width());

        let size   = item.size();
        let offset = self.offset(index);

        // reserve additional space
        self.reserve(offset,size);

        // get a constrained, mutable slice of bytes
        let data = self.section.slice_mut(offset,size)?;

        // write the item to the byte slice
        item.write(data)?;

        Ok(size)
    }

    /// Remove an item from the table by index
    pub fn remove(&mut self, index: usize) -> Result<T> {
        let data   = self.data(index)?;
        let offset = self.offset(index);
        let size   = data.len();
        
        let item = T::parse(data,&self.section)?;

        // remove the data from the buffer
        self.discard(offset,size);

        Ok(item)
    }

}

impl<'a,T> TableView<T> for TableMut<'a,T>
where
    T: TableItem + Default
{
    fn section(&self) -> &Section {
        self.section
    }
}

impl<'a> TryFrom<&'a Section> for Table<'a, Symbol> 
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

impl<'a> TryFrom<&'a mut Section> for TableMut<'a, Symbol>
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

impl<'a> TryFrom<&'a Section> for Table<'a, ArrayItem> 
{
    type Error = Error;

    fn try_from(section: &'a Section) -> Result<Self> {
        match section.header().kind() {
            SHType::SHT_INIT_ARRAY | SHType::SHT_PREINIT_ARRAY | SHType::SHT_FINI_ARRAY  => Ok(Self::new(section)),
            _ => Err(Error::WrongSectionError)
        }
    }
}

impl<'a> TryFrom<&'a mut Section> for TableMut<'a, ArrayItem>
{
    type Error = Error;

    fn try_from(section: &'a mut Section) -> Result<Self> {
        match section.header().kind() {
            SHType::SHT_INIT_ARRAY | SHType::SHT_PREINIT_ARRAY | SHType::SHT_FINI_ARRAY  => Ok(Self::new(section)),
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
    use crate::headers::FileHeader;
    use crate::utilities::read;

    use crate::utilities::tests::{
        LIBJPEG_DYNSYM as SYM_TEST,
        LIBVPF_SHSTRTAB as STR_TEST,
        LIBVPF_RELA_DYN as RELA_TEST,
        LIBQSCINTILLA_FINI_ARRAY as FINI_TEST,
        LIBQSCINTILLA_INIT_ARRAY as INIT_TEST, 
    };

    macro_rules! section {
        ( $path: expr, $index: expr ) => {
            read($path)
                .and_then(|d| FileHeader::parse(&d)
                .and_then(|h| Ok((d,h))))
                .and_then(|(d,h)|
                    Section::read_all(
                        &d,
                        h.shnum(),
                        h.shoff(),
                        h.shentsize(),
                        h.data(),
                        h.class()
                    )
                )
                .and_then(|s| s
                    .get($index)
                    .ok_or(Error::NotFound)
                    .cloned())
                .expect("Section not found")
        };
    }

    #[test]
    fn test_read_symbols_as_table() {
        let section = section!("assets/libjpeg/libjpeg.so.9", SYM_TEST.index);

        let result = SymbolTable::try_from(&section);
        assert!(result.is_ok());

        let table = result.unwrap();
        assert_eq!(table.len(),SYM_TEST.length);
    }

    #[test]
    fn test_read_strings_as_table() {
        let section = section!("assets/libvpf/libvpf.so.4.1", STR_TEST.index);

        let result = StringTable::try_from(&section);
        assert!(result.is_ok());

        let table = result.unwrap();
        assert_eq!(table.len(),STR_TEST.length);
    }

    #[test]
    fn test_read_relocations_addend_as_table() {
        let section = section!("assets/libvpf/libvpf.so.4.1", RELA_TEST.index);

        let result = RelaTable::try_from(&section);
        assert!(result.is_ok());

        let table = result.unwrap();
        assert_eq!(table.len(),RELA_TEST.length);
    }

    #[test]
    fn test_write_strings_prepend() {
        let mut section = section!("assets/libvpf/libvpf.so.4.1", STR_TEST.index);

        let result = StringTableMut::try_from(&mut section);
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
    fn test_write_strings_append() {
        let mut section = section!("assets/libvpf/libvpf.so.4.1", STR_TEST.index);

        let result = StringTableMut::try_from(&mut section);
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
    fn test_write_strings_insert() {
        let mut section = section!("assets/libvpf/libvpf.so.4.1", STR_TEST.index);

        let result = StringTableMut::try_from(&mut section);
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
    fn test_write_strings_remove() {
        let mut section = section!("assets/libvpf/libvpf.so.4.1", STR_TEST.index);

        let result = StringTableMut::try_from(&mut section);
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

    #[test]
    fn test_write_symbols_prepend() {
        let mut section = section!("assets/libjpeg/libjpeg.so.9", SYM_TEST.index);

        let result = SymbolTableMut::try_from(&mut section);
        assert!(result.is_ok());

        let mut table = result.unwrap();

        assert_eq!(table.len(),SYM_TEST.length);
        assert_eq!(table.size(),SYM_TEST.size);

        let item1 = Symbol::default();

        let result = table.prepend(item1.clone());
        assert!(result.is_ok());

        assert_eq!(table.len(),SYM_TEST.length + 1);
        assert_eq!(table.size(),SYM_TEST.size + SYM_TEST.entsize);

        let result = table.at(0);
        assert!(result.is_ok());

        let item2 = result.unwrap();
        assert_eq!(item2.name(),item1.name());
        assert_eq!(item2.value(),item1.value());
        assert_eq!(item2.size(),item1.size());
        assert_eq!(item2.other(),item1.other());
        assert_eq!(item2.shndx(),item1.shndx());
    }

    #[test]
    fn test_write_symbols_append() {
        let mut section = section!("assets/libjpeg/libjpeg.so.9", SYM_TEST.index);

        let result = SymbolTableMut::try_from(&mut section);
        assert!(result.is_ok());

        let mut table = result.unwrap();

        assert_eq!(table.len(),SYM_TEST.length);
        assert_eq!(table.size(),SYM_TEST.size);

        let item1 = Symbol::default();

        let result = table.append(item1.clone());
        assert!(result.is_ok());

        assert_eq!(table.len(),SYM_TEST.length + 1);
        assert_eq!(table.size(),SYM_TEST.size + SYM_TEST.entsize);

        let result = table.at(table.len() - 1);
        assert!(result.is_ok());

        let item2 = result.unwrap();
        assert_eq!(item2.name(),item1.name());
        assert_eq!(item2.value(),item1.value());
        assert_eq!(item2.size(),item1.size());
        assert_eq!(item2.other(),item1.other());
        assert_eq!(item2.shndx(),item1.shndx());
    }

    #[test]
    fn test_write_symbols_insert() {
        let mut section = section!("assets/libjpeg/libjpeg.so.9", SYM_TEST.index);

        let result = SymbolTableMut::try_from(&mut section);
        assert!(result.is_ok());

        let mut table = result.unwrap();

        assert_eq!(table.len(),SYM_TEST.length);
        assert_eq!(table.size(),SYM_TEST.size);

        let item1 = Symbol::default();

        let result = table.insert(3,item1.clone());
        assert!(result.is_ok());

        assert_eq!(table.len(),SYM_TEST.length + 1);
        assert_eq!(table.size(),SYM_TEST.size + SYM_TEST.entsize);

        let result = table.at(3);
        assert!(result.is_ok());

        let item2 = result.unwrap();
        assert_eq!(item2.name(),item1.name());
        assert_eq!(item2.value(),item1.value());
        assert_eq!(item2.size(),item1.size());
        assert_eq!(item2.other(),item1.other());
        assert_eq!(item2.shndx(),item1.shndx());
    }

    #[test]
    fn test_write_symbols_remove() {
        let mut section = section!("assets/libjpeg/libjpeg.so.9", SYM_TEST.index);

        let result = SymbolTableMut::try_from(&mut section);
        assert!(result.is_ok());

        let mut table = result.unwrap();

        assert_eq!(table.len(),SYM_TEST.length);
        assert_eq!(table.size(),SYM_TEST.size);

        let result = table.remove(3);
        assert!(result.is_ok());

        assert_eq!(table.len(),SYM_TEST.length - 1);
        assert_eq!(table.size(),SYM_TEST.size - SYM_TEST.entsize);
    }

    #[test]
    fn test_write_relocations_addend_prepend() {
        let mut section = section!("assets/libvpf/libvpf.so.4.1", RELA_TEST.index);

        let result = RelaTableMut::try_from(&mut section);
        assert!(result.is_ok());

        let mut table = result.unwrap();

        assert_eq!(table.len(),RELA_TEST.length);
        assert_eq!(table.size(),RELA_TEST.size);

        let item1 = RelaItem::default();

        let result = table.prepend(item1.clone());
        assert!(result.is_ok());

        assert_eq!(table.len(),RELA_TEST.length + 1);
        assert_eq!(table.size(),RELA_TEST.size + RELA_TEST.entsize);

        let result = table.at(0);
        assert!(result.is_ok());

        let item2 = result.unwrap();
        assert_eq!(item2.offset(),item1.offset());
        assert_eq!(item2.info(),item1.info());
    }

    #[test]
    fn test_write_relocations_addend_append() {
        let mut section = section!("assets/libvpf/libvpf.so.4.1", RELA_TEST.index);

        let result = RelaTableMut::try_from(&mut section);
        assert!(result.is_ok());

        let mut table = result.unwrap();

        assert_eq!(table.len(),RELA_TEST.length);
        assert_eq!(table.size(),RELA_TEST.size);

        let item1 = RelaItem::default();

        let result = table.append(item1.clone());
        assert!(result.is_ok());

        assert_eq!(table.len(),RELA_TEST.length + 1);
        assert_eq!(table.size(),RELA_TEST.size + RELA_TEST.entsize);

        let result = table.at(table.len() - 1);
        assert!(result.is_ok());

        let item2 = result.unwrap();
        assert_eq!(item2.offset(),item1.offset());
        assert_eq!(item2.info(),item1.info());
    }

    #[test]
    fn test_write_relocations_addend_insert() {
        let mut section = section!("assets/libvpf/libvpf.so.4.1", RELA_TEST.index);

        let result = RelaTableMut::try_from(&mut section);
        assert!(result.is_ok());

        let mut table = result.unwrap();

        assert_eq!(table.len(),RELA_TEST.length);
        assert_eq!(table.size(),RELA_TEST.size);

        let item1 = RelaItem::default();

        let result = table.insert(3,item1.clone());
        assert!(result.is_ok());

        assert_eq!(table.len(),RELA_TEST.length + 1);
        assert_eq!(table.size(),RELA_TEST.size + RELA_TEST.entsize);

        let result = table.at(3);
        assert!(result.is_ok());

        let item2 = result.unwrap();
        assert_eq!(item2.offset(),item1.offset());
        assert_eq!(item2.info(),item1.info());
    }

    #[test]
    fn test_write_relocations_addend_remove() {
        let mut section = section!("assets/libvpf/libvpf.so.4.1", RELA_TEST.index);

        let result = RelaTableMut::try_from(&mut section);
        assert!(result.is_ok());

        let mut table = result.unwrap();

        assert_eq!(table.len(),RELA_TEST.length);
        assert_eq!(table.size(),RELA_TEST.size);

        let result = table.remove(3);
        assert!(result.is_ok());

        assert_eq!(table.len(),RELA_TEST.length - 1);
        assert_eq!(table.size(),RELA_TEST.size - RELA_TEST.entsize);
    }

    #[test]
    fn test_read_init_array_as_array() {
        let section = section!("assets/libqscintilla2/libqscintilla2_qt5.so.15.0.0", INIT_TEST.index);

        let result = Array::try_from(&section);
        assert!(result.is_ok());

        let array = result.unwrap();
        assert_eq!(array.len(),INIT_TEST.length);
    }

    #[test]
    fn test_read_fini_array_as_array() {
        let section = section!("assets/libqscintilla2/libqscintilla2_qt5.so.15.0.0", FINI_TEST.index);

        let result = Array::try_from(&section);
        assert!(result.is_ok());

        let array = result.unwrap();
        assert_eq!(array.len(),FINI_TEST.length);
    }

    #[test]
    fn test_write_init_array_append() {
        let mut section = section!("assets/libqscintilla2/libqscintilla2_qt5.so.15.0.0", INIT_TEST.index);

        let result = ArrayMut::try_from(&mut section);
        assert!(result.is_ok());

        let mut array = result.unwrap();

        assert_eq!(array.len(),INIT_TEST.length);
        assert_eq!(array.size(),INIT_TEST.size);

        let result = array.append(123.into());
        assert!(result.is_ok());

        assert_eq!(array.len(),INIT_TEST.length + 1);
        assert_eq!(array.size(),INIT_TEST.size + INIT_TEST.entsize);

        let result = array.at(array.len() - 1);
        assert!(result.is_ok());

        let item = result.unwrap();
        assert_eq!(item.value(),123);
    }

    #[test]
    fn test_write_fini_array_append() {
        let mut section = section!("assets/libqscintilla2/libqscintilla2_qt5.so.15.0.0", FINI_TEST.index);

        let result = ArrayMut::try_from(&mut section);
        assert!(result.is_ok());

        let mut array = result.unwrap();

        assert_eq!(array.len(),FINI_TEST.length);
        assert_eq!(array.size(),FINI_TEST.size);

        let result = array.append(123.into());
        assert!(result.is_ok());

        assert_eq!(array.len(),FINI_TEST.length + 1);
        assert_eq!(array.size(),FINI_TEST.size + FINI_TEST.entsize);

        let result = array.at(array.len() - 1);
        assert!(result.is_ok());

        let item = result.unwrap();
        assert_eq!(item.value(),123);
    }

    #[test]
    fn test_write_init_array_prepend() {
        let mut section = section!("assets/libqscintilla2/libqscintilla2_qt5.so.15.0.0", INIT_TEST.index);

        let result = ArrayMut::try_from(&mut section);
        assert!(result.is_ok());

        let mut array = result.unwrap();

        assert_eq!(array.len(),INIT_TEST.length);
        assert_eq!(array.size(),INIT_TEST.size);

        let result = array.prepend(123.into());
        assert!(result.is_ok());

        assert_eq!(array.len(),INIT_TEST.length + 1);
        assert_eq!(array.size(),INIT_TEST.size + INIT_TEST.entsize);

        let result = array.at(0);
        assert!(result.is_ok());

        let item = result.unwrap();
        assert_eq!(item.value(),123);
    }

    #[test]
    fn test_write_fini_array_prepend() {
        let mut section = section!("assets/libqscintilla2/libqscintilla2_qt5.so.15.0.0", FINI_TEST.index);

        let result = ArrayMut::try_from(&mut section);
        assert!(result.is_ok());

        let mut array = result.unwrap();

        assert_eq!(array.len(),FINI_TEST.length);
        assert_eq!(array.size(),FINI_TEST.size);

        let result = array.prepend(123.into());
        assert!(result.is_ok());

        assert_eq!(array.len(),FINI_TEST.length + 1);
        assert_eq!(array.size(),FINI_TEST.size + FINI_TEST.entsize);

        let result = array.at(0);
        assert!(result.is_ok());

        let item = result.unwrap();
        assert_eq!(item.value(),123);
    }

    #[test]
    fn test_write_init_array_insert() {
        let mut section = section!("assets/libqscintilla2/libqscintilla2_qt5.so.15.0.0", INIT_TEST.index);

        let result = ArrayMut::try_from(&mut section);
        assert!(result.is_ok());

        let mut array = result.unwrap();

        assert_eq!(array.len(),INIT_TEST.length);
        assert_eq!(array.size(),INIT_TEST.size);

        let result = array.insert(3,123.into());
        assert!(result.is_ok());

        assert_eq!(array.len(),INIT_TEST.length + 1);
        assert_eq!(array.size(),INIT_TEST.size + INIT_TEST.entsize);

        let result = array.at(3);
        assert!(result.is_ok());

        let item = result.unwrap();
        assert_eq!(item.value(),123);
    }

    #[test]
    fn test_write_fini_array_insert() {
        let mut section = section!("assets/libqscintilla2/libqscintilla2_qt5.so.15.0.0", FINI_TEST.index);

        let result = ArrayMut::try_from(&mut section);
        assert!(result.is_ok());

        let mut array = result.unwrap();

        assert_eq!(array.len(),FINI_TEST.length);
        assert_eq!(array.size(),FINI_TEST.size);

        let result = array.insert(1,123.into());
        assert!(result.is_ok());

        assert_eq!(array.len(),FINI_TEST.length + 1);
        assert_eq!(array.size(),FINI_TEST.size + FINI_TEST.entsize);

        let result = array.at(1);
        assert!(result.is_ok());

        let item = result.unwrap();
        assert_eq!(item.value(),123);
    }

}