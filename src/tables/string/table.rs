use crate::errors::{Error, Result};
use crate::headers::section::header::{
    SectionHeader,
    SectionHeaderValues
};
use crate::headers::common::constants::{
    SHType
};
use crate::tables::common::ByteIter;
use std::ffi::CString;

pub struct StringTable {
    offset: usize,
    entity_size: usize,
    section_size: usize,
    values: Vec<String>
}

impl StringTable {

    pub fn new(header: SectionHeader) -> Result<Self> {
        match header.values.sh_type {
            SHType::SHT_STRTAB => Ok(Self {
                offset: header.values.sh_offset,
                entity_size: header.values.sh_entsize,
                section_size: header.values.sh_size,
                values: vec![],
            }),
            _ => Err(Error::WrongSectionError)
        }
    }

    pub fn read(&mut self, b: &[u8]) -> Vec<String> {
        let start = self.offset;
        let end = start + self.section_size;

        self.values = ByteIter::value(&b[start..end],b'\0')
            .filter_map(|d| std::str::from_utf8(d).ok())
            .map(|s| s.into())
            .collect();

        self.values.clone()
    }

    pub fn write(&self, bytes: &mut [u8]) -> Result<usize> {
        let section_size = self.size();
        let section_start = self.offset;
        let section_end = self.offset + section_size;

        // check buffer is big enough
        if bytes.len() > section_end {
            return Err(Error::OutOfBoundsError);
        }

        let mut string_start = section_start;

        // iterate all contained strings
        for string in self.values.iter() {
            // calculate end position in the output buffer
            let string_end = string_start + string.len();

            // get a constrained, mutable slice of bytes to write to
            let buffer = &mut bytes[string_start..string_end];

            // convert to nul-terminated c-string representation
            let cstr = CString::new(string.as_bytes())?;

            // copy the string to the byte slice
            buffer.clone_from_slice(cstr.as_bytes_with_nul());

            string_start = string_end;
        }

        Ok(self.values.len())
    }

    pub fn size(&self) -> usize {
        // +1 for null terminator
        self.values
            .iter()
            .fold(0,|a,v| a + v.len() + 1)
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
    fn test_extract_shstrtab_section_as_table() {
        let mut f = File::open("assets/libvpf.so.4.1").unwrap();
        let mut b = Vec::new();

        // the number of strings in the string table
        const STR_COUNT: usize = 26;
        
        f.read_to_end(&mut b)
            .unwrap();

        let file_header = FileHeader::parse(&b)
            .unwrap();

        let count = file_header.shnum();
        let offset = file_header.shoff();
        let layout = file_header.data();
        let width = file_header.class();
        let index = file_header.shstrndx();
        
        let section_headers = SectionHeader::parse_all(
            &b,
            count,
            offset,
            layout,
            width);

        assert!(section_headers.is_ok());
        let headers = section_headers.unwrap();

        for (i,section) in headers.into_iter().enumerate() {
            if i == index {
                let mut table = StringTable::new(section).unwrap();
                let result = table.read(&b);
                assert_eq!(result.len(),STR_COUNT);
                break;
            }
        }

    }
}