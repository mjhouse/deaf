use crate::errors::{Error, Result};
use crate::headers::common::constants::{Width,Layout,SHType,sizes};
use crate::headers::section::header::{
    SectionHeader,
    SectionHeaderValues
};
use crate::tables::symbol::Symbol;
use crate::tables::common::ByteIter;
use crate::tables::common::Table;

pub struct SymbolTable {
    offset: usize,
    layout: Layout,
    width: Width,
    entity_size: usize,
    section_size: usize,
    values: Vec<Symbol>
}

impl SymbolTable {

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

        // reserve a temporary buffer for symbols
        let mut values = vec![];
        values.reserve(self.section_size / size);

        for data in ByteIter::length(&bytes[start..end],size) {
            // parse a symbol from the byte range
            let symbol = Symbol::parse(data,layout,width)?;

            // add to vector of Symbol objects
            values.push(symbol);
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

        // iterate all contained symbols
        for (i,symbol) in self.values.iter().enumerate() {
            
            // calculate symbol position in the output buffer
            let symbol_start = i * size;
            let symbol_end = symbol_start + size;

            // get a constrained, mutable slice of bytes to write to
            let buffer = &mut bytes[symbol_start..symbol_end];

            // write the symbol to the byte slice
            symbol.write(buffer)?;
        }

        Ok(self.values.len())
    }

}

impl Table<Symbol> for SymbolTable {

    fn len(&self) -> usize {
        self.values.len()
    }

    fn size(&self) -> usize {
        self.len() * self.entity_size
    }

    fn get(&self, index: usize) -> Option<Symbol> {
        self.values.get(index).cloned()
    }

    fn set(&mut self, index: usize, item: Symbol) -> Result<usize> {
        self.values[index] = item;
        Ok(index)
    }

    fn add(&mut self, symbol: Symbol) -> Result<usize> {
        self.values.push(symbol);
        Ok(self.len().saturating_sub(1))
    }

    fn del(&mut self, index: usize) -> Option<Symbol> {
        if self.values.len() > index {
            Some(self.values.remove(index))
        } else {
            None
        }
    }

}

impl TryFrom<SectionHeader> for SymbolTable {
    type Error = Error;

    fn try_from(header: SectionHeader) -> Result<Self> {
        match header.values.sh_type {
            SHType::SHT_SYMTAB => Ok(Self::new(
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

    const TEST_TABLE: &[u8] = include!("../../../assets/bytes/libvpf_symtab.in");

    // the starting byte of the test table
    const TEST_TABLE_OFFSET: usize = 0;

    // the length in bytes of the test table
    const TEST_TABLE_LENGTH: usize = 7056;

    // the number of elements in the test table
    const TEST_TABLE_COUNT: usize = 294;

    #[test]
    fn test_extract_real_symtab_section_as_table() {
        const SYMBOL_COUNT: usize = 525;

        let mut f = File::open("assets/libjpeg.so.9").unwrap();
        let mut b = Vec::new();
        
        f.read_to_end(&mut b)
            .unwrap();

        let file_header = FileHeader::parse(&b)
            .unwrap();

        let count = file_header.shnum();
        let offset = file_header.shoff();
        let layout = file_header.data();
        let width = file_header.class();
        
        let section_headers = SectionHeader::parse_all(
            &b,
            count,
            offset,
            layout,
            width);

        assert!(section_headers.is_ok());
        let headers = section_headers.unwrap();

        for section in headers.into_iter() {
            if section.section_type() == SHType::SHT_SYMTAB {
                // build a string table from the section
                let result = SymbolTable::try_from(section);
                assert!(result.is_ok());
                let mut table = result.unwrap();

                // read the string table from the buffer
                assert!(table.read(&b).is_ok());

                // verify that the string table has expected length
                assert_eq!(table.len(),SYMBOL_COUNT);
            }
        }
    }

    // #[test]
    // fn test_read_symbol_table() {
    //     // read the test table data
    //     let mut table = SymbolTable::new(TEST_TABLE_OFFSET,TEST_TABLE_LENGTH);
    //     let result = table.read(TEST_TABLE);
    //     assert!(result.is_ok());

    //     // verify that the table has the expected number of elements
    //     assert_eq!(table.len(),TEST_TABLE_COUNT);
    // }
}