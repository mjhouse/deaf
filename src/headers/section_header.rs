use crate::common::{
    Width,
    Layout,
    SHType,
    SHFlags
};
use enumflags2::BitFlags;

use crate::common::field::Field;
use crate::common::ranges::*;
use crate::errors::{Error, Result};
use crate::impl_property;

#[derive(Debug,Clone)]
pub struct SectionHeaderValues {
    pub size: usize, 
    pub sh_name: u32,
    pub sh_type: SHType,
    pub sh_flags: BitFlags<SHFlags>,
    pub sh_address: u64,
    pub sh_offset: usize,
    pub sh_size: usize,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: usize,
}

#[derive(Debug)]
pub struct SectionHeader {
    offset: usize,
    layout: Layout,
    width: Width,
    size: usize, 

    sh_name: Field<u32>,
    sh_type: Field<u32,u32,SHType>,
    sh_flags: Field<u32,u64,BitFlags<SHFlags>>,
    sh_address: Field<u32,u64>,
    sh_offset: Field<u32,u64,usize>,
    sh_size: Field<u32,u64,usize>,
    sh_link: Field<u32>,
    sh_info: Field<u32>,
    sh_addralign: Field<u32,u64>,
    sh_entsize: Field<u32,u64,usize>,

    pub values: SectionHeaderValues,
}

impl SectionHeaderValues {

    pub fn new() -> Self {
        Self {
            size: 0, 
            sh_name: 0,
            sh_type: SHType::SHT_NULL,
            sh_flags: BitFlags::EMPTY,
            sh_address: 0,
            sh_offset: 0,
            sh_size: 0,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 0,
            sh_entsize: 0,
        }
    }
    
}

impl SectionHeader {

    pub fn new(offset: usize, size: usize, layout: Layout, width: Width) -> Self {
        Self {
            offset,
            layout,
            width,
            size,
            sh_name: Field::new(SH_NAME),
            sh_type: Field::new(SH_TYPE),
            sh_flags: Field::new(SH_FLAGS),
            sh_address: Field::new(SH_ADDR),
            sh_offset: Field::new(SH_OFFSET),
            sh_size: Field::new(SH_SIZE),
            sh_link: Field::new(SH_LINK),
            sh_info: Field::new(SH_INFO),
            sh_addralign: Field::new(SH_ADDRALIGN),
            sh_entsize: Field::new(SH_ENTSIZE),

            values: SectionHeaderValues::new(),
        }
    }

    pub fn parse(b: &[u8], offset: usize, size: usize, layout: Layout, width: Width) -> Result<Self> {
        let mut header = Self::new(offset,size,layout,width);
        header.read(b)?;
        Ok(header)
    }

    pub fn parse_all(b: &[u8], count: usize, offset: usize, size: usize, layout: Layout, width: Width) -> Result<Vec<Self>> {
        let mut result = vec![];
        result.reserve_exact(count);

        for i in 0..count {
            let offs = offset + i * size;
            result.push(Self::parse(
                &b[offs..],
                offs,
                size,
                layout,
                width)?);
        }

        Ok(result)
    }

    pub fn set_width(&mut self, width: Width) {
        self.width = width;
        self.sh_name.ranges.width = width;
        self.sh_type.ranges.width = width;
        self.sh_flags.ranges.width = width;
        self.sh_address.ranges.width = width;
        self.sh_offset.ranges.width = width;
        self.sh_size.ranges.width = width;
        self.sh_link.ranges.width = width;
        self.sh_info.ranges.width = width;
        self.sh_addralign.ranges.width = width;
        self.sh_entsize.ranges.width = width;
    }

    pub fn set_layout(&mut self, layout: Layout) {
        self.layout = layout;
        self.sh_name.layout = layout;
        self.sh_type.layout = layout;
        self.sh_flags.layout = layout;
        self.sh_address.layout = layout;
        self.sh_offset.layout = layout;
        self.sh_size.layout = layout;
        self.sh_link.layout = layout;
        self.sh_info.layout = layout;
        self.sh_addralign.layout = layout;
        self.sh_entsize.layout = layout;
    }

    pub fn read(&mut self, b: &[u8]) -> Result<SectionHeaderValues> {
        self.set_layout(self.layout);
        self.set_width(self.width);

        self.values.sh_name      = self.sh_name.get(b)?;
        self.values.sh_type      = self.sh_type.get(b)?;
        self.values.sh_flags     = self.sh_flags.get(b)?;
        self.values.sh_address   = self.sh_address.get(b)?;
        self.values.sh_offset    = self.sh_offset.get(b)?;
        self.values.sh_size      = self.sh_size.get(b)?;
        self.values.sh_link      = self.sh_link.get(b)?;
        self.values.sh_info      = self.sh_info.get(b)?;
        self.values.sh_addralign = self.sh_addralign.get(b)?;
        self.values.sh_entsize   = self.sh_entsize.get(b)?;

        Ok(self.values.clone())
    }

