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
    is_parsed: bool,
    entity_size: usize,
    section_size: usize,
    values: Vec<Symbol>
}

impl SymbolTable {

    pub fn new(header: SectionHeader) -> Result<Self> {
        match header.values.sh_type {
            SHType::SHT_SYMTAB => Ok(Self {
                offset: header.values.sh_offset,
                layout: header.layout(),
                width: header.width(),
                is_parsed: false,
                entity_size: header.values.sh_entsize,
                section_size: header.values.sh_size,
                values: vec![],
            }),
            _ => Err(Error::WrongSectionError)
        }
    }

    pub fn parse(header: SectionHeader, bytes: &[u8]) -> Result<Self> {
        let mut table = Self::new(header)?;
        table.read(bytes)?;
        Ok(table)
    }

    pub fn read(&mut self, bytes: &[u8]) -> Result<&Vec<Symbol>> {
        self.values.clear();

        let start = self.offset;
        let end = self.offset + self.section_size;

        let size = self.entity_size;
        let layout = self.layout;
        let width = self.width;

        self.values = ByteIter::length(&bytes[start..end],size)
            .map(|s| Symbol::parse(s,layout,width))
            .collect::<Result<Vec<Symbol>>>()?;

        self.is_parsed = true;
        Ok(&self.values)
    }

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

            // write each symbol to the symbol table
            let buffer = &mut bytes[symbol_start..symbol_end];
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

    fn set(&mut self, index: usize, item: Symbol) {
        self.values[index] = item;
    }

    fn add(&mut self, symbol: Symbol) {
        self.values.push(symbol);
    }

    fn del(&mut self, index: usize) -> Option<Symbol> {
        if self.values.len() > index {
            Some(self.values.remove(index))
        } else {
            None
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

    #[test]
    fn test_extract_symtab_section_as_table() {
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
                let mut result = SymbolTable::parse(section,&b);
                
                assert!(result.is_ok());
                let table = result.unwrap();

                assert_eq!(table.len(),SYMBOL_COUNT);
            }
        }
    }
}