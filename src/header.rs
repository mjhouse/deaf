use crate::errors::{Error, Result};
use crate::constants::{Width,Layout,ELF_SIZE_32,ELF_SIZE_64};
use crate::field::Field;
use crate::ranges::*;

#[derive(Debug,Clone)]
pub struct HeaderValues {
    ei_size: usize,
    ei_magic: String,
    ei_class: Width,
    ei_data: Layout,
    ei_version: u8,
    ei_osabi: u8,
    ei_abiversion: u8,
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

pub struct Header {
    ei_size: usize,
    ei_magic: Field<String>,
    ei_class: Field<u8,u8,Width>,
    ei_data: Field<u8,u8,Layout>,
    ei_version: Field<u8>,
    ei_osabi: Field<u8>,
    ei_abiversion: Field<u8>,
    e_type: Field<u16>,
    e_machine: Field<u16>,
    e_version: Field<u32>,
    e_entry: Field<u32,u64>,
    e_phoff: Field<u32,u64>,
    e_shoff: Field<u32,u64>,
    e_flags: Field<u32>,
    e_ehsize: Field<u16>,
    e_phentsize: Field<u16>,
    e_phnum: Field<u16>,
    e_shentsize: Field<u16>,
    e_shnum: Field<u16>,
    e_shstrndx: Field<u16>,
    values: HeaderValues,
}

impl HeaderValues {

    pub fn new() -> Self {
        Self {
            ei_size: 0,
            ei_magic: "".into(),
            ei_class: Width::X32,
            ei_data: Layout::Little,
            ei_version: 0,
            ei_osabi: 0,
            ei_abiversion: 0,
            e_type: 0,
            e_machine: 0,
            e_version: 0,
            e_entry: 0,
            e_phoff: 0,
            e_shoff: 0,
            e_flags: 0,
            e_ehsize: 0,
            e_phentsize: 0,
            e_phnum: 0,
            e_shentsize: 0,
            e_shnum: 0,
            e_shstrndx: 0,
        }
    }

}

impl Header {

    pub fn new() -> Self {
        Self {
            ei_size: 0,
            ei_magic: Field::new(EI_MAGIC),
            ei_class: Field::new(EI_CLASS),
            ei_data: Field::new(EI_DATA),
            ei_version: Field::new(EI_VERSION),
            ei_osabi: Field::new(EI_OSABI),
            ei_abiversion: Field::new(EI_ABIVERSION),
            e_type: Field::new(E_TYPE),
            e_machine: Field::new(E_MACHINE),
            e_version: Field::new(E_VERSION),
            e_entry: Field::new(E_ENTRY),
            e_phoff: Field::new(E_PHOFF),
            e_shoff: Field::new(E_SHOFF),
            e_flags: Field::new(E_FLAGS),
            e_ehsize: Field::new(E_EHSIZE),
            e_phentsize: Field::new(E_PHENTSIZE),
            e_phnum: Field::new(E_PHNUM),
            e_shentsize: Field::new(E_SHENTSIZE),
            e_shnum: Field::new(E_SHNUM),
            e_shstrndx: Field::new(E_SHSTRNDX),
            values: HeaderValues::new(),
        }
    }

    pub fn parse(b: &[u8]) -> Result<Self> {
        let mut h = Self::new();
        h.read(b)?;
        Ok(h)
    }

    fn set_layout(&mut self, layout: Layout) {
        self.e_type.layout = layout.clone();
        self.e_machine.layout = layout.clone();
        self.e_version.layout = layout.clone();
        self.e_entry.layout = layout.clone();
        self.e_phoff.layout = layout.clone();
        self.e_shoff.layout = layout.clone();
        self.e_flags.layout = layout.clone();
        self.e_ehsize.layout = layout.clone();
        self.e_phentsize.layout = layout.clone();
        self.e_phnum.layout = layout.clone();
        self.e_shentsize.layout = layout.clone();
        self.e_shnum.layout = layout.clone();
        self.e_shstrndx.layout = layout.clone();
    }

