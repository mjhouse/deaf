use crate::common::{SHType};
use crate::errors::{Error,Result};
use crate::headers::SectionHeader;
use crate::tables::{SymbolItem,Table};

/// Alias for a Table that contains SymbolItem records
pub type SymbolTable = Table<SymbolItem>;

impl TryFrom<&SectionHeader> for SymbolTable {
    type Error = Error;

    fn try_from(header: &SectionHeader) -> Result<Self> {
        match header.kind() {
            SHType::SHT_SYMTAB => Ok(Self::new(header)),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::headers::{FileHeader,SectionHeader,SectionHeaderData};
    use crate::common::{Width,Layout,SHType};
    use crate::utilities::read;

    use crate::utilities::tests::{
        LIBVPF_DYNSYM as SYM_TEST,
    };

    #[test]
    fn test_extract_real_symtab_section_as_table() {
        const SYMBOL_COUNT: usize = 525;

        let b = read("assets/libjpeg/libjpeg.so.9").unwrap();

        let file_header = FileHeader::parse(&b).unwrap();

        let count = file_header.shnum();
        let offset = file_header.shoff();
        let size = file_header.shentsize();
        let layout = file_header.data();
        let width = file_header.class();
        
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
            h.kind() == SHType::SHT_SYMTAB);

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
        
        let header = SectionHeader::from(SectionHeaderData {
            layout: Layout::Little,
            width: Width::X64,
            sh_type: SHType::SHT_SYMTAB,
            sh_offset: 0, // because we're reading directly
            sh_size: SYM_TEST.size,
            sh_entsize: SYM_TEST.entsize,
            ..Default::default()
        });

        // directly initialize a table
        let mut table = SymbolTable::try_from(header).unwrap();

        // read the test table and verify success
        let result = table.read(SYM_TEST.bytes);
        assert!(result.is_ok());

        // verify that the table has the expected number of elements
        assert_eq!(table.len(),SYM_TEST.length);
    }

    #[test]
    fn test_write_symbol_table_with_no_changes() {

        let header = SectionHeader::from(SectionHeaderData {
            layout: Layout::Little,
            width: Width::X64,
            sh_type: SHType::SHT_SYMTAB,
            sh_offset: 0, // because we're reading directly
            sh_size: SYM_TEST.size,
            sh_entsize: SYM_TEST.entsize,
            ..Default::default()
        });

        // directly initialize a table
        let mut table = SymbolTable::try_from(header).unwrap();

        // read the test table and verify success
        let mut result = table.read(SYM_TEST.bytes);
        assert!(result.is_ok());

        // initialize a buffer big enough for table data
        let mut buffer: Vec<u8> = vec![0;table.size()];

        // write to the new table
        result = table.write(buffer.as_mut_slice());
        assert!(result.is_ok());

        // verify that the written table is the same as original
        assert_eq!(buffer.as_slice(),SYM_TEST.bytes);
    }

    #[test]
    fn test_write_symbol_table_with_changes() {

        let header = SectionHeader::from(SectionHeaderData {
            layout: Layout::Little,
            width: Width::X64,
            sh_type: SHType::SHT_SYMTAB,
            sh_offset: 0, // because we're reading directly
            sh_size: SYM_TEST.size,
            sh_entsize: SYM_TEST.entsize,
            ..Default::default()
        });

        // directly initialize a table
        let mut table = SymbolTable::try_from(header).unwrap();

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
        assert_eq!(item.value(),20);
    }
}