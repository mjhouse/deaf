use crate::headers::common::constants::{
    Width,
    Layout,
    PHType,
    PH_SIZE_32,
    PH_SIZE_64
};

use crate::common::field::Field;
use crate::common::ranges::*;
use crate::errors::{Error, Result};

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

    pub fn new(offset: usize, layout: Layout, width: Width) -> Self {
        Self {
            offset,
            layout,
            width,
            size: 0,
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

    pub fn parse(b: &[u8], offset: usize, layout: Layout, width: Width) -> Result<Self> {
        let mut header = Self::new(offset,layout,width);
        header.read(b)?;
        Ok(header)
    }

    pub fn parse_all(b: &[u8], count: usize, offset: usize, layout: Layout, width: Width) -> Result<Vec<Self>> {
        let mut result = vec![];
        result.reserve_exact(count);

        let size = match width {
            Width::X64 => PH_SIZE_64,
            Width::X32 => PH_SIZE_32,
        };

        // TODO: set the size on each ProgramHeader as they are parsed

        for i in 0..count {
            let index = offset + i * size;
            result.push(Self::parse(
                b,
                index,
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
        let s = &b[self.offset..];

        self.set_layout(self.layout);
        self.set_width(self.width);

        self.values.p_type   = self.p_type.get(s)?;
        self.values.p_flags  = self.p_flags.get(s)?;
        self.values.p_offset = self.p_offset.get(s)?;
        self.values.p_vaddr  = self.p_vaddr.get(s)?;
        self.values.p_paddr  = self.p_paddr.get(s)?;
        self.values.p_filesz = self.p_filesz.get(s)?;
        self.values.p_memsz  = self.p_memsz.get(s)?;
        self.values.p_align  = self.p_align.get(s)?;

        Ok(self.values.clone())
    }

    pub fn write(&self, b: &mut [u8]) -> Result<()> {
        let s = &mut b[self.offset..];
        self.p_type.set(s,self.values.p_type)?;
        self.p_flags.set(s,self.values.p_flags)?;
        self.p_offset.set(s,self.values.p_offset)?;
        self.p_vaddr.set(s,self.values.p_vaddr)?;
        self.p_paddr.set(s,self.values.p_paddr)?;
        self.p_filesz.set(s,self.values.p_filesz)?;
        self.p_memsz.set(s,self.values.p_memsz)?;
        self.p_align.set(s,self.values.p_align)?;
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

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;
    use crate::headers::file::header::FileHeader;

    #[test]
    fn test_extract_program_headers() {
        let mut f = File::open("assets/libvpf/libvpf.so.4.1").unwrap();
        let mut b = Vec::new();
        
        f.read_to_end(&mut b)
            .unwrap();

        let file_header = FileHeader::parse(&b)
            .unwrap();

        let count = file_header.phnum();
        let offset = file_header.phoff();
        let layout = file_header.data();
        let width = file_header.class();
        
        let program_headers = ProgramHeader::parse_all(
            &b,
            count,
            offset,
            layout,
            width);

        assert!(program_headers.is_ok())
    }
}