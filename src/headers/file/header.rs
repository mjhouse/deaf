use crate::errors::{Error, Result};
use crate::headers::common::constants::{Width,Layout,FH_SIZE_32,FH_SIZE_64};
use crate::headers::common::field::Field;
use crate::headers::common::ranges::*;
use crate::impl_property;

#[derive(Debug,Clone)]
pub struct FileHeaderValues {
    ei_key: u8,
    ei_magic: String,
    ei_class: Width,
    ei_data: Layout,
    ei_version: u8,
    ei_osabi: u8,
    ei_abiversion: u8,
    ei_pad: [u8;7],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: usize,
    e_shoff: usize,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: usize,
    e_shentsize: u16,
    e_shnum: usize,
    e_shstrndx: usize,
}

#[derive(Debug)]
pub struct FileHeader {
    ei_key: Field<u8>,
    ei_magic: Field<String>,
    ei_class: Field<u8,u8,Width>,
    ei_data: Field<u8,u8,Layout>,
    ei_version: Field<u8>,
    ei_osabi: Field<u8>,
    ei_abiversion: Field<u8>,
    ei_pad: Field<[u8;7]>,
    e_type: Field<u16>,
    e_machine: Field<u16>,
    e_version: Field<u32>,
    e_entry: Field<u32,u64>,
    e_phoff: Field<u32,u64,usize>,
    e_shoff: Field<u32,u64,usize>,
    e_flags: Field<u32>,
    e_ehsize: Field<u16>,
    e_phentsize: Field<u16>,
    e_phnum: Field<u16,u16,usize>,
    e_shentsize: Field<u16>,
    e_shnum: Field<u16,u16,usize>,
    e_shstrndx: Field<u16,u16,usize>,
    values: FileHeaderValues,
}

impl FileHeaderValues {

