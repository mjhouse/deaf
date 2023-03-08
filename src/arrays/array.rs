use crate::errors::{Error, Result};
use crate::headers::common::constants::{Width,Layout,SHType};
use crate::common::{Ranges,ranges::ADDRESS};
use crate::headers::section::header::{
    SectionHeader,
    SectionHeaderValues
};
use crate::common::{FromBytes,IntoBytes,Field,Item};
use crate::arrays::arrayitem::ArrayItem;
use crate::tables::common::ByteIter;

type Address = Item<i32,i64>;

pub struct Array {
    offset: usize,
    layout: Layout,
    width: Width,
    kind: SHType,
    entity_size: usize,
    section_size: usize,
    values: Vec<Address>
}

impl Array {

    pub fn new(offset: usize, size: usize, layout: Layout, width: Width, kind: SHType, entity_size: usize) -> Self {
        Self {
            offset: offset,
            layout: layout,
            width: width,
            kind: kind,
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
            let item = Item::new(ADDRESS)
                .with_width(self.width)
                .with_layout(self.layout)
                .parse(data)?;

            // let value = i64::read(data,self.width,self.layout)?;

            // add the address to values
            values.push(item);
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
        for (i,value) in self.values.iter().enumerate() {
            
            // calculate item position in the output buffer
            let start = i * size;
            let end = start + size;

            // get a constrained, mutable slice of bytes to write to
            let buffer = &mut bytes[start..end];

            // write the item to the byte slice
            value.write(buffer)?;
        }

        Ok(self.values.len())
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    fn size(&self) -> usize {
        self.len() * self.entity_size
    }

    fn get(&self, index: usize) -> Option<&Address> {
        self.values.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut Address> {
        self.values.get_mut(index)
    }

    fn insert(&mut self, index: usize, item: Address) {
        self.values.insert(index,item);
    }

    fn push(&mut self, item: Address) {
        self.values.push(item);
    }

    fn remove(&mut self, index: usize) -> Address {
        self.values.remove(index)
    }

}

impl TryFrom<&SectionHeader> for Array {
    type Error = Error;

    fn try_from(header: &SectionHeader) -> Result<Self> {
        match header.values.sh_type {
            SHType::SHT_INIT_ARRAY | SHType::SHT_PREINIT_ARRAY | SHType::SHT_FINI_ARRAY 
            => Ok(Self::new(
                header.offset(),
                header.size(),
                header.layout(),
                header.width(),
                header.kind(),
                header.entsize()
            )),
            _ => Err(Error::WrongSectionError)
        }
    }
}

impl TryFrom<SectionHeader> for Array {
    type Error = Error;

    fn try_from(header: SectionHeader) -> Result<Self> {
        Self::try_from(&header)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use crate::headers::file::header::FileHeader;
//     use crate::headers::section::header::SectionHeader;

//     use crate::utilities::tests::{
//         LIBQSCINTILLA_FINI_ARRAY as FINI_TEST,
//         LIBQSCINTILLA_INIT_ARRAY as INIT_TEST, 
//         read
//     };

//     #[test]
//     fn test_extract_real_init_array() {
//         let b = read("assets/libqscintilla2/libqscintilla2_qt5.so.15.0.0"); 

//         let file_header = FileHeader::parse(&b)
//             .unwrap();

//         let count = file_header.shnum();
//         let offset = file_header.shoff();
//         let size = file_header.shentsize();
//         let layout = file_header.data();
//         let width = file_header.class();
        
//         let section_headers = SectionHeader::parse_all(
//             &b,
//             count,
//             offset,
//             size,
//             layout,
//             width);

//         assert!(section_headers.is_ok());
//         let headers = section_headers.unwrap();

//         let result = headers.iter().find(|&h| 
//             h.kind() == SHType::SHT_INIT_ARRAY);

//         assert!(result.is_some());

//         let header = result.unwrap();
//         let result = Array::try_from(header);

//         assert!(result.is_ok());

//         let mut array = result.unwrap();

//         assert!(array.read(&b).is_ok());
//         assert_eq!(array.len(),INIT_TEST.length);
//     }

//     #[test]
//     fn test_extract_real_fini_array() {
//         let b = read("assets/libqscintilla2/libqscintilla2_qt5.so.15.0.0"); 

//         let file_header = FileHeader::parse(&b)
//             .unwrap();

//         let count = file_header.shnum();
//         let offset = file_header.shoff();
//         let size = file_header.shentsize();
//         let layout = file_header.data();
//         let width = file_header.class();
        
//         let section_headers = SectionHeader::parse_all(
//             &b,
//             count,
//             offset,
//             size,
//             layout,
//             width);

//         assert!(section_headers.is_ok());
//         let headers = section_headers.unwrap();

//         let result = headers.iter().find(|&h| 
//             h.kind() == SHType::SHT_FINI_ARRAY);

//         assert!(result.is_some());

//         let header = result.unwrap();
//         let result = Array::try_from(header);

//         assert!(result.is_ok());

//         let mut array = result.unwrap();

//         assert!(array.read(&b).is_ok());
//         assert_eq!(array.len(),FINI_TEST.length);
//     }

//     #[test]
//     fn test_read_init_array() {
        
//         // directly initialize an array
//         let mut array = Array::new(
//             0, // because we're reading directly
//             INIT_TEST.size,
//             Layout::Little,
//             Width::X64,
//             SHType::SHT_INIT_ARRAY,
//             INIT_TEST.entsize
//         );

//         // read the test array and verify success
//         let result = array.read(INIT_TEST.bytes);
//         assert!(result.is_ok());

//         // verify that the array has the expected number of elements
//         assert_eq!(array.len(),INIT_TEST.length);
//     }

//     #[test]
//     fn test_read_fini_array() {
        
//         // directly initialize an array
//         let mut array = Array::new(
//             0, // because we're reading directly
//             FINI_TEST.size,
//             Layout::Little,
//             Width::X64,
//             SHType::SHT_FINI_ARRAY,
//             FINI_TEST.entsize
//         );

//         // read the test array and verify success
//         let result = array.read(FINI_TEST.bytes);
//         assert!(result.is_ok());

//         // verify that the array has the expected number of elements
//         assert_eq!(array.len(),FINI_TEST.length);
//     }

//     #[test]
//     fn test_write_init_array_with_no_changes() {

//         // directly initialize an array
//         let mut array = Array::new(
//             0, // because we're reading directly
//             INIT_TEST.size,
//             Layout::Little,
//             Width::X64,
//             SHType::SHT_INIT_ARRAY,
//             INIT_TEST.entsize
//         );

//         // read the test array and verify success
//         let result = array.read(INIT_TEST.bytes);
//         assert!(result.is_ok());

//         // initialize a buffer big enough for array data
//         let mut buffer: Vec<u8> = vec![];
//         buffer.resize(array.size(),0x00);

//         // write to the new array
//         let result = array.write(buffer.as_mut_slice());
//         assert!(result.is_ok());

//         // verify that the written array is the same as original
//         assert_eq!(buffer.as_slice(),INIT_TEST.bytes);
//     }

//     #[test]
//     fn test_write_fini_array_with_no_changes() {

//         // directly initialize an array
//         let mut array = Array::new(
//             0, // because we're reading directly
//             FINI_TEST.size,
//             Layout::Little,
//             Width::X64,
//             SHType::SHT_FINI_ARRAY,
//             FINI_TEST.entsize
//         );

//         // read the test array and verify success
//         let result = array.read(FINI_TEST.bytes);
//         assert!(result.is_ok());

//         // initialize a buffer big enough for array data
//         let mut buffer: Vec<u8> = vec![];
//         buffer.resize(array.size(),0x00);

//         // write to the new array
//         let result = array.write(buffer.as_mut_slice());
//         assert!(result.is_ok());

//         // verify that the written array is the same as original
//         assert_eq!(buffer.as_slice(),FINI_TEST.bytes);
//     }

//     #[test]
//     fn test_write_init_array_with_changes() {

//         // directly initialize an array
//         let mut array = Array::new(
//             0, // because we're reading directly
//             INIT_TEST.size,
//             Layout::Little,
//             Width::X64,
//             SHType::SHT_INIT_ARRAY,
//             INIT_TEST.entsize
//         );

//         // read the test array and verify success
//         let result = array.read(INIT_TEST.bytes);
//         assert!(result.is_ok());

//         // remove an element from the array
//         let result = array.remove(1);
//         assert_eq!(result,0xa2ed0);

//         // insert an element at that index
//         array.insert(1,123);

//         // initialize a buffer big enough for modified table data
//         let mut buffer: Vec<u8> = vec![];
//         buffer.resize(array.size(),0x00);

//         // write to the new table
//         let result = array.write(buffer.as_mut_slice());
//         assert!(result.is_ok());

//         // verify that the written table is not the same as original
//         assert_ne!(buffer.as_slice(),INIT_TEST.bytes);

//         // read the buffer and verify success
//         let result = array.read(&buffer);
//         assert!(result.is_ok());

//         // get an element from the table
//         let result = array.get(1);
//         assert!(result.is_some());

//         // check the element is changed
//         let value = result.unwrap();
//         assert_eq!(value,&123);
//     }

//     #[test]
//     fn test_write_fini_array_with_changes() {

//         // directly initialize an array
//         let mut array = Array::new(
//             0, // because we're reading directly
//             FINI_TEST.size,
//             Layout::Little,
//             Width::X64,
//             SHType::SHT_FINI_ARRAY,
//             FINI_TEST.entsize
//         );

//         // read the test array and verify success
//         let result = array.read(FINI_TEST.bytes);
//         assert!(result.is_ok());

//         // remove an element from the array
//         let result = array.remove(0);
//         assert_eq!(result,0x0a5b70);

//         // insert an element at that index
//         array.insert(0,123);

//         // initialize a buffer big enough for modified table data
//         let mut buffer: Vec<u8> = vec![];
//         buffer.resize(array.size(),0x00);

//         // write to the new table
//         let result = array.write(buffer.as_mut_slice());
//         assert!(result.is_ok());

//         // verify that the written table is not the same as original
//         assert_ne!(buffer.as_slice(),FINI_TEST.bytes);

//         // read the buffer and verify success
//         let result = array.read(&buffer);
//         assert!(result.is_ok());

//         // get an element from the table
//         let result = array.get(0);
//         assert!(result.is_some());

//         // check the element is changed
//         let value = result.unwrap();
//         assert_eq!(value,&123);
//     }
// }