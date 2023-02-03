use crate::headers::common::constants::{
    Width,
    Layout,
    SHType,
    sizes
};

use crate::headers::common::field::Field;
use crate::headers::common::ranges::*;
use crate::errors::{Error, Result};

#[derive(Debug,Clone)]
pub struct SectionHeaderValues {
    size: usize, 
    sh_name: u32,
    sh_type: SHType,
    sh_flags: u64,
    sh_address: u64,
    sh_offset: u64,
    sh_size: u64,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: u64,
    sh_entsize: u64,
}

#[derive(Debug)]
pub struct SectionHeader {
    offset: usize,
    layout: Layout,
    width: Width,
    size: usize, 

    sh_name: Field<u32>,
    sh_type: Field<u32,u32,SHType>,
    sh_flags: Field<u32,u64>,
    sh_address: Field<u32,u64>,
    sh_offset: Field<u32,u64>,
    sh_size: Field<u32,u64>,
    sh_link: Field<u32>,
    sh_info: Field<u32>,
    sh_addralign: Field<u32,u64>,
    sh_entsize: Field<u32,u64>,

    values: SectionHeaderValues,
}

impl SectionHeaderValues {

    pub fn new() -> Self {
        Self {
            size: 0, 
            sh_name: 0,
            sh_type: SHType::SHT_NULL,
            sh_flags: 0,
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

    pub fn new(offset: usize, layout: Layout, width: Width) -> Self {
        let size = sizes::section_header(width);

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

    pub fn parse(b: &[u8], offset: usize, layout: Layout, width: Width) -> Result<Self> {
        let mut header = Self::new(offset,layout,width);
        header.read(b)?;
        Ok(header)
    }

    pub fn parse_all(b: &[u8], count: usize, offset: usize, layout: Layout, width: Width) -> Result<Vec<Self>> {
        let mut result = vec![];
        result.reserve_exact(count);

        let size = sizes::section_header(width);

        for i in 0..count {
            result.push(Self::parse(
                b,
                offset + i * size,
                layout,
                width)?);
        }

        Ok(result)
    }

    pub fn set_width(&mut self, width: Width) {
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
        let s = &b[self.offset..];

        self.set_layout(self.layout);
        self.set_width(self.width);

        self.values.sh_name      = self.sh_name.get(s)?;
        self.values.sh_type      = self.sh_type.get(s)?;
        self.values.sh_flags     = self.sh_flags.get(s)?;
        self.values.sh_address   = self.sh_address.get(s)?;
        self.values.sh_offset    = self.sh_offset.get(s)?;
        self.values.sh_size      = self.sh_size.get(s)?;
        self.values.sh_link      = self.sh_link.get(s)?;
        self.values.sh_info      = self.sh_info.get(s)?;
        self.values.sh_addralign = self.sh_addralign.get(s)?;
        self.values.sh_entsize   = self.sh_entsize.get(s)?;

        Ok(self.values.clone())
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
        let mut f = File::open("assets/libvpf.so.4.1").unwrap();
        let mut b = Vec::new();
        
        f.read_to_end(&mut b)
            .unwrap();

        let file_header = FileHeader::parse(&b)
            .unwrap();

        let count = file_header.shnum();
        let offset = file_header.shoff();
        let layout = file_header.data();
        let width = file_header.class();
        
        let section_headers = SectionHeader::parse_all(
            &b,
            count,
            offset,
            layout,
            width);

        assert!(section_headers.is_ok())
    }
}