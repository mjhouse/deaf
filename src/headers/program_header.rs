use crate::common::{
    Width,
    Layout,
    PHType
};

use crate::common::field::Field;
use crate::common::ranges::*;
use crate::errors::{Result};
use crate::impl_property;

#[derive(Debug,Clone)]
pub struct ProgramHeaderValues {
    size: usize,  
    p_type: PHType,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}

#[derive(Debug)]
pub struct ProgramHeader {
    offset: usize,
    layout: Layout,
    width: Width,
    size: usize,
    p_type: Field<u32,u32,PHType>,
    p_flags: Field<u32>,
    p_offset: Field<u32,u64>,
    p_vaddr: Field<u32,u64>,
    p_paddr: Field<u32,u64>,
    p_filesz: Field<u32,u64>,
    p_memsz: Field<u32,u64>,
    p_align: Field<u32,u64>,
    values: ProgramHeaderValues,
}

impl ProgramHeaderValues {

    pub fn new() -> Self {
        Self {
            size: 0, 
            p_type: PHType::PT_NULL,
            p_flags: 0,
            p_offset: 0,
            p_vaddr: 0,
            p_paddr: 0,
            p_filesz: 0,
            p_memsz: 0,
            p_align: 0,
        }
    }
}

impl ProgramHeader {

    pub fn new(offset: usize, size: usize, layout: Layout, width: Width) -> Self {
        Self {
            offset,
            layout,
            width,
            size: size,
            p_type: Field::new(P_TYPE),
            p_flags: Field::new(P_FLAGS),
            p_offset: Field::new(P_OFFSET),
            p_vaddr: Field::new(P_VADDR),
            p_paddr: Field::new(P_PADDR),
            p_filesz: Field::new(P_FILESZ),
            p_memsz: Field::new(P_MEMSZ),
            p_align: Field::new(P_ALIGN),
            values: ProgramHeaderValues::new(),
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
        self.p_type.ranges.width = width;
        self.p_flags.ranges.width = width;
        self.p_offset.ranges.width = width;
        self.p_vaddr.ranges.width = width;
        self.p_paddr.ranges.width = width;
        self.p_filesz.ranges.width = width;
        self.p_memsz.ranges.width = width;
        self.p_align.ranges.width = width;
    }

    pub fn set_layout(&mut self, layout: Layout) {
        self.layout = layout;
        self.p_type.layout = layout;
        self.p_flags.layout = layout;
        self.p_offset.layout = layout;
        self.p_vaddr.layout = layout;
        self.p_paddr.layout = layout;
        self.p_filesz.layout = layout;
        self.p_memsz.layout = layout;
        self.p_align.layout = layout;
    }

    pub fn read(&mut self, b: &[u8]) -> Result<ProgramHeaderValues> {
        self.set_layout(self.layout);
        self.set_width(self.width);

        self.values.p_type   = self.p_type.get(b)?;
        self.values.p_flags  = self.p_flags.get(b)?;
        self.values.p_offset = self.p_offset.get(b)?;
        self.values.p_vaddr  = self.p_vaddr.get(b)?;
        self.values.p_paddr  = self.p_paddr.get(b)?;
        self.values.p_filesz = self.p_filesz.get(b)?;
        self.values.p_memsz  = self.p_memsz.get(b)?;
        self.values.p_align  = self.p_align.get(b)?;

        Ok(self.values.clone())
    }

    pub fn write(&self, b: &mut [u8]) -> Result<()> {
        self.p_type.set(b,self.values.p_type)?;
        self.p_flags.set(b,self.values.p_flags)?;
        self.p_offset.set(b,self.values.p_offset)?;
        self.p_vaddr.set(b,self.values.p_vaddr)?;
        self.p_paddr.set(b,self.values.p_paddr)?;
        self.p_filesz.set(b,self.values.p_filesz)?;
        self.p_memsz.set(b,self.values.p_memsz)?;
        self.p_align.set(b,self.values.p_align)?;
        Ok(())
    }

    pub fn header_size(&self) -> usize {
        self.p_type.size() +
        self.p_flags.size() +
        self.p_offset.size() +
        self.p_vaddr.size() +
        self.p_paddr.size() +
        self.p_filesz.size() +
        self.p_memsz.size() +
        self.p_align.size()
    }

    impl_property!(kind, p_type, PHType);
    impl_property!(flags, p_flags, u32);
    impl_property!(offset, p_offset, u64);
    impl_property!(vaddr, p_vaddr, u64);
    impl_property!(paddr, p_paddr, u64);
    impl_property!(filesz, p_filesz, u64);
    impl_property!(memsz, p_memsz, u64);
    impl_property!(align, p_align, u64);

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::headers::FileHeader;

    use crate::utilities::tests::read;

    #[test]
    fn test_read_program_headers() {
        let b = read("assets/libvpf/libvpf.so.4.1");

        // get the file header to find program headers
        let file_header = FileHeader::parse(&b)
            .unwrap();

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
        assert_eq!(header.header_size(),size);
        assert_eq!(header.filesz(),0x4348);
        assert_eq!(header.align(),0x1000);
    }

    #[test]
    fn test_write_program_header_with_no_changes() {
        let b = read("assets/libvpf/libvpf.so.4.1");

        // get the file header to find program headers
        let file_header = FileHeader::parse(&b)
            .unwrap();

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
    fn test_write_program_header_with_changes() {
        let b = read("assets/libvpf/libvpf.so.4.1");

        // get the file header to find program headers
        let file_header = FileHeader::parse(&b)
            .unwrap();

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
        assert_eq!(header.paddr(),123);
    }

}