    pub fn write(&self, b: &mut [u8]) -> Result<()> {
        self.sh_name.set(b,self.values.sh_name)?;
        self.sh_type.set(b,self.values.sh_type)?;
        self.sh_flags.set(b,self.values.sh_flags)?;
        self.sh_address.set(b,self.values.sh_address)?;
        self.sh_offset.set(b,self.values.sh_offset)?;
        self.sh_size.set(b,self.values.sh_size)?;
        self.sh_link.set(b,self.values.sh_link)?;
        self.sh_info.set(b,self.values.sh_info)?;
        self.sh_addralign.set(b,self.values.sh_addralign)?;
        self.sh_entsize.set(b,self.values.sh_entsize)?;
        Ok(())
    }

    pub fn body<'a>(&self, b: &'a [u8]) -> Result<&'a [u8]> {
        let start = self.offset;
        let end = start + self.values.sh_size;

        if end < b.len() {
            Ok(&b[start..end])
        } else {
            Err(Error::OutOfBoundsError)
        }
    }

    pub fn header_size(&self) -> usize {
        self.sh_name.size() +
        self.sh_type.size() +
        self.sh_flags.size() +
        self.sh_address.size() +
        self.sh_offset.size() +
        self.sh_size.size() +
        self.sh_link.size() +
        self.sh_info.size() +
        self.sh_addralign.size() +
        self.sh_entsize.size()
    }

    pub fn layout(&self) -> Layout {
        self.layout.clone()
    }

    pub fn width(&self) -> Width {
        self.width.clone()
    }

    impl_property!(name,sh_name,u32);
    impl_property!(kind,sh_type,SHType);
    impl_property!(flags,sh_flags,BitFlags<SHFlags>);
    impl_property!(address,sh_address,u64);
    impl_property!(offset,sh_offset,usize);
    impl_property!(size,sh_size,usize);
    impl_property!(link,sh_link,u32);
    impl_property!(info,sh_info,u32);
    impl_property!(addralign,sh_addralign,u64);
    impl_property!(entsize,sh_entsize,usize);

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::headers::FileHeader;

    use crate::utilities::tests::read;

    #[test]
    fn test_read_section_headers() {
        let b = read("assets/libvpf/libvpf.so.4.1");

        // get the file header to find section headers
        let file_header = FileHeader::parse(&b)
            .unwrap();

        let count = file_header.shnum();
        let offset = file_header.shoff();
        let size = file_header.shentsize();
        let layout = file_header.data();
        let width = file_header.class();
        
        // parse all section headers in file
        let section_headers = SectionHeader::parse_all(
            &b,
            count,
            offset,
            size,
            layout,
            width);

        // get the first section header for testing
        assert!(section_headers.is_ok());
        let headers = section_headers.unwrap();

        let header = &headers[3];

        // check values are what we expected
        assert_eq!(header.header_size(),size);
        assert_eq!(header.size(),0x7c4);
        assert_eq!(header.offset(),0x2f0);
    }

    #[test]
    fn test_write_section_header_with_no_changes() {
        let b = read("assets/libvpf/libvpf.so.4.1");

        // get the file header to find section headers
        let file_header = FileHeader::parse(&b)
            .unwrap();

        let count = file_header.shnum();
        let offset = file_header.shoff();
        let size = file_header.shentsize();
        let layout = file_header.data();
        let width = file_header.class();
        
        // parse all section headers in file
        let section_headers = SectionHeader::parse_all(
            &b,
            count,
            offset,
            size,
            layout,
            width);

        // get the first section header for testing
        assert!(section_headers.is_ok());
        let mut headers = section_headers.unwrap();

        let header = &mut headers[0];

        // initialize a buffer big enough for the header
        let mut buffer: Vec<u8> = vec![];
        buffer.resize(header.header_size(),0x00);        

        // write to the new buffer
        let result = header.write(buffer.as_mut_slice());
        assert!(result.is_ok());

        // get the expected result from the original buffer
        let expected = &b[offset..offset + header.header_size()];

        // verify that the written header is the same as original
        assert_eq!(buffer.as_slice(),expected);
    }

    #[test]
    fn test_write_section_header_with_changes() {
        let b = read("assets/libvpf/libvpf.so.4.1");

        // get the file header to find section headers
        let file_header = FileHeader::parse(&b)
            .unwrap();

        let count = file_header.shnum();
        let offset = file_header.shoff();
        let size = file_header.shentsize();
        let layout = file_header.data();
        let width = file_header.class();
        
        // parse all section headers in file
        let section_headers = SectionHeader::parse_all(
            &b,
            count,
            offset,
            size,
            layout,
            width);

        // get the first section header for testing
        assert!(section_headers.is_ok());
        let mut headers = section_headers.unwrap();

        let header = &mut headers[0];

        // change a field in the section header
        header.set_address(123);

        // initialize a buffer big enough for the header
        let mut buffer: Vec<u8> = vec![];
        buffer.resize(header.header_size(),0x00);        

        // write to the new buffer
        let result = header.write(buffer.as_mut_slice());
        assert!(result.is_ok());

        // get the expected result from the original buffer
        let expected = &b[offset..offset + header.header_size()];

        // verify that the written header is the same as original
        assert_ne!(buffer.as_slice(),expected);

        // read the modified data back from the buffer
        let result = header.read(&buffer);
        assert!(result.is_ok());

        // check that the re-parsed header has changed value
        assert_eq!(header.address(),123);
    }
}