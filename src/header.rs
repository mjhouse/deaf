use crate::errors::{Error, Result};
use crate::constants::{Width,Layout,ELF_SIZE_32,ELF_SIZE_64};
use crate::field::Field;
use crate::ranges::*;

#[derive(Default,Debug,Clone)]
pub struct HeaderValues {
    size: usize,
    magic: String,
    ei_class: u8,
    ei_data: u8,
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
    size: usize,
    magic: Field<String>,
    ei_class: Field<u8>,
    ei_data: Field<u8>,
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
}

impl Header {

    pub fn new() -> Self {
        Self {
            size: 0,
            magic: Field::simple(EI_MAGIC),
            ei_class: Field::simple(EI_CLASS),
            ei_data: Field::simple(EI_DATA),
            ei_version: Field::simple(EI_VERSION),
            ei_osabi: Field::simple(EI_OSABI),
            ei_abiversion: Field::simple(EI_ABIVERSION),
            e_type: Field::simple(E_TYPE),
            e_machine: Field::simple(E_MACHINE),
            e_version: Field::simple(E_VERSION),
            e_entry: Field::simple(E_ENTRY),
            e_phoff: Field::simple(E_PHOFF),
            e_shoff: Field::simple(E_SHOFF),
            e_flags: Field::simple(E_FLAGS),
            e_ehsize: Field::simple(E_EHSIZE),
            e_phentsize: Field::simple(E_PHENTSIZE),
            e_phnum: Field::simple(E_PHNUM),
            e_shentsize: Field::simple(E_SHENTSIZE),
            e_shnum: Field::simple(E_SHNUM),
            e_shstrndx: Field::simple(E_SHSTRNDX),
        }
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
        // self.e_type.ranges.width = width.clone();
        // self.e_machine.ranges.width = width.clone();
        // self.e_version.ranges.width = width.clone();

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

    pub fn read(&mut self, b: &[u8]) -> Result<HeaderValues> {
        let mut values = HeaderValues::default();

        values.magic         = self.magic.get(b)?;
        values.ei_class      = self.ei_class.get(b)?;
        values.ei_data       = self.ei_data.get(b)?;
        values.ei_version    = self.ei_version.get(b)?;
        values.ei_osabi      = self.ei_osabi.get(b)?;
        values.ei_abiversion = self.ei_abiversion.get(b)?;

        // get the address width for the binary
        let width = match values.ei_class {
            2 => Width::X64,
            1 => Width::X32,
            _ => Err(Error::ParseError)?,
        };

        // get the header size for the binary
        self.size = match values.ei_class {
            2 => ELF_SIZE_64,
            1 => ELF_SIZE_32,
            _ => Err(Error::ParseError)?,
        };

        // get the endianness of the binary
        let layout = match values.ei_data {
            2 => Layout::Big,
            1 => Layout::Little,
            _ => Err(Error::ParseError)?,
        };

        self.set_layout(layout);
        self.set_width(width);

        values.size          = self.size;
        values.e_type        = self.e_type.get(b)?;
        values.e_machine     = self.e_machine.get(b)?;
        values.e_version     = self.e_version.get(b)?;
        values.e_entry       = self.e_entry.get(b)?;
        values.e_phoff       = self.e_phoff.get(b)?;
        values.e_shoff       = self.e_shoff.get(b)?;
        values.e_flags       = self.e_flags.get(b)?;
        values.e_ehsize      = self.e_ehsize.get(b)?;
        values.e_phentsize   = self.e_phentsize.get(b)?;
        values.e_phnum       = self.e_phnum.get(b)?;
        values.e_shentsize   = self.e_shentsize.get(b)?;
        values.e_shnum       = self.e_shnum.get(b)?;
        values.e_shstrndx    = self.e_shstrndx.get(b)?;

        Ok(values)
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

        let mut header = Header::new();
        let values = header.read(&b);
        dbg!(values);
    }
}