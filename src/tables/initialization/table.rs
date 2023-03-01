use crate::errors::{Error, Result};
use crate::headers::common::constants::{Width,Layout,SHType};
use crate::headers::section::header::{
    SectionHeader,
    SectionHeaderValues
};
use crate::tables::initialization::Initialization;
use crate::tables::common::ByteIter;
use crate::tables::common::Table;

pub struct InitializationTable {
    offset: usize,
    layout: Layout,
    width: Width,
    entity_size: usize,
    section_size: usize,
    values: Vec<Initialization>
}

impl InitializationTable {

    pub fn new(offset: usize, size: usize, layout: Layout, width: Width, entity_size: usize) -> Self {
        Self {
            offset: offset,
            layout: layout,
            width: width,
            entity_size: entity_size,
            section_size: size,
            values: vec![],
        }
    }

    // reads from an offset to offset + section_size
    pub fn read(&mut self, bytes: &[u8]) -> Result<usize> {
        let start = self.offset;
        let end = self.offset + self.section_size;

        let size = self.entity_size;
        let layout = self.layout;
        let width = self.width;

        // check that the entity size is > 0
        if size == 0 {
            return Err(Error::MalformedDataError);
        }

        // check that the section has data
        if self.section_size == 0 {
            return Err(Error::MalformedDataError);
        }

        // check that entities fit cleanly into section
        if self.section_size % size != 0 {
            return Err(Error::MalformedDataError);
        }

        // reserve a temporary buffer for entities
        let mut values = vec![];
        values.reserve(self.section_size / size);

        for data in ByteIter::length(&bytes[start..end],size) {
            // parse an entity from the byte range
            let initialization = Initialization::parse(data,layout,width)?;

            // add to vector of Initialization objects
            values.push(initialization);
        }

        // don't update self until successful read
        self.values = values;
        Ok(self.values.len())
    }

    // writes from the beginning of the given byte slice
    pub fn write(&self, bytes: &mut [u8]) -> Result<usize> {

        // check buffer is big enough
        if bytes.len() > self.size() {
            return Err(Error::OutOfBoundsError);
        }

        let size = self.entity_size;
        let layout = self.layout;
        let width = self.width;

        // iterate all contained initializations
        for (i,initialization) in self.values.iter().enumerate() {
            
            // calculate initialization position in the output buffer
            let reloc_start = i * size;
            let reloc_end = reloc_start + size;

            // get a constrained, mutable slice of bytes to write to
            let buffer = &mut bytes[reloc_start..reloc_end];

            // write the initialization to the byte slice
            initialization.write(buffer)?;
        }

        Ok(self.values.len())
    }

}

impl Table<Initialization> for InitializationTable {

    fn len(&self) -> usize {
        self.values.len()
    }

    fn size(&self) -> usize {
        self.len() * self.entity_size
    }

    fn get(&self, index: usize) -> Option<Initialization> {
        self.values.get(index).cloned()
    }

    fn set(&mut self, index: usize, item: Initialization) -> Result<usize> {
        self.values[index] = item;
        Ok(index)
    }

    fn add(&mut self, item: Initialization) -> Result<usize> {
        self.values.push(item);
        Ok(self.len().saturating_sub(1))
    }

    fn del(&mut self, index: usize) -> Option<Initialization> {
        if self.values.len() > index {
            Some(self.values.remove(index))
        } else {
            None
        }
    }

}

impl TryFrom<SectionHeader> for InitializationTable {
    type Error = Error;

