use crate::headers::common::constants::Width;
use crate::headers::common::constants::Layout;
use crate::headers::common::field::Field;
use crate::headers::common::ranges::*;

use crate::errors::{Error, Result};

#[derive(Debug,Clone)]
pub struct ProgramHeaderValues {
    p_size: usize, 
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}

pub struct ProgramHeader {
    offset: usize,
    layout: Layout,
    width: Width,

    p_size: usize,
    p_type: Field<u32>,
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
            p_size: 0, 
            p_type: 0,
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
            p_size: 0, 
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

    pub fn set_width(&mut self, width: Width) {
        self.p_type.ranges.width = width.clone();
        self.p_flags.ranges.width = width.clone();
        self.p_offset.ranges.width = width.clone();
        self.p_vaddr.ranges.width = width.clone();
        self.p_paddr.ranges.width = width.clone();
        self.p_filesz.ranges.width = width.clone();
        self.p_memsz.ranges.width = width.clone();
        self.p_align.ranges.width = width.clone();
    }

    pub fn set_layout(&mut self, layout: Layout) {
        self.p_type.layout = layout.clone();
        self.p_flags.layout = layout.clone();
        self.p_offset.layout = layout.clone();
        self.p_vaddr.layout = layout.clone();
        self.p_paddr.layout = layout.clone();
        self.p_filesz.layout = layout.clone();
        self.p_memsz.layout = layout.clone();
        self.p_align.layout = layout.clone();
    }

    pub fn read(&mut self, b: &[u8]) -> Result<ProgramHeaderValues> {
        let s = &b[self.offset..];

        self.set_layout(self.layout.clone());
        self.set_width(self.width.clone());

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

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    // #[test]
    // fn test_extract_header_from_shared_library() {
    //     let mut f = File::open("assets/libvpf.so.4.1").unwrap();
    //     let mut b = Vec::new();
    //     f.read_to_end(&mut b).unwrap();

    //     let header = FileHeader::parse(&b);
    //     assert!(header.is_ok());
    // }
}