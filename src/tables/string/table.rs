use crate::errors::{Error, Result};
use crate::headers::section::header::{
    SectionHeader,
    SectionHeaderValues
};
use crate::headers::common::constants::{
    SHType
};
use crate::tables::common::ByteIter;
use crate::tables::common::Table;
use std::ffi::{CString,CStr};

pub struct StringTable {
    offset: usize,
    section_size: usize,
    values: Vec<CString>
}

impl StringTable {

    pub fn new(offset: usize, size: usize) -> Self {
        Self {
            offset: offset,
            section_size: size,
            values: vec![]
        }
    }

    // reads from an offset to offset + section_size
    pub fn read(&mut self, b: &[u8]) -> Result<usize> {
        let start = self.offset;
        let end = start + self.section_size;

        // get a constrained slice of bytes to read
        let bytes = &b[start..end];
        let mut values = vec![];

        // skip the first element because it's empty
        for data in ByteIter::value(bytes,b'\0').skip(1) {

            // parse as c-style string from byte slice
            let cstr = CStr::from_bytes_with_nul(data)?;

            // add to vector of String values
            values.push(cstr.into());
        }

        // don't update self until successful read
        self.values = values;

        Ok(self.values.len())
    }

    // writes from the beginning of the given byte slice
    pub fn write(&self, bytes: &mut [u8]) -> Result<usize> {

        // check buffer is big enough
        if bytes.len() != self.size() {
            return Err(Error::OutOfBoundsError);
        }

        let mut string_start = 0;

        // initial empty string required by ELF format
        let initial = CString::default();
        let iter = std::iter::once(&initial);

        // iterate all contained strings
        for string in iter.chain(self.values.iter()) {
            
            // convert to nul-terminated c-string representation
            let data = string.as_bytes_with_nul();

            // calculate end position in the output buffer
            let string_end = string_start + data.len();

            // get a constrained, mutable slice of bytes to write to
            let buffer = &mut bytes[string_start..string_end];

            // copy the string to the byte slice
            buffer.clone_from_slice(data);

            // update the starting position for the next string
            string_start = string_end;
        }

        Ok(self.values.len())
    }
}

impl Table<String> for StringTable {

    fn len(&self) -> usize {
        self.values.len()
    }

    fn size(&self) -> usize {
        // add one for the empty first string
        1 + self.values
            .iter()
            .fold(0,|a,v| a + v.as_bytes_with_nul().len())
    }

    fn get(&self, index: usize) -> Option<String> {
        let cstring = self.values.get(index)?.clone();
        cstring.into_string().ok()
    }

    fn set(&mut self, index: usize, item: String) -> Result<usize> {
        let cstring = CString::new(item.as_bytes())?;
        self.values[index] = cstring;
        Ok(index)
    }

    fn add(&mut self, item: String) -> Result<usize> {
        let cstring = CString::new(item.as_bytes())?;
        self.values.push(cstring);
        Ok(self.len().saturating_sub(1))
    }

    fn del(&mut self, index: usize) -> Option<String> {
        if self.values.len() > index {
            self.values
                .remove(index)
                .into_string()
                .ok()
        } else {
            None
        }
    }

}

impl TryFrom<&SectionHeader> for StringTable {
    type Error = Error;

    fn try_from(header: &SectionHeader) -> Result<Self> {
        match header.values.sh_type {
            SHType::SHT_STRTAB => Ok(Self::new(
                header.offset(),
                header.size())),
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
    use std::fs::File;
    use std::io::Read;
    use crate::headers::file::header::FileHeader;
    use crate::headers::section::header::SectionHeader;

    const TEST_TABLE: &[u8] = include!("../../../assets/bytes/libvpf_strtab.in");

    // the starting byte of the test table
    const TEST_TABLE_OFFSET: usize = 0;

    // the length in bytes of the test table
    const TEST_TABLE_LENGTH: usize = 263;

    // the number of elements in the test table
    const TEST_TABLE_COUNT: usize = 25;

    #[test]
    fn test_extract_real_shstrtab_section_as_table() {
        let mut f = File::open("assets/libvpf.so.4.1").unwrap();
        let mut b = Vec::new();
        
        f.read_to_end(&mut b)
            .unwrap();

        // get the fileheader and use it to find section headers
        let file_header = FileHeader::parse(&b)
            .unwrap();

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

        assert!(table.read(&b).is_ok());
        assert_eq!(table.len(),TEST_TABLE_COUNT);
    }

    #[test]
    fn test_read_string_table() {
        // read the test table data
        let mut table = StringTable::new(TEST_TABLE_OFFSET,TEST_TABLE_LENGTH);
        let result = table.read(TEST_TABLE);
        assert!(result.is_ok());

        // verify that the table has the expected number of elements
        assert_eq!(table.len(),TEST_TABLE_COUNT);
    }

    #[test]
    fn test_write_string_table_with_no_changes() {
        // read the test table data
        let mut table = StringTable::new(TEST_TABLE_OFFSET,TEST_TABLE_LENGTH);
        let mut result = table.read(TEST_TABLE);
        assert!(result.is_ok());

        // initialize a buffer big enough for table data
        let mut buffer: Vec<u8> = vec![];
        buffer.resize(table.size(),0x00);

        // write to the new table
        result = table.write(buffer.as_mut_slice());
        assert!(result.is_ok());

        // verify that the written table is the same as original
        assert_eq!(buffer.as_slice(),TEST_TABLE);
    }

    #[test]
    fn test_write_string_table_with_changes() {
        const TEST_STR: &str  = "-test";
        const TEST_LEN: usize = 5;

        // read the test table data
        let mut table = StringTable::new(TEST_TABLE_OFFSET,TEST_TABLE_LENGTH);
        let result = table.read(TEST_TABLE);
        assert!(result.is_ok());

        // get a string from the table
        let result = table.get(0);  
        assert!(result.is_some());

        // append a test value to the string
        let mut string = result.unwrap();
        string += TEST_STR;
        assert_eq!(string.as_str(),".shstrtab-test");

        // update the string table with the modified string
        let result = table.set(0,string);
        assert!(result.is_ok());

        // initialize a buffer big enough for modified table data
        let mut buffer: Vec<u8> = vec![];
        buffer.resize(table.size(),0x00);

        // write to the new table
        let result = table.write(buffer.as_mut_slice());
        assert!(result.is_ok());

        // verify that the written table is not the same as original
        assert_ne!(buffer.as_slice(),TEST_TABLE);
        assert_eq!(buffer.len(),TEST_TABLE_LENGTH + TEST_LEN);
    }

}