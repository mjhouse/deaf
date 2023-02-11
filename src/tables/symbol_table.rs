use crate::errors::{Error, Result};
use crate::headers::section::header::{
    SectionHeader,
    SectionHeaderValues
};
use crate::headers::common::constants::{
    SHType
};

// https://docs.oracle.com/cd/E23824_01/html/819-0690/chapter6-79797.html
#[derive(Default,Debug,Clone)]
pub struct Symbol {
    name: u64,  // st_name
    value: u64, // st_value
    size: u64,  // st_size
    info: u8,   // st_info
    other: u8,  // st_other
    shndx: u64, // st_shndx
}

pub struct SymbolTable {
    offset: usize,
    entity_size: usize,
    section_size: usize,
    values: Vec<Symbol>
}

impl SymbolTable {

    pub fn new(header: SectionHeader) -> Result<Self> {
        match header.values.sh_type {
            SHType::SHT_SYMTAB => Ok(Self {
                offset: header.values.sh_offset,
                entity_size: header.values.sh_entsize,
                section_size: header.values.sh_size,
                values: vec![],
            }),
            _ => Err(Error::WrongSectionError)
        }
    }

    pub fn read(&mut self, b: &[u8]) -> Vec<Symbol> {
        let start = self.offset;
        let end = start + self.section_size;

        // for v in b[start..end].iter() {
        //     match *v as char {
        //         '\0' => {
        //             result.push(string.clone());
        //             string.clear();
        //         },
        //         ch   => string.push(ch),
        //     }
        // }

        self.values.clone()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use crate::headers::file::header::FileHeader;
    use crate::headers::section::header::SectionHeader;

    // #[test]
    // fn test_extract_shstrtab_section_as_table() {
    //     let mut f = File::open("assets/libvpf.so.4.1").unwrap();
    //     let mut b = Vec::new();

    //     // the number of strings in the string table
    //     const STR_COUNT: usize = 26;
        
    //     f.read_to_end(&mut b)
    //         .unwrap();

    //     let file_header = FileHeader::parse(&b)
    //         .unwrap();

    //     let count = file_header.shnum();
    //     let offset = file_header.shoff();
    //     let layout = file_header.data();
    //     let width = file_header.class();
    //     let index = file_header.shstrndx();
        
    //     let section_headers = SectionHeader::parse_all(
    //         &b,
    //         count,
    //         offset,
    //         layout,
    //         width);

    //     assert!(section_headers.is_ok());
    //     let headers = section_headers.unwrap();

    //     for (i,section) in headers.into_iter().enumerate() {
    //         if i == index {
    //             let mut table = SymbolTable::new(section).unwrap();
    //             let result = table.read(&b);
    //             assert_eq!(result.len(),STR_COUNT);
    //             break;
    //         }
    //     }
    // }
}