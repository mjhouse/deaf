use crate::errors::{Error, Result};
use crate::headers::common::constants::{Width,Layout};
use crate::headers::section::header::{
    SectionHeader,
    SectionHeaderValues
};
use crate::headers::common::constants::{
    SHType
};
use crate::tables::symbol::Symbol;
use crate::tables::common::ByteIterator;

pub struct SymbolTable {
    offset: usize,
    layout: Layout,
    width: Width,
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
                entity_size: header.values.sh_entsize,
                section_size: header.values.sh_size,
                values: vec![],
            }),
            _ => Err(Error::WrongSectionError)
        }
    }

    pub fn read(&mut self, b: &[u8]) -> Result<&Vec<Symbol>> {
        let start = self.offset;
        let end = start + self.section_size;

        let size = self.entity_size;
        let layout = self.layout;
        let width = self.width;

        self.values = ByteIterator::length(&b[start..end],size)
            .map(|s| Symbol::parse(s,layout,width))
            .collect::<Result<Vec<Symbol>>>()?;

        Ok(&self.values)
    }

    pub fn len(&self) -> usize {
        self.values.len()
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
                let mut result = SymbolTable::new(section);
                assert!(result.is_ok());

                let mut table = result.unwrap();
                assert!(table.read(&b).is_ok());
                assert_eq!(table.len(),SYMBOL_COUNT);
            }
        }
    }
}