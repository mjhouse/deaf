use crate::common::{SHType};
use crate::errors::{Error,Result};
use crate::headers::SectionHeader;
use crate::tables::{StringItem,Table};

/// Alias for a Table that contains StringItem records
pub type StringTable = Table<StringItem>;

impl TryFrom<&SectionHeader> for StringTable {
    type Error = Error;

    fn try_from(header: &SectionHeader) -> Result<Self> {
        match header.kind() {
            SHType::SHT_STRTAB => Ok(Self::new(
                header.offset(),
                header.body_size(),
                header.entsize(),
                header.layout(),
                header.width()
            )),
            _ => Err(Error::WrongSectionError)
        }
    }
}

impl TryFrom<SectionHeader> for StringTable {
    type Error = Error;

    fn try_from(header: SectionHeader) -> Result<Self> {
        Self::try_from(&header)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::headers::{FileHeader,SectionHeader};
    use crate::common::{Width,Layout};
    use crate::utilities::read;

    use crate::utilities::tests::{
        LIBVPF_SHSTRTAB as STR_TEST,
    };

    #[test]
    fn test_extract_real_shstrtab_section_as_table() {
        let b = read("assets/libvpf/libvpf.so.4.1").unwrap();

        // get the fileheader and use it to find section headers
        let file_header = FileHeader::parse(&b).unwrap();

        let count = file_header.shnum();
        let offset = file_header.shoff();
        let size = file_header.shentsize();
        let layout = file_header.data();
        let width = file_header.class();
        let index = file_header.shstrndx();
        
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

        let result = headers.get(index);

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
}