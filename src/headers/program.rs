use crate::common::{
    Width,
    Layout,
    PHType,
    Updateable
};

use crate::common::{Item, ranges::*};
use crate::errors::Result;

/// Program headers extracted from an ELF file.
/// 
/// Normally found at the offset declared in the FileHeader 
/// as 'phoff'.
#[derive(Debug,Clone)]
pub struct ProgramHeader {
    layout: Layout,
    width: Width,
    p_type: Item<u32,u32,PHType>,
    p_flags: Item<u32>,
    p_offset: Item<u32,u64,usize>,
    p_vaddr: Item<u32,u64>,
    p_paddr: Item<u32,u64>,
    p_filesz: Item<u32,u64,usize>,
    p_memsz: Item<u32,u64>,
    p_align: Item<u32,u64>,
}

impl ProgramHeader {

    /// Create a new header with given Layout and Width
    ///
    /// All fields are None until read
    pub fn new(layout: Layout, width: Width) -> Self {
        Self {
            layout,
            width,
            p_type: Item::make(P_TYPE,width,layout),
            p_flags: Item::make(P_FLAGS,width,layout),
            p_offset: Item::make(P_OFFSET,width,layout),
            p_vaddr: Item::make(P_VADDR,width,layout),
            p_paddr: Item::make(P_PADDR,width,layout),
            p_filesz: Item::make(P_FILESZ,width,layout),
            p_memsz: Item::make(P_MEMSZ,width,layout),
            p_align: Item::make(P_ALIGN,width,layout),
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
        self.p_type.read(b)?;
        self.p_flags.read(b)?;
        self.p_offset.read(b)?;
        self.p_vaddr.read(b)?;
        self.p_paddr.read(b)?;
        self.p_filesz.read(b)?;
        self.p_memsz.read(b)?;
        self.p_align.read(b)?;
        Ok(())
    }

    /// Write values to a byte buffer 
    ///
    /// Byte buffer is assumed to be sliced such that the
    /// header will be written at the correct position.
    pub fn write(&self, b: &mut [u8]) -> Result<()> {
        self.p_type.write(b)?;
        self.p_flags.write(b)?;
        self.p_offset.write(b)?;
        self.p_vaddr.write(b)?;
        self.p_paddr.write(b)?;
        self.p_filesz.write(b)?;
        self.p_memsz.write(b)?;
        self.p_align.write(b)?;
        Ok(())
    }

    /// The size of the header in bytes
    pub fn size(&self) -> usize {
        self.p_type.size() +
        self.p_flags.size() +
        self.p_offset.size() +
        self.p_vaddr.size() +
        self.p_paddr.size() +
        self.p_filesz.size() +
        self.p_memsz.size() +
        self.p_align.size()
    }

    /// Get the width (32 or 64-bit) of the header
    pub fn width(&self) -> Width {
        self.width
    }

    /// Set the width of the header
    pub fn set_width(&mut self, width: Width) {
        self.width = width;
        self.p_type.set_width(width);
        self.p_flags.set_width(width);
        self.p_offset.set_width(width);
        self.p_vaddr.set_width(width);
        self.p_paddr.set_width(width);
        self.p_filesz.set_width(width);
        self.p_memsz.set_width(width);
        self.p_align.set_width(width);
    }

    /// Get the layout (little or big-endian) of the header
    pub fn layout(&self) -> Layout {
        self.layout
    }

    /// Set the layout of the header
    pub fn set_layout(&mut self, layout: Layout) {
        self.layout = layout;
        self.p_type.set_layout(layout);
        self.p_flags.set_layout(layout);
        self.p_offset.set_layout(layout);
        self.p_vaddr.set_layout(layout);
        self.p_paddr.set_layout(layout);
        self.p_filesz.set_layout(layout);
        self.p_memsz.set_layout(layout);
        self.p_align.set_layout(layout);
    }

    /// Get the `p_type` attribute of the header
    pub fn kind(&self) -> PHType {
        self.p_type.get()
    }

    /// Set the `p_type` attribute of the header 
    pub fn set_kind(&mut self, kind: PHType) {
        self.p_type.set(kind);
    }

    /// Get the `p_flags` attribute of the header
    pub fn flags(&self) -> u32 {
        self.p_flags.get()
    }

    /// Set the `p_flags` attribute of the header 
    pub fn set_flags(&mut self, flags: u32) {
        self.p_flags.set(flags);
    }

    /// Get the `p_offset` attribute of the header
    pub fn offset(&self) -> usize {
        self.p_offset.get()
    }

    /// Set the `p_offset` attribute of the header 
    pub fn set_offset(&mut self, offset: usize) {
        self.p_offset.set(offset);
    }

    /// Get the `p_vaddr` attribute of the header
    pub fn vaddr(&self) -> u64 {
        self.p_vaddr.get()
    }

    /// Set the `p_vaddr` attribute of the header 
    pub fn set_vaddr(&mut self, vaddr: u64) {
        self.p_vaddr.set(vaddr);
    }

    /// Get the `p_paddr` attribute of the header
    pub fn paddr(&self) -> u64 {
        self.p_paddr.get()
    }

    /// Set the `p_paddr` attribute of the header 
    pub fn set_paddr(&mut self, paddr: u64) {
        self.p_paddr.set(paddr);
    }

    /// Get the `p_filesz` attribute of the header
    pub fn filesz(&self) -> usize {
        self.p_filesz.get()
    }

    /// Set the `p_filesz` attribute of the header 
    pub fn set_filesz(&mut self, filesz: usize) {
        self.p_filesz.set(filesz);
    }

    /// Get the `p_memsz` attribute of the header
    pub fn memsz(&self) -> u64 {
        self.p_memsz.get()
    }

    /// Set the `p_memsz` attribute of the header 
    pub fn set_memsz(&mut self, memsz: u64) {
        self.p_memsz.set(memsz);
    }

    /// Get the `p_align` attribute of the header
    pub fn align(&self) -> u64 {
        self.p_align.get()
    }

    /// Set the `p_align` attribute of the header 
    pub fn set_align(&mut self, align: u64) {
        self.p_align.set(align);
    }

    /// Get the `p_filesz` attribute of the header
    pub fn body_size(&self) -> usize {
        self.p_filesz.get()
    }

    /// Set the `p_filesz` attribute of the header 
    pub fn set_body_size(&mut self, body_size: usize) {
        self.p_filesz.set(body_size);
    }

}

impl Updateable for ProgramHeader {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::headers::FileHeader;

