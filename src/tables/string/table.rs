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

    pub fn read(&mut self, b: &[u8]) -> Result<usize> {
        let start = self.offset;
        let end = start + self.section_size;

        let bytes = &b[start..end];
        let mut values = vec![];

        for data in ByteIter::value(bytes,b'\0') {
            // parse as c-style string from byte slice
            let cstr = CStr::from_bytes_with_nul(data)?;

            // add to vector of String values
            values.push(cstr.into());
        }

        // don't update self until successful read
        self.values = values;

        Ok(self.values.len())
    }

    pub fn write(&self, bytes: &mut [u8]) -> Result<usize> {

        // check buffer is big enough
        if bytes.len() > self.size() {
            return Err(Error::OutOfBoundsError);
        }

        let mut string_start = 0;

        // iterate all contained strings
        for string in self.values.iter() {
            
            // convert to nul-terminated c-string representation
            let cstring = CString::new(string.as_bytes())?;
            let data = cstring.as_bytes_with_nul();

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
        self.values
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

impl TryFrom<SectionHeader> for StringTable {
    type Error = Error;

    fn try_from(header: SectionHeader) -> Result<Self> {
        match header.values.sh_type {
            SHType::SHT_STRTAB => Ok(Self::new(
                header.offset(),
                header.size())),
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

    const TEST_TABLE: &[u8] = &[
        0x00, 0x2e, 0x73, 0x68, 0x73, 0x74, 0x72, 0x74, 0x61, 0x62, 0x00, 0x2e, 0x6e, 0x6f, 0x74, 0x65, 0x2e, 0x67, 0x6e, 0x75, 0x2e, 0x70, 0x72, 0x6f,
        0x70, 0x65, 0x72, 0x74, 0x79, 0x00, 0x2e, 0x6e, 0x6f, 0x74, 0x65, 0x2e, 0x67, 0x6e, 0x75, 0x2e, 0x62, 0x75, 0x69, 0x6c, 0x64, 0x2d, 0x69, 0x64,
        0x00, 0x2e, 0x67, 0x6e, 0x75, 0x2e, 0x68, 0x61, 0x73, 0x68, 0x00, 0x2e, 0x64, 0x79, 0x6e, 0x73, 0x79, 0x6d, 0x00, 0x2e, 0x64, 0x79, 0x6e, 0x73,
        0x74, 0x72, 0x00, 0x2e, 0x67, 0x6e, 0x75, 0x2e, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x00, 0x2e, 0x67, 0x6e, 0x75, 0x2e, 0x76, 0x65, 0x72,
        0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x72, 0x00, 0x2e, 0x72, 0x65, 0x6c, 0x61, 0x2e, 0x64, 0x79, 0x6e, 0x00, 0x2e, 0x72, 0x65, 0x6c, 0x61, 0x2e, 0x70,
        0x6c, 0x74, 0x00, 0x2e, 0x69, 0x6e, 0x69, 0x74, 0x00, 0x2e, 0x70, 0x6c, 0x74, 0x2e, 0x67, 0x6f, 0x74, 0x00, 0x2e, 0x70, 0x6c, 0x74, 0x2e, 0x73,
        0x65, 0x63, 0x00, 0x2e, 0x74, 0x65, 0x78, 0x74, 0x00, 0x2e, 0x66, 0x69, 0x6e, 0x69, 0x00, 0x2e, 0x72, 0x6f, 0x64, 0x61, 0x74, 0x61, 0x00, 0x2e,
        0x65, 0x68, 0x5f, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x5f, 0x68, 0x64, 0x72, 0x00, 0x2e, 0x65, 0x68, 0x5f, 0x66, 0x72, 0x61, 0x6d, 0x65, 0x00, 0x2e,
        0x69, 0x6e, 0x69, 0x74, 0x5f, 0x61, 0x72, 0x72, 0x61, 0x79, 0x00, 0x2e, 0x66, 0x69, 0x6e, 0x69, 0x5f, 0x61, 0x72, 0x72, 0x61, 0x79, 0x00, 0x2e,
        0x64, 0x61, 0x74, 0x61, 0x2e, 0x72, 0x65, 0x6c, 0x2e, 0x72, 0x6f, 0x00, 0x2e, 0x64, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x00, 0x2e, 0x64, 0x61,
        0x74, 0x61, 0x00, 0x2e, 0x62, 0x73, 0x73, 0x00, 0x2e, 0x67, 0x6e, 0x75, 0x5f, 0x64, 0x65, 0x62, 0x75, 0x67, 0x6c, 0x69, 0x6e, 0x6b, 0x00,
    ];

    // the starting byte of the libvpf string table
    const TEST_TABLE_OFFSET: usize = 0;

    // the length in bytes of the libvpf string table
    const TEST_TABLE_LENGTH: usize = 263;

    // the number of strings in the libvpf string table
    const TEST_TABLE_COUNT: usize = 26;

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
        let layout = file_header.data();
        let width = file_header.class();
        let index = file_header.shstrndx();
        
        // parse all section headers from the buffer
        let section_headers = SectionHeader::parse_all(
            &b,
            count,
            offset,
            layout,
            width);

        assert!(section_headers.is_ok());
        let headers = section_headers.unwrap();

        // find the first string table header
        for (i,section) in headers.into_iter().enumerate() {
            if i == index {
                // build a string table from the section
                let result = StringTable::try_from(section);
                assert!(result.is_ok());
                let mut table = result.unwrap();

                // read the string table from the buffer
                assert!(table.read(&b).is_ok());

                // verify that the string table has expected length
                assert_eq!(table.len(),TEST_TABLE_COUNT);

                // don't process any more section headers
                break;
            }
        }
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
        let result = table.get(1);
        assert!(result.is_some());

        // append a test value to the string
        let mut string = result.unwrap();
        string += TEST_STR;
        assert_eq!(string.as_str(),".shstrtab-test");

        // update the string table with the modified string
        let result = table.set(1,string);
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