    pub fn new() -> Self {
        Self {
            ei_key: 0,
            ei_magic: "".into(),
            ei_class: Width::X32,
            ei_data: Layout::Little,
            ei_version: 0,
            ei_osabi: 0,
            ei_abiversion: 0,
            ei_pad: [0;7],
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

impl FileHeader {

    pub fn new() -> Self {
        Self {
            ei_key: Field::new(EI_KEY),
            ei_magic: Field::new(EI_MAGIC),
            ei_class: Field::new(EI_CLASS),
            ei_data: Field::new(EI_DATA),
            ei_version: Field::new(EI_VERSION),
            ei_osabi: Field::new(EI_OSABI),
            ei_abiversion: Field::new(EI_ABIVERSION),
            ei_pad: Field::new(EI_PAD),
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
            values: FileHeaderValues::new(),
        }
    }

    pub fn parse(b: &[u8]) -> Result<Self> {
        let mut h = Self::new();
        h.read(b)?;
        Ok(h)
    }

    pub fn set_layout(&mut self, layout: Layout) {
        // set internel layout value
        self.values.ei_data = layout;

        // set layout for all other fields
        self.e_type.layout = layout;
        self.e_machine.layout = layout;
        self.e_version.layout = layout;
        self.e_entry.layout = layout;
        self.e_phoff.layout = layout;
        self.e_shoff.layout = layout;
        self.e_flags.layout = layout;
        self.e_ehsize.layout = layout;
        self.e_phentsize.layout = layout;
        self.e_phnum.layout = layout;
        self.e_shentsize.layout = layout;
        self.e_shnum.layout = layout;
        self.e_shstrndx.layout = layout;
    }

    pub fn set_width(&mut self, width: Width) {
        // set internal width value
        self.values.ei_class = width;

        // set width for all other fields
        self.e_entry.ranges.width = width;
        self.e_phoff.ranges.width = width;
        self.e_shoff.ranges.width = width;
        self.e_flags.ranges.width = width;
        self.e_ehsize.ranges.width = width;
        self.e_phentsize.ranges.width = width;
        self.e_phnum.ranges.width = width;
        self.e_shentsize.ranges.width = width;
        self.e_shnum.ranges.width = width;
        self.e_shstrndx.ranges.width = width;
    }

    pub fn read(&mut self, b: &[u8]) -> Result<FileHeaderValues> {
        self.values.ei_key        = self.ei_key.get(b)?;
        self.values.ei_magic      = self.ei_magic.get(b)?;
        self.values.ei_class      = self.ei_class.get(b)?;
        self.values.ei_data       = self.ei_data.get(b)?;
        self.values.ei_version    = self.ei_version.get(b)?;
        self.values.ei_osabi      = self.ei_osabi.get(b)?;
        self.values.ei_abiversion = self.ei_abiversion.get(b)?;
        self.values.ei_pad        = self.ei_pad.get(b)?;

        self.set_layout(self.values.ei_data);
        self.set_width(self.values.ei_class);

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

    pub fn write(&mut self, b: &mut [u8]) -> Result<()> {
        self.ei_key.set(b,self.values.ei_key)?;
        self.ei_magic.set(b,self.values.ei_magic.clone())?;
        self.ei_class.set(b,self.values.ei_class)?;
        self.ei_data.set(b,self.values.ei_data)?;
        self.ei_version.set(b,self.values.ei_version)?;
        self.ei_osabi.set(b,self.values.ei_osabi)?;
        self.ei_abiversion.set(b,self.values.ei_abiversion)?;
        self.ei_pad.set(b,self.values.ei_pad)?;
        self.e_type.set(b,self.values.e_type)?;
        self.e_machine.set(b,self.values.e_machine)?;
        self.e_version.set(b,self.values.e_version)?;
        self.e_entry.set(b,self.values.e_entry)?;
        self.e_phoff.set(b,self.values.e_phoff)?;
        self.e_shoff.set(b,self.values.e_shoff)?;
        self.e_flags.set(b,self.values.e_flags)?;
        self.e_ehsize.set(b,self.values.e_ehsize)?;
        self.e_phentsize.set(b,self.values.e_phentsize)?;
        self.e_phnum.set(b,self.values.e_phnum)?;
        self.e_shentsize.set(b,self.values.e_shentsize)?;
        self.e_shnum.set(b,self.values.e_shnum)?;
        self.e_shstrndx.set(b,self.values.e_shstrndx)?;
        Ok(())
    }

    pub fn size(&self) -> usize {
        self.ei_key.size() +
        self.ei_magic.size() +
        self.ei_class.size() +
        self.ei_data.size() +
        self.ei_version.size() +
        self.ei_osabi.size() +
        self.ei_abiversion.size() +
        self.ei_pad.size() +
        self.e_type.size() +
        self.e_machine.size() +
        self.e_version.size() +
        self.e_entry.size() +
        self.e_phoff.size() +
        self.e_shoff.size() +
        self.e_flags.size() +
        self.e_ehsize.size() +
        self.e_phentsize.size() +
        self.e_phnum.size() +
        self.e_shentsize.size() +
        self.e_shnum.size() +
        self.e_shstrndx.size()
    }

    impl_property!(magic, ei_magic,String);
    impl_property!(class, ei_class,Width);
    impl_property!(data,ei_data,Layout);
    impl_property!(version,ei_version,u8);
    impl_property!(osabi,ei_osabi,u8);
    impl_property!(abiversion,ei_abiversion,u8);
    impl_property!(file_type,e_type,u16);
    impl_property!(machine,e_machine,u16);
    impl_property!(entry,e_entry,u64);
    impl_property!(phoff,e_phoff,usize);
    impl_property!(shoff,e_shoff,usize);
    impl_property!(flags,e_flags,u32);
    impl_property!(ehsize,e_ehsize,u16);
    impl_property!(phentsize,e_phentsize,u16);
    impl_property!(phnum,e_phnum,usize);
    impl_property!(shentsize,e_shentsize,u16);
    impl_property!(shnum,e_shnum,usize);
    impl_property!(shstrndx,e_shstrndx,usize);

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_read_file_header() {
        // read file as byte array
        let mut f = File::open("assets/libvpf.so.4.1").unwrap();
        let mut b = Vec::new();
        f.read_to_end(&mut b).unwrap();

        // parse the file header from the bytes
        let result = FileHeader::parse(&b);
        assert!(result.is_ok());

        let header = result.unwrap();

        // check values equal values from readelf
        //      readelf -h assets/libvpf.so.4.1
        assert_eq!(header.magic(),"ELF".to_string());
        assert_eq!(header.entry(),0x5740);
        assert_eq!(header.shoff(),287440);

        // check calculated size matches known x64 ELF size
        assert_eq!(header.size(),FH_SIZE_64);
    }

    #[test]
    fn test_write_file_header_with_no_changes() {
        // read file as byte array
        let mut f = File::open("assets/libvpf.so.4.1").unwrap();
        let mut b = Vec::new();
        f.read_to_end(&mut b).unwrap();

        // parse the file header from the bytes
        let result = FileHeader::parse(&b);
        assert!(result.is_ok());

        let mut header = result.unwrap();

        // initialize a buffer big enough for the header
        let mut buffer: Vec<u8> = vec![];
        buffer.resize(header.size(),0x00);        

        // write to the new buffer
        let result = header.write(buffer.as_mut_slice());
        assert!(result.is_ok());

        // verify that the written header is the same as original
        assert_eq!(buffer.as_slice(),&b[..header.size()]);
    }

    #[test]
    fn test_write_file_header_with_changes() {
        // read file as byte array
        let mut f = File::open("assets/libvpf.so.4.1").unwrap();
        let mut b = Vec::new();
        f.read_to_end(&mut b).unwrap();

        // parse the file header from the bytes
        let result = FileHeader::parse(&b);
        assert!(result.is_ok());

        let mut header = result.unwrap();

        // initialize a buffer big enough for the header
        let mut buffer: Vec<u8> = vec![];
        buffer.resize(header.size(),0x00);        

        // change the section header entity size
        header.set_shentsize(123);

        // write to the new buffer
        let result = header.write(buffer.as_mut_slice());
        assert!(result.is_ok());

        // verify that the written header is different
        assert_ne!(buffer.as_slice(),&b[..header.size()]);

        // re-parse the file header from the buffer
        let result = FileHeader::parse(&buffer);
        assert!(result.is_ok());

        let mut header = result.unwrap();

        // verify that the re-parsed shentsize is the new value
        assert_eq!(header.shentsize(),123);
    }
}