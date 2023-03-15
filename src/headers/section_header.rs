use crate::common::{
    Width,
    Layout,
    SHType,
    SHFlags
};
use enumflags2::BitFlags;

use crate::common::Item;
use crate::common::ranges::*;
use crate::errors::Result;

#[derive(Debug)]
pub struct SectionHeader {
    layout: Layout,
    width: Width,
    sh_name: Item<u32>,
    sh_type: Item<u32,u32,SHType>,
    sh_flags: Item<u32,u64,BitFlags<SHFlags>>,
    sh_address: Item<u32,u64>,
    sh_offset: Item<u32,u64,usize>,
    sh_size: Item<u32,u64,usize>,
    sh_link: Item<u32>,
    sh_info: Item<u32>,
    sh_addralign: Item<u32,u64>,
    sh_entsize: Item<u32,u64,usize>,
}

impl SectionHeader {

    /// Create a new header with given Layout and Width
    ///
    /// All fields are None until read
    pub fn new(layout: Layout, width: Width) -> Self {
        Self {
            layout,
            width,
            sh_name: Item::make(SH_NAME,width,layout),
            sh_type: Item::make(SH_TYPE,width,layout),
            sh_flags: Item::make(SH_FLAGS,width,layout),
            sh_address: Item::make(SH_ADDR,width,layout),
            sh_offset: Item::make(SH_OFFSET,width,layout),
            sh_size: Item::make(SH_SIZE,width,layout),
            sh_link: Item::make(SH_LINK,width,layout),
            sh_info: Item::make(SH_INFO,width,layout),
            sh_addralign: Item::make(SH_ADDRALIGN,width,layout),
            sh_entsize: Item::make(SH_ENTSIZE,width,layout),
        }
    }

    /// Parse a header from the provided byte buffer
    pub fn parse(b: &[u8], layout: Layout, width: Width) -> Result<Self> {
        let mut header = Self::new(layout,width);
        header.read(b)?;
        Ok(header)
    }

    /// Parse all headers for a byte array given count, offset etc.
    pub fn parse_all(b: &[u8], count: usize, offset: usize, size: usize, layout: Layout, width: Width) -> Result<Vec<Self>> {
        let mut result = vec![];
        result.reserve_exact(count);

        for i in 0..count {
            let start = offset + i * size;
            result.push(Self::parse(
                &b[start..],
                layout,
                width)?);
        }

        Ok(result)
    }

    /// Read values from a byte buffer 
    ///
    /// Byte buffer is assumed to be sliced such that the
    /// header is at the beginning of the buffer.
    pub fn read(&mut self, b: &[u8]) -> Result<()> {
        self.sh_name.read(b)?;
        self.sh_type.read(b)?;
        self.sh_flags.read(b)?;
        self.sh_address.read(b)?;
        self.sh_offset.read(b)?;
        self.sh_size.read(b)?;
        self.sh_link.read(b)?;
        self.sh_info.read(b)?;
        self.sh_addralign.read(b)?;
        self.sh_entsize.read(b)?;
        Ok(())
    }

    /// Write values to a byte buffer 
    ///
    /// Byte buffer is assumed to be sliced such that the
    /// header will be written at the correct position.
    pub fn write(&self, b: &mut [u8]) -> Result<()> {
        self.sh_name.write(b)?;
        self.sh_type.write(b)?;
        self.sh_flags.write(b)?;
        self.sh_address.write(b)?;
        self.sh_offset.write(b)?;
        self.sh_size.write(b)?;
        self.sh_link.write(b)?;
        self.sh_info.write(b)?;
        self.sh_addralign.write(b)?;
        self.sh_entsize.write(b)?;
        Ok(())
    }