    use crate::utilities::read;

    #[test]
    fn test_read_program_headers() {
        let b = read("assets/libvpf/libvpf.so.4.1").unwrap();

        // get the file header to find program headers
        let file_header = FileHeader::parse(&b).unwrap();

        let count = file_header.phnum();
        let offset = file_header.phoff();
        let size = file_header.phentsize();
        let layout = file_header.data();
        let width = file_header.class();
        
        // parse all program headers in file
        let program_headers = ProgramHeader::parse_all(
            &b,
            count,
            offset,
            size,
            layout,
            width);

        // get the first program header for testing
        assert!(program_headers.is_ok());
        let headers = program_headers.unwrap();

        let header = &headers[0];

        // check values are what we expected
        assert_eq!(header.size(),size);
        assert_eq!(header.filesz(),0x4348);
        assert_eq!(header.align(),0x1000);
    }

    #[test]
    fn test_write_program_header_with_no_changes() {
        let b = read("assets/libvpf/libvpf.so.4.1").unwrap();

        // get the file header to find program headers
        let file_header = FileHeader::parse(&b).unwrap();

        let count = file_header.phnum();
        let offset = file_header.phoff();
        let size = file_header.phentsize();
        let layout = file_header.data();
        let width = file_header.class();
        
        // parse all program headers in file
        let program_headers = ProgramHeader::parse_all(
            &b,
            count,
            offset,
            size,
            layout,
            width);

        // get the first program header for testing
        assert!(program_headers.is_ok());
        let mut headers = program_headers.unwrap();

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
    fn test_write_program_header_with_changes() {
        let b = read("assets/libvpf/libvpf.so.4.1").unwrap();

        // get the file header to find program headers
        let file_header = FileHeader::parse(&b).unwrap();

        let count = file_header.phnum();
        let offset = file_header.phoff();
        let size = file_header.phentsize();
        let layout = file_header.data();
        let width = file_header.class();
        
        // parse all program headers in file
        let program_headers = ProgramHeader::parse_all(
            &b,
            count,
            offset,
            size,
            layout,
            width);

        // get the first program header for testing
        assert!(program_headers.is_ok());
        let mut headers = program_headers.unwrap();

        let header = &mut headers[0];

        // change a field in the program header
        header.set_paddr(123);

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
        assert_eq!(header.paddr(),123);
    }

}