    fn set_width(&mut self, width: Width) {
        self.e_entry.ranges.width = width.clone();
        self.e_phoff.ranges.width = width.clone();
        self.e_shoff.ranges.width = width.clone();
        self.e_flags.ranges.width = width.clone();
        self.e_ehsize.ranges.width = width.clone();
        self.e_phentsize.ranges.width = width.clone();
        self.e_phnum.ranges.width = width.clone();
        self.e_shentsize.ranges.width = width.clone();
        self.e_shnum.ranges.width = width.clone();
        self.e_shstrndx.ranges.width = width.clone();
    }

    fn get_size(&self) -> usize {
        match self.values.ei_class {
            Width::X64 => ELF_SIZE_64,
            Width::X32 => ELF_SIZE_32,
        } 
    }

    pub fn read(&mut self, b: &[u8]) -> Result<HeaderValues> {
        self.values.ei_magic      = self.ei_magic.get(b)?;
        self.values.ei_class      = self.ei_class.get(b)?;
        self.values.ei_data       = self.ei_data.get(b)?;
        self.values.ei_version    = self.ei_version.get(b)?;
        self.values.ei_osabi      = self.ei_osabi.get(b)?;
        self.values.ei_abiversion = self.ei_abiversion.get(b)?;

        self.set_layout(self.values.ei_data.clone());
        self.set_width(self.values.ei_class.clone());

        self.values.ei_size       = self.get_size();
        self.values.e_type        = self.e_type.get(b)?;
        self.values.e_machine     = self.e_machine.get(b)?;
        self.values.e_version     = self.e_version.get(b)?;
        self.values.e_entry       = self.e_entry.get(b)?;
        self.values.e_phoff       = self.e_phoff.get(b)?;
        self.values.e_shoff       = self.e_shoff.get(b)?;
        self.values.e_flags       = self.e_flags.get(b)?;
        self.values.e_ehsize      = self.e_ehsize.get(b)?;
        self.values.e_phentsize   = self.e_phentsize.get(b)?;
        self.values.e_phnum       = self.e_phnum.get(b)?;
        self.values.e_shentsize   = self.e_shentsize.get(b)?;
        self.values.e_shnum       = self.e_shnum.get(b)?;
        self.values.e_shstrndx    = self.e_shstrndx.get(b)?;

        Ok(self.values.clone())
    }
    
    pub fn size(&self) -> usize {
        self.values.ei_size.clone()
    }
    
    pub fn magic(&self) -> String {
        self.values.ei_magic.clone()
    }
    
    pub fn class(&self) -> Width {
        self.values.ei_class.clone()
    }
    
    pub fn data(&self) -> Layout {
        self.values.ei_data.clone()
    }
    
    pub fn version(&self) -> u8 {
        self.values.ei_version.clone()
    }
    
    pub fn osabi(&self) -> u8 {
        self.values.ei_osabi.clone()
    }
    
    pub fn abiversion(&self) -> u8 {
        self.values.ei_abiversion.clone()
    }
    
    pub fn file_type(&self) -> u16 {
        self.values.e_type.clone()
    }
    
    pub fn machine(&self) -> u16 {
        self.values.e_machine.clone()
    }
    
    pub fn entry(&self) -> u64 {
        self.values.e_entry.clone()
    }
    
    pub fn phoff(&self) -> u64 {
        self.values.e_phoff.clone()
    }
    
    pub fn shoff(&self) -> u64 {
        self.values.e_shoff.clone()
    }
    
    pub fn flags(&self) -> u32 {
        self.values.e_flags.clone()
    }
    
    pub fn ehsize(&self) -> u16 {
        self.values.e_ehsize.clone()
    }
    
    pub fn phentsize(&self) -> u16 {
        self.values.e_phentsize.clone()
    }
    
    pub fn phnum(&self) -> u16 {
        self.values.e_phnum.clone()
    }
    
    pub fn shentsize(&self) -> u16 {
        self.values.e_shentsize.clone()
    }
    
    pub fn shnum(&self) -> u16 {
        self.values.e_shnum.clone()
    }
    
    pub fn shstrndx(&self) -> u16 {
        self.values.e_shstrndx.clone()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_extract_header_from_shared_library() {
        let mut f = File::open("assets/libvpf.so.4.1").unwrap();
        let mut b = Vec::new();
        f.read_to_end(&mut b).unwrap();

        let header = Header::parse(&b);
        assert!(header.is_ok());
    }
}