    /// The size of the header in bytes
    pub fn size(&self) -> usize {
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

    /// Get the width (32 or 64-bit) of the header
    pub fn width(&self) -> Width {
        self.width
    }

    /// Set the width of the header
    pub fn set_width(&mut self, width: Width) {
        self.width = width;
        self.sh_name.set_width(width);
        self.sh_type.set_width(width);
        self.sh_flags.set_width(width);
        self.sh_address.set_width(width);
        self.sh_offset.set_width(width);
        self.sh_size.set_width(width);
        self.sh_link.set_width(width);
        self.sh_info.set_width(width);
        self.sh_addralign.set_width(width);
        self.sh_entsize.set_width(width);
    }

    /// Get the layout (little or big-endian) of the header
    pub fn layout(&self) -> Layout {
        self.layout
    }

    /// Set the layout of the header
    pub fn set_layout(&mut self, layout: Layout) {
        self.layout = layout;
        self.sh_name.set_layout(layout);
        self.sh_type.set_layout(layout);
        self.sh_flags.set_layout(layout);
        self.sh_address.set_layout(layout);
        self.sh_offset.set_layout(layout);
        self.sh_size.set_layout(layout);
        self.sh_link.set_layout(layout);
        self.sh_info.set_layout(layout);
        self.sh_addralign.set_layout(layout);
        self.sh_entsize.set_layout(layout);
    }

    /// Get the `sh_name` attribute of the header
    pub fn name(&self) -> Option<u32> {
        self.sh_name.get()
    }

    /// Set the `sh_name` attribute of the header 
    pub fn set_name(&mut self, name: u32) {
        self.sh_name.set(name);
    }

    /// Get the `sh_type` attribute of the header
    pub fn kind(&self) -> Option<SHType> {
        self.sh_type.get()
    }

    /// Set the `sh_type` attribute of the header 
    pub fn set_kind(&mut self, kind: SHType) {
        self.sh_type.set(kind);
    }

    /// Get the `sh_flags` attribute of the header
    pub fn flags(&self) -> Option<BitFlags<SHFlags>> {
        self.sh_flags.get()
    }

    /// Set the `sh_flags` attribute of the header 
    pub fn set_flags(&mut self, flags: BitFlags<SHFlags>) {
        self.sh_flags.set(flags);
    }

    /// Get the `sh_address` attribute of the header
    pub fn address(&self) -> Option<u64> {
        self.sh_address.get()
    }

    /// Set the `sh_address` attribute of the header 
    pub fn set_address(&mut self, address: u64) {
        self.sh_address.set(address);
    }

    /// Get the `sh_offset` attribute of the header
    pub fn offset(&self) -> Option<usize> {
        self.sh_offset.get()
    }

    /// Set the `sh_offset` attribute of the header 
    pub fn set_offset(&mut self, offset: usize) {
        self.sh_offset.set(offset);
    }

    /// Get the `sh_size` attribute of the header
    pub fn body_size(&self) -> Option<usize> {
        self.sh_size.get()
    }

    /// Set the `sh_size` attribute of the header 
    pub fn set_body_size(&mut self, body_size: usize) {
        self.sh_size.set(body_size);
    }

    /// Get the `sh_link` attribute of the header
    pub fn link(&self) -> Option<u32> {
        self.sh_link.get()
    }

    /// Set the `sh_link` attribute of the header 
    pub fn set_link(&mut self, link: u32) {
        self.sh_link.set(link);
    }

    /// Get the `sh_info` attribute of the header
    pub fn info(&self) -> Option<u32> {
        self.sh_info.get()
    }

    /// Set the `sh_info` attribute of the header 
    pub fn set_info(&mut self, info: u32) {
        self.sh_info.set(info);
    }

    /// Get the `sh_addralign` attribute of the header
    pub fn addralign(&self) -> Option<u64> {
        self.sh_addralign.get()
    }

    /// Set the `sh_addralign` attribute of the header 
    pub fn set_addralign(&mut self, addralign: u64) {
        self.sh_addralign.set(addralign);
    }

    /// Get the `sh_entsize` attribute of the header
    pub fn entsize(&self) -> Option<usize> {
        self.sh_entsize.get()
    }

    /// Set the `sh_entsize` attribute of the header 
    pub fn set_entsize(&mut self, entsize: usize) {
        self.sh_entsize.set(entsize);
    }

    // pub fn body<'a>(&self, b: &'a [u8]) -> Result<&'a [u8]> {
    //     let start = self.offset;
    //     let end = start + self.values.sh_size;

    //     if end < b.len() {
    //         Ok(&b[start..end])
    //     } else {
    //         Err(Error::OutOfBoundsError)
    //     }
    // }
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

        let count = file_header.shnum().unwrap();
        let offset = file_header.shoff().unwrap();
        let size = file_header.shentsize().unwrap();
        let layout = file_header.data().unwrap();
        let width = file_header.class().unwrap();
        
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
        assert_eq!(header.size(),size);
        assert_eq!(header.body_size(),Some(0x7c4));
        assert_eq!(header.offset(),Some(0x2f0));
    }

    #[test]
    fn test_write_section_header_with_no_changes() {
        let b = read("assets/libvpf/libvpf.so.4.1");

        // get the file header to find section headers
        let file_header = FileHeader::parse(&b)
            .unwrap();

        let count = file_header.shnum().unwrap();
        let offset = file_header.shoff().unwrap();
        let size = file_header.shentsize().unwrap();
        let layout = file_header.data().unwrap();
        let width = file_header.class().unwrap();
        
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
        buffer.resize(header.size(),0x00);        

        // write to the new buffer
        let result = header.write(buffer.as_mut_slice());
        assert!(result.is_ok());

        // get the expected result from the original buffer
        let expected = &b[offset..offset + header.size()];

        // verify that the written header is the same as original
        assert_eq!(buffer.as_slice(),expected);
    }

    #[test]
    fn test_write_section_header_with_changes() {
        let b = read("assets/libvpf/libvpf.so.4.1");

        // get the file header to find section headers
        let file_header = FileHeader::parse(&b)
            .unwrap();

        let count = file_header.shnum().unwrap();
        let offset = file_header.shoff().unwrap();
        let size = file_header.shentsize().unwrap();
        let layout = file_header.data().unwrap();
        let width = file_header.class().unwrap();
        
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
        buffer.resize(header.size(),0x00);        

        // write to the new buffer
        let result = header.write(buffer.as_mut_slice());
        assert!(result.is_ok());

        // get the expected result from the original buffer
        let expected = &b[offset..offset + header.size()];

        // verify that the written header is the same as original
        assert_ne!(buffer.as_slice(),expected);

        // read the modified data back from the buffer
        let result = header.read(&buffer);
        assert!(result.is_ok());

        // check that the re-parsed header has changed value
        assert_eq!(header.address(),Some(123));
    }
}