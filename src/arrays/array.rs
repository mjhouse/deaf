use crate::errors::{Error, Result};
use crate::common::{Width,Layout,SHType};
use crate::headers::SectionHeader;
use crate::arrays::array_item::ArrayItem;
use crate::tables::common::ByteIter;

pub struct Array {
    offset: usize,
    layout: Layout,
    width: Width,
    kind: SHType,
    entity_size: usize,
    section_size: usize,
    values: Vec<ArrayItem>
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
            // add the address to values
            values.push(ArrayItem::read(
                self.layout,
                self.width,
                data
            )?);
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

    fn get(&self, index: usize) -> Option<&ArrayItem> {
        self.values.get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut ArrayItem> {
        self.values.get_mut(index)
    }

    fn insert(&mut self, index: usize, mut item: ArrayItem) {
        item.set_layout(self.layout);
        item.set_width(self.width);
        self.values.insert(index,item);
    }

    fn push(&mut self, mut item: ArrayItem) {
        item.set_layout(self.layout);
        item.set_width(self.width);
        self.values.push(item);
    }

    fn remove(&mut self, index: usize) -> ArrayItem {
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


#[cfg(test)]
mod tests {
    use super::*;

    use crate::headers::{FileHeader,SectionHeader};

    use crate::utilities::tests::{
        LIBQSCINTILLA_FINI_ARRAY as FINI_TEST,
        LIBQSCINTILLA_INIT_ARRAY as INIT_TEST, 
        read
    };

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
            h.kind() == SHType::SHT_INIT_ARRAY);

        assert!(result.is_some());

        let header = result.unwrap();
        let result = Array::try_from(header);

        assert!(result.is_ok());

        let mut array = result.unwrap();

        assert!(array.read(&b).is_ok());
        assert_eq!(array.len(),INIT_TEST.length);
    }

    #[test]
    fn test_extract_real_fini_array() {
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
            h.kind() == SHType::SHT_FINI_ARRAY);

        assert!(result.is_some());

        let header = result.unwrap();
        let result = Array::try_from(header);

        assert!(result.is_ok());

        let mut array = result.unwrap();

        assert!(array.read(&b).is_ok());
        assert_eq!(array.len(),FINI_TEST.length);
    }

    #[test]
    fn test_read_init_array() {
        
        // directly initialize an array
        let mut array = Array::new(
            0, // because we're reading directly
            INIT_TEST.size,
            Layout::Little,
            Width::X64,
            SHType::SHT_INIT_ARRAY,
            INIT_TEST.entsize
        );

        // read the test array and verify success
        let result = array.read(INIT_TEST.bytes);
        assert!(result.is_ok());

        // verify that the array has the expected number of elements
        assert_eq!(array.len(),INIT_TEST.length);
    }

    #[test]
    fn test_read_fini_array() {
        
        // directly initialize an array
        let mut array = Array::new(
            0, // because we're reading directly
            FINI_TEST.size,
            Layout::Little,
            Width::X64,
            SHType::SHT_FINI_ARRAY,
            FINI_TEST.entsize
        );

        // read the test array and verify success
        let result = array.read(FINI_TEST.bytes);
        assert!(result.is_ok());

        // verify that the array has the expected number of elements
        assert_eq!(array.len(),FINI_TEST.length);
    }

    #[test]
    fn test_write_init_array_with_no_changes() {

        // directly initialize an array
        let mut array = Array::new(
            0, // because we're reading directly
            INIT_TEST.size,
            Layout::Little,
            Width::X64,
            SHType::SHT_INIT_ARRAY,
            INIT_TEST.entsize
        );

        // read the test array and verify success
        let result = array.read(INIT_TEST.bytes);
        assert!(result.is_ok());

        // initialize a buffer big enough for array data
        let mut buffer: Vec<u8> = vec![];
        buffer.resize(array.size(),0x00);

        // write to the new array
        let result = array.write(buffer.as_mut_slice());
        assert!(result.is_ok());

        // verify that the written array is the same as original
        assert_eq!(buffer.as_slice(),INIT_TEST.bytes);
    }

    #[test]
    fn test_write_fini_array_with_no_changes() {

        // directly initialize an array
        let mut array = Array::new(
            0, // because we're reading directly
            FINI_TEST.size,
            Layout::Little,
            Width::X64,
            SHType::SHT_FINI_ARRAY,
            FINI_TEST.entsize
        );

        // read the test array and verify success
        let result = array.read(FINI_TEST.bytes);
        assert!(result.is_ok());

        // initialize a buffer big enough for array data
        let mut buffer: Vec<u8> = vec![];
        buffer.resize(array.size(),0x00);

        // write to the new array
        let result = array.write(buffer.as_mut_slice());
        assert!(result.is_ok());

        // verify that the written array is the same as original
        assert_eq!(buffer.as_slice(),FINI_TEST.bytes);
    }

    #[test]
    fn test_write_init_array_with_changes() {

        // directly initialize an array
        let mut array = Array::new(
            0, // because we're reading directly
            INIT_TEST.size,
            Layout::Little,
            Width::X64,
            SHType::SHT_INIT_ARRAY,
            INIT_TEST.entsize
        );

        // read the test array and verify success
        let result = array.read(INIT_TEST.bytes);
        assert!(result.is_ok());

        // get an element from the array
        let result = array.get_mut(1);
        assert!(result.is_some());

        // change the value of the element
        if let Some(item) = result {
            item.set_value(123);
        }

        // initialize a buffer big enough for modified table data
        let mut buffer: Vec<u8> = vec![];
        buffer.resize(array.size(),0x00);

        // write to the new table
        let result = array.write(buffer.as_mut_slice());
        assert!(result.is_ok());

        // verify that the written table is not the same as original
        assert_ne!(buffer.as_slice(),INIT_TEST.bytes);

        // read the buffer and verify success
        let result = array.read(&buffer);
        assert!(result.is_ok());

        // get an element from the table
        let result = array.get(1);
        assert!(result.is_some());

        // check the element is changed
        let item = result.unwrap();
        assert_eq!(item.value(),Some(123));
    }

    #[test]
    fn test_write_fini_array_with_changes() {

        // directly initialize an array
        let mut array = Array::new(
            0, // because we're reading directly
            FINI_TEST.size,
            Layout::Little,
            Width::X64,
            SHType::SHT_FINI_ARRAY,
            FINI_TEST.entsize
        );

        // read the test array and verify success
        let result = array.read(FINI_TEST.bytes);
        assert!(result.is_ok());

        // get an element from the array
        let result = array.get_mut(0);
        assert!(result.is_some());

        // change the value of the element
        if let Some(item) = result {
            item.set_value(123);
        }

        // initialize a buffer big enough for modified table data
        let mut buffer: Vec<u8> = vec![];
        buffer.resize(array.size(),0x00);

        // write to the new table
        let result = array.write(buffer.as_mut_slice());
        assert!(result.is_ok());

        // verify that the written table is not the same as original
        assert_ne!(buffer.as_slice(),FINI_TEST.bytes);

        // read the buffer and verify success
        let result = array.read(&buffer);
        assert!(result.is_ok());

        // get an element from the table
        let result = array.get(0);
        assert!(result.is_some());

        // check the element is changed
        let item = result.unwrap();
        assert_eq!(item.value(),Some(123));
    }
}