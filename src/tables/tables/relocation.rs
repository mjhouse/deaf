use crate::common::{SHType};
use crate::errors::{Error,Result};
use crate::headers::SectionHeader;

use crate::tables::{
    RelaItem,
    RelItem,
    Table
};

/// Alias for a Table that contains RelaItem records
pub type RelaTable = Table<RelaItem>;

/// Alias for a Table that contains RelItem records
pub type RelTable = Table<RelItem>;

impl TryFrom<&SectionHeader> for RelaTable {
    type Error = Error;

    fn try_from(header: &SectionHeader) -> Result<Self> {
        match header.kind() {
            SHType::SHT_RELA => Ok(Self::new(header)),
            _ => Err(Error::WrongSectionError)
        }
    }
}

impl TryFrom<&SectionHeader> for RelTable {
    type Error = Error;

    fn try_from(header: &SectionHeader) -> Result<Self> {
        match header.kind() {
            SHType::SHT_REL => Ok(Self::new(header)),
            _ => Err(Error::WrongSectionError)
        }
    }
}

impl TryFrom<SectionHeader> for RelTable {
    type Error = Error;

    fn try_from(header: SectionHeader) -> Result<Self> {
        Self::try_from(&header)
    }
}

impl TryFrom<SectionHeader> for RelaTable {
    type Error = Error;

    fn try_from(header: SectionHeader) -> Result<Self> {
        Self::try_from(&header)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::headers::{FileHeader,SectionHeader,SectionHeaderData};
    use crate::common::{Width,Layout};
    use crate::utilities::read;

    use crate::utilities::tests::{
        LIBVPF_RELA_DYN as REL_TEST,
    };

    #[test]
    fn test_extract_real_rela_section_as_table() {
        const SYMBOL_COUNT: usize = 210;

        let b = read("assets/libjpeg/libjpeg.so.9").unwrap();

        let file_header = FileHeader::parse(&b).unwrap();

        let count = file_header.shnum();
        let offset = file_header.shoff();
        let size = file_header.shentsize();
        let layout = file_header.data();
        let width = file_header.class();
        
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
            h.kind() == SHType::SHT_RELA);

        assert!(result.is_some());

        let header = result.unwrap();
        let result = RelaTable::try_from(header);

        assert!(result.is_ok());

        let mut table = result.unwrap();

        assert!(table.read(&b).is_ok());
        assert_eq!(table.len(),SYMBOL_COUNT);
    }

    #[test]
    fn test_read_rela_table() {
        
        let header = SectionHeader::from(SectionHeaderData {
            layout: Layout::Little,
            width: Width::X64,
            sh_type: SHType::SHT_RELA,
            sh_offset: 0, // because we're reading directly
            sh_size: REL_TEST.size,
            sh_entsize: REL_TEST.entsize,
            ..Default::default()
        });

        // directly initialize a test table
        let mut table = RelaTable::try_from(header).unwrap();

        // read the test table and verify success
        let result = table.read(REL_TEST.bytes);
        assert!(result.is_ok());

        // verify that the table has the expected number of elements
        assert_eq!(table.len(),REL_TEST.length);
    }

    #[test]
    fn test_write_rela_table_with_no_changes() {

        let header = SectionHeader::from(SectionHeaderData {
            layout: Layout::Little,
            width: Width::X64,
            sh_type: SHType::SHT_RELA,
            sh_offset: 0, // because we're reading directly
            sh_size: REL_TEST.size,
            sh_entsize: REL_TEST.entsize,
            ..Default::default()
        });

        // directly initialize a test table
        let mut table = RelaTable::try_from(header).unwrap();

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
    fn test_write_rela_table_with_changes() {

        let header = SectionHeader::from(SectionHeaderData {
            layout: Layout::Little,
            width: Width::X64,
            sh_type: SHType::SHT_RELA,
            sh_offset: 0, // because we're reading directly
            sh_size: REL_TEST.size,
            sh_entsize: REL_TEST.entsize,
            ..Default::default()
        });

        // directly initialize a test table
        let mut table = RelaTable::try_from(header).unwrap();

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
        assert_eq!(item.addend(),20);
    }
}