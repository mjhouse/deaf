use crate::errors::{Error,Result};
use crate::common::{ByteIter,Width,Layout,SHType};
use crate::headers::SectionHeader;

use crate::tables::table_item::{
    TableItem,
    StringItem,
    SymbolItem,
    RelocationItem
};

/// Alias for a Table that contains StringItem records
pub type StringTable = Table<StringItem>;

/// Alias for a Table that contains SymbolItem records
pub type SymbolTable = Table<SymbolItem>;

/// Alias for a Table that contains RelocationItem records
pub type RelocationTable = Table<RelocationItem>;

/// A section interpreted as a table
///
/// Each Table instance is considered to be 
/// a series of records of type `T`. Tables can
/// be parsed from a SectionHeader and a byte
/// buffer containing the body of the section.
pub struct Table<T>
where
    T: TableItem + Default
{
    table_offset: usize,
    table_size: usize,
    item_size: usize,
    layout: Layout,
    width: Width,
    items: Vec<T>
}

impl<T> Table<T>
where
    T: TableItem + Default
{

    /// Create a new table from section information taken from
    /// a section header.
    pub fn new(table_offset: usize, table_size: usize, item_size: usize, layout: Layout, width: Width) -> Self {
        Self {
            table_offset,
            table_size,
            item_size,
            layout,
            width,
            items: vec![]
        }
    }

    /// Read from buffer, returning the number of items read
    pub fn read(&mut self, bytes: &[u8]) -> Result<usize> {
        let start = self.table_offset;
        let end = start + self.table_size;

        let size = self.item_size;

        // reserve a temporary buffer for items
        let mut items: Vec<T> = vec![];

        // if a size is given, reserve space upfront
        if size > 0 {
            items.reserve(self.table_size / size);
        }

        // build a delimiter for the item type
        let delim = T::delimiter(size);

        // iterate over entity-sized slices of the byte array
        for data in ByteIter::new(&bytes[start..end],delim) {
            let mut item = T::default();

            // set expected layout and width from table
            item.set_layout(self.layout);
            item.set_width(self.width);

            // parse the table item and add to collection
            item.read(data)?;
            items.push(item);
        }

        // don't update self until successful read
        self.items = items;
        Ok(self.items.len())
    }

    /// Write to buffer, returning the number of items written
    pub fn write(&self, bytes: &mut [u8]) -> Result<usize> {

        // check buffer is big enough
        if bytes.len() > self.size() {
            return Err(Error::OutOfBoundsError);
        }

        let mut offset = 0;

        // reserve a temporary buffer for items
        let mut items: Vec<u8> = vec![];
        items.resize(self.size(),0);

        // iterate all contained items
        for item in self.items.iter() {
            
            // calculate position in the output buffer
            let start = offset;
            let end = start + item.size();

            // get a constrained, mutable slice of bytes
            let buffer = &mut items[start..end];

            // write the item to the byte slice
            item.write(buffer)?;

            // update the offset for the next item
            offset += item.size();
        }

        // write to output buffer on success
        bytes.copy_from_slice(&items);

        Ok(self.len())
    }

    /// Get the number of items in the table
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Get the calculated size of the table in bytes
    pub fn size(&self) -> usize {
        self.items
            .iter()
            .fold(0,|a,v| a + v.size())
    }

    /// Get an item at a given index
    pub fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    /// Set index to item, returning the index
    pub fn set(&mut self, index: usize, item: T) -> Result<usize> {
        if self.items.len() > index {
            self.items[index] = item;
            Ok(index)
        } else {
            Err(Error::OutOfBoundsError)
        }
    }

    /// Add item, returning the index
    pub fn add(&mut self, item: T) -> Result<usize> {
        self.items.push(item);
        Ok(self.len().saturating_sub(1))
    }

    /// Delete and return an item
    pub fn del(&mut self, index: usize) -> Option<T> {
        if self.items.len() > index {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

}

impl TryFrom<&SectionHeader> for SymbolTable {
    type Error = Error;

    fn try_from(header: &SectionHeader) -> Result<Self> {
        match header.kind().unwrap_or(SHType::SHT_NULL) {
            SHType::SHT_SYMTAB => Ok(Self::new(
                header.offset().ok_or(Error::MalformedDataError)?,
                header.body_size().ok_or(Error::MalformedDataError)?,
                header.entsize().ok_or(Error::MalformedDataError)?,
                header.layout(),
                header.width()
            )),
            _ => Err(Error::WrongSectionError)
        }
    }
}

impl TryFrom<&SectionHeader> for StringTable {
    type Error = Error;

    fn try_from(header: &SectionHeader) -> Result<Self> {
        match header.kind().unwrap_or(SHType::SHT_NULL) {
            SHType::SHT_STRTAB => Ok(Self::new(
                header.offset().ok_or(Error::MalformedDataError)?,
                header.body_size().ok_or(Error::MalformedDataError)?,
                header.entsize().ok_or(Error::MalformedDataError)?,
                header.layout(),
                header.width()
            )),
            _ => Err(Error::WrongSectionError)
        }
    }
}

impl TryFrom<&SectionHeader> for RelocationTable {
    type Error = Error;

    fn try_from(header: &SectionHeader) -> Result<Self> {
        match header.kind().unwrap_or(SHType::SHT_NULL) {
            SHType::SHT_RELA | SHType::SHT_REL => Ok(Self::new(
                header.offset().ok_or(Error::MalformedDataError)?,
                header.body_size().ok_or(Error::MalformedDataError)?,
                header.entsize().ok_or(Error::MalformedDataError)?,
                header.layout(),
                header.width()
            )),
            _ => Err(Error::WrongSectionError)
        }
    }
}

impl TryFrom<SectionHeader> for SymbolTable {
    type Error = Error;

    fn try_from(header: SectionHeader) -> Result<Self> {
        Self::try_from(&header)
    }
}

impl TryFrom<SectionHeader> for StringTable {
    type Error = Error;

    fn try_from(header: SectionHeader) -> Result<Self> {
        Self::try_from(&header)
    }
}

impl TryFrom<SectionHeader> for RelocationTable {
    type Error = Error;

    fn try_from(header: SectionHeader) -> Result<Self> {
        Self::try_from(&header)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::headers::{FileHeader,SectionHeader};

    use crate::utilities::tests::{
        LIBVPF_DYNSYM as SYM_TEST,
        LIBVPF_SHSTRTAB as STR_TEST,
        LIBVPF_RELA_DYN as REL_TEST,
        read
    };

    #[test]
    fn test_extract_real_relocation_section_as_table() {
        const SYMBOL_COUNT: usize = 210;

        let b = read("assets/libjpeg/libjpeg.so.9");

        let file_header = FileHeader::parse(&b)
            .unwrap();

        let count = file_header.shnum().unwrap();
        let offset = file_header.shoff().unwrap();
        let size = file_header.shentsize().unwrap();
        let layout = file_header.data().unwrap();
        let width = file_header.class().unwrap();
        
        let section_headers = SectionHeader::parse_all(
            &b,
            count,
            offset,
            size,
            layout,
            width);

        assert!(section_headers.is_ok());
        let headers = section_headers.unwrap();

        let result = headers.iter().find(|&h| 
            h.kind() == Some(SHType::SHT_RELA));

        assert!(result.is_some());

        let header = result.unwrap();
        let result = RelocationTable::try_from(header);

        assert!(result.is_ok());

        let mut table = result.unwrap();

        assert!(table.read(&b).is_ok());
        assert_eq!(table.len(),SYMBOL_COUNT);
    }

    #[test]
    fn test_read_relocation_table() {
        
        // directly initialize a test table
        let mut table = RelocationTable::new(
            0, // because we're reading directly
            REL_TEST.size,
            REL_TEST.entsize,
            Layout::Little,
            Width::X64
        );

        // read the test table and verify success
        let result = table.read(REL_TEST.bytes);
        assert!(result.is_ok());

        // verify that the table has the expected number of elements
        assert_eq!(table.len(),REL_TEST.length);
    }

    #[test]
    fn test_write_relocation_table_with_no_changes() {

        // directly initialize a test table
        let mut table = RelocationTable::new(
            0, // because we're reading directly
            REL_TEST.size,
            REL_TEST.entsize,
            Layout::Little,
            Width::X64
        );

        // read the test table and verify success
        let mut result = table.read(REL_TEST.bytes);
        assert!(result.is_ok());

        // initialize a buffer big enough for table data
        let mut buffer: Vec<u8> = vec![];
        buffer.resize(table.size(),0x00);

        // write to the new table
        result = table.write(buffer.as_mut_slice());
        assert!(result.is_ok());

        // verify that the written table is the same as original
        assert_eq!(buffer.as_slice(),REL_TEST.bytes);
    }

    #[test]
    fn test_write_relocation_table_with_changes() {

        // directly initialize a test table
        let mut table = RelocationTable::new(
            0, // because we're reading directly
            REL_TEST.size,
            REL_TEST.entsize,
            Layout::Little,
            Width::X64
        );

        // read the test table and verify success
        let result = table.read(REL_TEST.bytes);
        assert!(result.is_ok());

        // get an item from the table
        let result = table.get(1);
        assert!(result.is_some());

        // modify the item attributes
        let mut item = result.cloned().unwrap();
        item.set_addend(20);

        // update the table with the modified item
        let result = table.set(1,item);
        assert!(result.is_ok());

        // initialize a buffer big enough for modified table data
        let mut buffer: Vec<u8> = vec![];
        buffer.resize(table.size(),0x00);

        // write to the new table
        let result = table.write(buffer.as_mut_slice());
        assert!(result.is_ok());

        // verify that the written table is not the same as original
        assert_ne!(buffer.as_slice(),REL_TEST.bytes);

        // read the buffer and verify success
        let result = table.read(&buffer);
        assert!(result.is_ok());

        // get a relocation from the table
        let result = table.get(1);
        assert!(result.is_some());

        // check the item attribute is changed
        let item = result.unwrap();
        assert_eq!(item.addend_unchecked(),20);
    }

    #[test]
    fn test_extract_real_shstrtab_section_as_table() {
        let b = read("assets/libvpf/libvpf.so.4.1");

        // get the fileheader and use it to find section headers
        let file_header = FileHeader::parse(&b)
            .unwrap();

        let count = file_header.shnum().unwrap();
        let offset = file_header.shoff().unwrap();
        let size = file_header.shentsize().unwrap();
        let layout = file_header.data().unwrap();
        let width = file_header.class().unwrap();
        let index = file_header.shstrndx().unwrap();
        
        // parse all section headers from the buffer
        let section_headers = SectionHeader::parse_all(
            &b,
            count,
            offset,
            size,
            layout,
            width);

        assert!(section_headers.is_ok());
        let headers = section_headers.unwrap();

        let result = headers
            .iter()
            .enumerate()
            .find(|(i,_)| *i == index)
            .map(|(_,h)| h);

        assert!(result.is_some());

        let header = result.unwrap();
        let result = StringTable::try_from(header);

        assert!(result.is_ok());

        let mut table = result.unwrap();

        let result = table.read(&b);
        assert!(result.is_ok());
        assert_eq!(table.len(),STR_TEST.length);
    }

    #[test]
    fn test_read_string_table() {
        // directly initialize a table
        let mut table = StringTable::new(
            0, // because we're reading directly
            STR_TEST.size,
            STR_TEST.entsize,
            Layout::Little,
            Width::X64
        );

        let result = table.read(STR_TEST.bytes);
        assert!(result.is_ok());

        // verify that the table has the expected number of elements
        assert_eq!(table.len(),STR_TEST.length);
    }

    #[test]
    fn test_write_string_table_with_no_changes() {
        // directly initialize a table
        let mut table = StringTable::new(
            0, // because we're reading directly
            STR_TEST.size,
            STR_TEST.entsize,
            Layout::Little,
            Width::X64
        );

        let mut result = table.read(STR_TEST.bytes);
        assert!(result.is_ok());

        // initialize a buffer big enough for table data
        let mut buffer: Vec<u8> = vec![];
        buffer.resize(table.size(),0x00);

        // write to the new table
        result = table.write(buffer.as_mut_slice());
        assert!(result.is_ok());

        // verify that the written table is the same as original
        assert_eq!(buffer.as_slice(),STR_TEST.bytes);
    }

    #[test]
    fn test_write_string_table_with_changes() {
        const TEST_STR: &str  = "-test";
        const TEST_LEN: usize = 5;

        // directly initialize a table
        let mut table = StringTable::new(
            0, // because we're reading directly
            STR_TEST.size,
            STR_TEST.entsize,
            Layout::Little,
            Width::X64
        );

        // read the test table and verify success
        let result = table.read(STR_TEST.bytes);
        assert!(result.is_ok());

        // get a value from the table
        let result = table.get(1);  
        assert!(result.is_some());

        // get the item and string from the table
        let mut item = result.cloned().unwrap();
        let mut string = item.string_lossy();

        // modify the string by appending test str
        string += TEST_STR;
        assert_eq!(string.as_str(),".shstrtab-test");

        let result = item.set_string(string);
        assert!(result.is_ok());

        // update the table with the modified value
        let result = table.set(1,item);
        assert!(result.is_ok());

        // initialize a buffer big enough for modified table data
        let mut buffer: Vec<u8> = vec![];
        buffer.resize(table.size(),0x00);

        // write to the new table
        let result = table.write(buffer.as_mut_slice());
        assert!(result.is_ok());

        // verify that the written table is not the same as original
        assert_ne!(buffer.as_slice(),STR_TEST.bytes);
        assert_eq!(buffer.len(),STR_TEST.size + TEST_LEN);
    }


    #[test]
    fn test_extract_real_symtab_section_as_table() {
        const SYMBOL_COUNT: usize = 525;

        let b = read("assets/libjpeg/libjpeg.so.9");

        let file_header = FileHeader::parse(&b)
            .unwrap();

        let count = file_header.shnum().unwrap();
        let offset = file_header.shoff().unwrap();
        let size = file_header.shentsize().unwrap();
        let layout = file_header.data().unwrap();
        let width = file_header.class().unwrap();
        
        // parse all section headers from the buffer
        let section_headers = SectionHeader::parse_all(
            &b,
            count,
            offset,
            size,
            layout,
            width);

        assert!(section_headers.is_ok());
        let headers = section_headers.unwrap();

        let result = headers.iter().find(|&h| 
            h.kind() == Some(SHType::SHT_SYMTAB));

        assert!(result.is_some());

        let header = result.unwrap();
        let result = SymbolTable::try_from(header);

        assert!(result.is_ok());

        let mut table = result.unwrap();

        assert!(table.read(&b).is_ok());
        assert_eq!(table.len(),SYMBOL_COUNT);
    }

    #[test]
    fn test_read_symbol_table() {
        
        // directly initialize a table
        let mut table = SymbolTable::new(
            0, // because we're reading directly
            SYM_TEST.size,
            SYM_TEST.entsize,
            Layout::Little,
            Width::X64
        );

        // read the test table and verify success
        let result = table.read(SYM_TEST.bytes);
        assert!(result.is_ok());

        // verify that the table has the expected number of elements
        assert_eq!(table.len(),SYM_TEST.length);
    }

    #[test]
    fn test_write_symbol_table_with_no_changes() {

        // directly initialize a table
        let mut table = SymbolTable::new(
            0, // because we're reading directly
            SYM_TEST.size,
            SYM_TEST.entsize,
            Layout::Little,
            Width::X64
        );

        // read the test table and verify success
        let mut result = table.read(SYM_TEST.bytes);
        assert!(result.is_ok());

        // initialize a buffer big enough for table data
        let mut buffer: Vec<u8> = vec![];
        buffer.resize(table.size(),0x00);

        // write to the new table
        result = table.write(buffer.as_mut_slice());
        assert!(result.is_ok());

        // verify that the written table is the same as original
        assert_eq!(buffer.as_slice(),SYM_TEST.bytes);
    }

    #[test]
    fn test_write_symbol_table_with_changes() {

        // directly initialize a table
        let mut table = SymbolTable::new(
            0, // because we're reading directly
            SYM_TEST.size,
            SYM_TEST.entsize,
            Layout::Little,
            Width::X64
        );

        // read the test table and verify success
        let result = table.read(SYM_TEST.bytes);
        assert!(result.is_ok());

        // get an item from the table
        let result = table.get(1);
        assert!(result.is_some());

        // modify the item attributes
        let mut item = result.cloned().unwrap();
        item.set_value(20);

        // update the table with the modified item
        let result = table.set(1,item);
        assert!(result.is_ok());

        // initialize a buffer big enough for modified table data
        let mut buffer: Vec<u8> = vec![];
        buffer.resize(table.size(),0x00);

        // write to the new table
        let result = table.write(buffer.as_mut_slice());
        assert!(result.is_ok());

        // verify that the written table is not the same as original
        assert_ne!(buffer.as_slice(),SYM_TEST.bytes);

        // read the buffer and verify success
        let result = table.read(&buffer);
        assert!(result.is_ok());

        // get a item from the table
        let result = table.get(1);
        assert!(result.is_some());

        // check the item attribute is changed
        let item = result.unwrap();
        assert_eq!(item.value_unchecked(),20);
    }
}