use crate::errors::{Error, Result};
use crate::headers::common::constants::{Width,Layout,SHType};
use crate::headers::section::header::{
    SectionHeader,
    SectionHeaderValues
};
use crate::common::bytes::{FromBytes,IntoBytes};
use crate::tables::common::ByteIter;

use crate::arrays::common::{
    Array,
    ArrayItem,
    constants::*
};

pub struct InitArray {
    offset: usize,
    layout: Layout,
    width: Width,
    entity_size: usize,
    section_size: usize,
    values: Vec<i64>
}

impl InitArray {

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

        // reserve a temporary buffer for entities
        let mut values = vec![];
        values.reserve(self.section_size / size);

        for data in ByteIter::length(&bytes[start..end],size) {            
            // parse an address from the byte slice
            let value = i64::read(data,self.width,self.layout)?;

            // add the address to values
            values.push(value);
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

        // iterate all contained addresses
        for (i,&value) in self.values.iter().enumerate() {
            
            // calculate item position in the output buffer
            let start = i * size;
            let end = start + size;

            // get a constrained, mutable slice of bytes to write to
            let buffer = &mut bytes[start..end];

            // write the item to the byte slice
            value.write(buffer,self.width,self.layout)?;
        }

        Ok(self.values.len())
    }

    pub fn size(&self) -> usize {
        self.entity_size * self.values.len()
    }

}

impl Array<i64> for InitArray {

    fn len(&self) -> usize {
        self.values.len()
    }

    fn size(&self) -> usize {
        self.len() * self.entity_size
    }

    fn get(&self, index: usize) -> Option<&i64> {
        self.values.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut i64> {
        self.values.get_mut(index)
    }

    fn insert(&mut self, index: usize, item: i64) {
        self.values.insert(index,item);
    }

    fn push(&mut self, item: i64) {
        self.values.push(item);
    }

    fn remove(&mut self, index: usize) -> i64 {
        self.values.remove(index)
    }

}

impl TryFrom<&SectionHeader> for InitArray {
    type Error = Error;

    fn try_from(header: &SectionHeader) -> Result<Self> {
        match header.values.sh_type {
            SHType::SHT_INIT_ARRAY => Ok(Self::new(
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

impl TryFrom<SectionHeader> for InitArray {
    type Error = Error;

    fn try_from(header: SectionHeader) -> Result<Self> {
        Self::try_from(&header)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::headers::file::header::FileHeader;
    use crate::headers::section::header::SectionHeader;

    use crate::utilities::tests::{LIBQSCINTILLA_INIT_ARRAY as TEST, read};

    #[test]
    fn test_extract_real_init_array() {
        let b = read("assets/libqscintilla2/libqscintilla2_qt5.so.15.0.0"); 

        let file_header = FileHeader::parse(&b)
            .unwrap();

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
            h.section_type() == SHType::SHT_INIT_ARRAY);

        assert!(result.is_some());

        let header = result.unwrap();
        let result = InitArray::try_from(header);

        assert!(result.is_ok());

        let mut array = result.unwrap();

        assert!(array.read(&b).is_ok());
        assert_eq!(array.len(),TEST.length);
    }

    #[test]
    fn test_read_init_array() {
        
        // directly initialize an array
        let mut array = InitArray::new(
            0, // because we're reading directly
            TEST.size,
            Layout::Little,
            Width::X64,
            TEST.entsize
        );

        // read the test array and verify success
        let result = array.read(TEST.bytes);
        assert!(result.is_ok());

        // verify that the array has the expected number of elements
        assert_eq!(array.len(),TEST.length);
    }

    #[test]
    fn test_write_init_array_with_no_changes() {

        // directly initialize an array
        let mut array = InitArray::new(
            0, // because we're reading directly
            TEST.size,
            Layout::Little,
            Width::X64,
            TEST.entsize
        );

        // read the test array and verify success
        let result = array.read(TEST.bytes);
        assert!(result.is_ok());

        // initialize a buffer big enough for array data
        let mut buffer: Vec<u8> = vec![];
        buffer.resize(array.size(),0x00);

        // write to the new array
        let result = array.write(buffer.as_mut_slice());
        assert!(result.is_ok());

        // verify that the written array is the same as original
        assert_eq!(buffer.as_slice(),TEST.bytes);
    }

    #[test]
    fn test_write_init_array_with_changes() {

        // directly initialize an array
        let mut array = InitArray::new(
            0, // because we're reading directly
            TEST.size,
            Layout::Little,
            Width::X64,
            TEST.entsize
        );

        // read the test array and verify success
        let result = array.read(TEST.bytes);
        assert!(result.is_ok());

        // remove an element from the array
        let result = array.remove(1);
        assert_eq!(result,0xa2ed0);

        // insert an element at that index
        array.insert(1,123);

        // initialize a buffer big enough for modified table data
        let mut buffer: Vec<u8> = vec![];
        buffer.resize(array.size(),0x00);

        // write to the new table
        let result = array.write(buffer.as_mut_slice());
        assert!(result.is_ok());

        // verify that the written table is not the same as original
        assert_ne!(buffer.as_slice(),TEST.bytes);

        // read the buffer and verify success
        let result = array.read(&buffer);
        assert!(result.is_ok());

        // get an element from the table
        let result = array.get(1);
        assert!(result.is_some());

        // check the element is changed
        let value = result.unwrap();
        assert_eq!(value,&123);
    }
}