    fn try_from(header: SectionHeader) -> Result<Self> {
        match header.values.sh_type {
            SHType::SHT_INIT_ARRAY => Ok(Self::new(
                header.offset(),
                header.size(),
                header.layout(),
                header.width(),
                header.entsize()
            )),
            _ => Err(Error::WrongSectionError)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use crate::headers::file::header::FileHeader;
    use crate::headers::section::header::SectionHeader;

    const TEST_TABLE: &[u8] = include!("../../../assets/bytes/libqscintilla_init_array.in");

    // the starting byte of the test table
    const TEST_TABLE_OFFSET: usize = 0;

    // the length in bytes of the test table
    const TEST_TABLE_LENGTH: usize = 912;

    // the number of elements in the test table
    const TEST_TABLE_COUNT: usize = 114;

    // the size of an element in the test table
    const TEST_TABLE_ENTITY: usize = 8;

    // #[test]
    // fn test_extract_real_relocation_section_as_table() {
    //     const SYMBOL_COUNT: usize = 210;

    //     let mut f = File::open("assets/libjpeg.so.9").unwrap();
    //     let mut b = Vec::new();
        
    //     f.read_to_end(&mut b)
    //         .unwrap();

    //     let file_header = FileHeader::parse(&b)
    //         .unwrap();

    //     let count = file_header.shnum();
    //     let offset = file_header.shoff();
    //     let size = file_header.shentsize();
    //     let layout = file_header.data();
    //     let width = file_header.class();
        
    //     let section_headers = SectionHeader::parse_all(
    //         &b,
    //         count,
    //         offset,
    //         size,
    //         layout,
    //         width);

    //     assert!(section_headers.is_ok());
    //     let headers = section_headers.unwrap();

    //     for section in headers.into_iter() {
    //         if section.section_type() == SHType::SHT_RELA {
    //             // build a table from the section
    //             let result = RelocationTable::try_from(section);
    //             assert!(result.is_ok());
    //             let mut table = result.unwrap();

    //             // read the table from the buffer
    //             assert!(table.read(&b).is_ok());

    //             // verify that the table has expected length
    //             assert_eq!(table.len(),SYMBOL_COUNT);
    //             break;
    //         }
    //     }
    // }

    // #[test]
    // fn test_read_relocation_table() {
        
    //     // directly initialize a relocation table
    //     let mut table = RelocationTable::new(
    //         TEST_TABLE_OFFSET,
    //         TEST_TABLE_LENGTH,
    //         Layout::Little,
    //         Width::X64,
    //         TEST_TABLE_ENTITY
    //     );

    //     // read the test table and verify success
    //     let result = table.read(TEST_TABLE);
    //     assert!(result.is_ok());

    //     // verify that the table has the expected number of elements
    //     assert_eq!(table.len(),TEST_TABLE_COUNT);
    // }

    // #[test]
    // fn test_write_relocation_table_with_no_changes() {

    //     // directly initialize a symbol table
    //     let mut table = RelocationTable::new(
    //         TEST_TABLE_OFFSET,
    //         TEST_TABLE_LENGTH,
    //         Layout::Little,
    //         Width::X64,
    //         TEST_TABLE_ENTITY
    //     );

    //     // read the test table and verify success
    //     let mut result = table.read(TEST_TABLE);
    //     assert!(result.is_ok());

    //     // initialize a buffer big enough for table data
    //     let mut buffer: Vec<u8> = vec![];
    //     buffer.resize(table.size(),0x00);

    //     // write to the new table
    //     result = table.write(buffer.as_mut_slice());
    //     assert!(result.is_ok());

    //     // verify that the written table is the same as original
    //     assert_eq!(buffer.as_slice(),TEST_TABLE);
    // }

    // #[test]
    // fn test_write_relocation_table_with_changes() {

    //     // directly initialize a relocation table
    //     let mut table = RelocationTable::new(
    //         TEST_TABLE_OFFSET,
    //         TEST_TABLE_LENGTH,
    //         Layout::Little,
    //         Width::X64,
    //         TEST_TABLE_ENTITY
    //     );

    //     // read the test table and verify success
    //     let mut result = table.read(TEST_TABLE);
    //     assert!(result.is_ok());

    //     // get a relocation from the table
    //     let result = table.get(1);
    //     assert!(result.is_some());

    //     // modify the relocation attributes
    //     let mut relocation = result.unwrap();
    //     relocation.set_addend(Some(20));

    //     // update the string table with the modified string
    //     let result = table.set(1,relocation);
    //     assert!(result.is_ok());

    //     // initialize a buffer big enough for modified table data
    //     let mut buffer: Vec<u8> = vec![];
    //     buffer.resize(table.size(),0x00);

    //     // write to the new table
    //     let result = table.write(buffer.as_mut_slice());
    //     assert!(result.is_ok());

    //     // verify that the written table is not the same as original
    //     assert_ne!(buffer.as_slice(),TEST_TABLE);

    //     // read the buffer and verify success
    //     let mut result = table.read(&buffer);
    //     assert!(result.is_ok());

    //     // get a relocation from the table
    //     let result = table.get(1);
    //     assert!(result.is_some());

    //     // check the relocation attribute is changed
    //     let mut relocation = result.unwrap();
    //     assert_eq!(relocation.addend(),Some(20));
    // }
}