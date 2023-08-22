use crate::errors::{Error,Result};
use crate::common::{Width,Layout};
use crate::common::{Item,ranges::*};

/// The ELF file header parsed from the beginning of the file
#[derive(Debug)]
pub struct FileHeader {
    layout: Layout,
    width: Width,
    ei_key: Item<u8>,
    ei_magic: Item<String>,
    ei_class: Item<u8,u8,Width>,
    ei_data: Item<u8,u8,Layout>,
    ei_version: Item<u8>,
    ei_osabi: Item<u8>,
    ei_abiversion: Item<u8>,
    ei_pad: Item<[u8;7]>,
    e_type: Item<u16>,
    e_machine: Item<u16>,
    e_version: Item<u32>,
    e_entry: Item<u32,u64>,
    e_phoff: Item<u32,u64,usize>,
    e_shoff: Item<u32,u64,usize>,
    e_flags: Item<u32>,
    e_ehsize: Item<u16>,
    e_phentsize: Item<u16,u16,usize>,
    e_phnum: Item<u16,u16,usize>,
    e_shentsize: Item<u16,u16,usize>,
    e_shnum: Item<u16,u16,usize>,
    e_shstrndx: Item<u16,u16,usize>,
}

impl FileHeader {

    /// Create a new header with given Layout and Width
    ///
    /// All fields are None until read
    pub fn new() -> Self {
        Self {
            layout: Layout::Little,
            width: Width::X64,
            ei_key: Item::new(EI_KEY),
            ei_magic: Item::new(EI_MAGIC),
            ei_class: Item::new(EI_CLASS),
            ei_data: Item::new(EI_DATA),
            ei_version: Item::new(EI_VERSION),
            ei_osabi: Item::new(EI_OSABI),
            ei_abiversion: Item::new(EI_ABIVERSION),
            ei_pad: Item::new(EI_PAD),
            e_type: Item::new(E_TYPE),
            e_machine: Item::new(E_MACHINE),
            e_version: Item::new(E_VERSION),
            e_entry: Item::new(E_ENTRY),
            e_phoff: Item::new(E_PHOFF),
            e_shoff: Item::new(E_SHOFF),
            e_flags: Item::new(E_FLAGS),
            e_ehsize: Item::new(E_EHSIZE),
            e_phentsize: Item::new(E_PHENTSIZE),
            e_phnum: Item::new(E_PHNUM),
            e_shentsize: Item::new(E_SHENTSIZE),
            e_shnum: Item::new(E_SHNUM),
            e_shstrndx: Item::new(E_SHSTRNDX),
        }
    }

    /// Parse a header from the provided byte buffer
    pub fn parse(b: &[u8]) -> Result<Self> {
        let mut h = Self::new();
        h.read(b)?;
        Ok(h)
    }

    /// Read values from a byte buffer 
    ///
    /// Byte buffer is assumed to be sliced such that the
    /// header is at the beginning of the buffer.
    pub fn read(&mut self, b: &[u8]) -> Result<()> {
        self.ei_key.read(b)?;
        self.ei_magic.read(b)?;
        self.ei_class.read(b)?;
        self.ei_data.read(b)?;
        self.ei_version.read(b)?;
        self.ei_osabi.read(b)?;
        self.ei_abiversion.read(b)?;
        self.ei_pad.read(b)?;

        let layout = self
            .data();

        let width = self
            .class();

        self.set_layout(layout);
        self.set_width(width);

        self.e_type.read(b)?;
        self.e_machine.read(b)?;
        self.e_version.read(b)?;
        self.e_entry.read(b)?;
        self.e_phoff.read(b)?;
        self.e_shoff.read(b)?;
        self.e_flags.read(b)?;
        self.e_ehsize.read(b)?;
        self.e_phentsize.read(b)?;
        self.e_phnum.read(b)?;
        self.e_shentsize.read(b)?;
        self.e_shnum.read(b)?;
        self.e_shstrndx.read(b)?;

        Ok(())
    }

    /// Write values to a byte buffer 
    ///
    /// Byte buffer is assumed to be sliced such that the
    /// header will be written at the correct position.
    pub fn write(&mut self, b: &mut [u8]) -> Result<()> {
        self.ei_key.write(b)?;
        self.ei_magic.write(b)?;
        self.ei_class.write(b)?;
        self.ei_data.write(b)?;
        self.ei_version.write(b)?;
        self.ei_osabi.write(b)?;
        self.ei_abiversion.write(b)?;
        self.ei_pad.write(b)?;
        self.e_type.write(b)?;
        self.e_machine.write(b)?;
        self.e_version.write(b)?;
        self.e_entry.write(b)?;
        self.e_phoff.write(b)?;
        self.e_shoff.write(b)?;
        self.e_flags.write(b)?;
        self.e_ehsize.write(b)?;
        self.e_phentsize.write(b)?;
        self.e_phnum.write(b)?;
        self.e_shentsize.write(b)?;
        self.e_shnum.write(b)?;
        self.e_shstrndx.write(b)?;
        Ok(())
    }

    /// The size of the header in bytes
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

    /// Get the width (32 or 64-bit) of the header
    pub fn width(&self) -> Width {
        self.width
    }

    /// Set the width of the header
    pub fn set_width(&mut self, width: Width) {
        self.width = width;
        self.e_entry.set_width(width);
        self.e_phoff.set_width(width);
        self.e_shoff.set_width(width);
        self.e_flags.set_width(width);
        self.e_ehsize.set_width(width);
        self.e_phentsize.set_width(width);
        self.e_phnum.set_width(width);
        self.e_shentsize.set_width(width);
        self.e_shnum.set_width(width);
        self.e_shstrndx.set_width(width);
    }

    /// Get the layout (little or big-endian) of the header
    pub fn layout(&self) -> Layout {
        self.layout
    }

    /// Set the layout of the header
    pub fn set_layout(&mut self, layout: Layout) {
        self.layout = layout;
        self.e_type.set_layout(layout);
        self.e_machine.set_layout(layout);
        self.e_version.set_layout(layout);
        self.e_entry.set_layout(layout);
        self.e_phoff.set_layout(layout);
        self.e_shoff.set_layout(layout);
        self.e_flags.set_layout(layout);
        self.e_ehsize.set_layout(layout);
        self.e_phentsize.set_layout(layout);
        self.e_phnum.set_layout(layout);
        self.e_shentsize.set_layout(layout);
        self.e_shnum.set_layout(layout);
        self.e_shstrndx.set_layout(layout);
    }

    /// Get the `ei_magic` attribute of the header
    pub fn magic(&self) -> String {
        self.ei_magic.get()
    }

    /// Set the `ei_magic` attribute of the header 
    pub fn set_magic(&mut self, magic: String) {
        self.ei_magic.set(magic);
    }

    /// Get the `ei_class` attribute of the header
    pub fn class(&self) -> Width {
        self.ei_class.get()
    }

    /// Set the `ei_class` attribute of the header 
    pub fn set_class(&mut self, class: Width) {
        self.ei_class.set(class);
    }

    /// Get the `ei_data` attribute of the header
    pub fn data(&self) -> Layout {
        self.ei_data.get()
    }

    /// Set the `ei_data` attribute of the header 
    pub fn set_data(&mut self, data: Layout) {
        self.ei_data.set(data);
    }

    /// Get the `ei_version` attribute of the header
    pub fn version(&self) -> u8 {
        self.ei_version.get()
    }

    /// Set the `ei_version` attribute of the header 
    pub fn set_version(&mut self, version: u8) {
        self.ei_version.set(version);
    }

    /// Get the `ei_osabi` attribute of the header
    pub fn osabi(&self) -> u8 {
        self.ei_osabi.get()
    }

    /// Set the `ei_osabi` attribute of the header 
    pub fn set_osabi(&mut self, osabi: u8) {
        self.ei_osabi.set(osabi);
    }

    /// Get the `abiversion` attribute of the header
    pub fn abiversion(&self) -> u8 {
        self.ei_abiversion.get()
    }

    /// Set the `abiversion` attribute of the header 
    pub fn set_abiversion(&mut self, abiversion: u8) {
        self.ei_abiversion.set(abiversion);
    }

    /// Get the `e_type` attribute of the header
    pub fn file_type(&self) -> u16 {
        self.e_type.get()
    }

    /// Set the `e_type` attribute of the header 
    pub fn set_file_type(&mut self, file_type: u16) {
        self.e_type.set(file_type);
    }

    /// Get the `e_machine` attribute of the header
    pub fn machine(&self) -> u16 {
        self.e_machine.get()
    }

    /// Set the `e_machine` attribute of the header 
    pub fn set_machine(&mut self, machine: u16) {
        self.e_machine.set(machine);
    }

    /// Get the `e_entry` attribute of the header
    pub fn entry(&self) -> u64 {
        self.e_entry.get()
    }

    /// Set the `e_entry` attribute of the header 
    pub fn set_entry(&mut self, entry: u64) {
        self.e_entry.set(entry);
    }

    /// Get the `e_phoff` attribute of the header
    pub fn phoff(&self) -> usize {
        self.e_phoff.get()
    }

    /// Set the `e_phoff` attribute of the header 
    pub fn set_phoff(&mut self, phoff: usize) {
        self.e_phoff.set(phoff);
    }

    /// Get the `e_shoff` attribute of the header
    pub fn shoff(&self) -> usize {
        self.e_shoff.get()
    }

    /// Set the `e_shoff` attribute of the header 
    pub fn set_shoff(&mut self, shoff: usize) {
        self.e_shoff.set(shoff);
    }

    /// Get the `e_flags` attribute of the header
    pub fn flags(&self) -> u32 {
        self.e_flags.get()
    }

    /// Set the `e_flags` attribute of the header 
    pub fn set_flags(&mut self, flags: u32) {
        self.e_flags.set(flags);
    }

    /// Get the `e_ehsize` attribute of the header
    pub fn ehsize(&self) -> u16 {
        self.e_ehsize.get()
    }

    /// Set the `e_ehsize` attribute of the header 
    pub fn set_ehsize(&mut self, ehsize: u16) {
        self.e_ehsize.set(ehsize);
    }

    /// Get the `e_phentsize` attribute of the header
    pub fn phentsize(&self) -> usize {
        self.e_phentsize.get()
    }

    /// Set the `e_phentsize` attribute of the header 
    pub fn set_phentsize(&mut self, phentsize: usize) {
        self.e_phentsize.set(phentsize);
    }

    /// Get the `e_phnum` attribute of the header
    pub fn phnum(&self) -> usize {
        self.e_phnum.get()
    }

    /// Set the `e_phnum` attribute of the header 
    pub fn set_phnum(&mut self, phnum: usize) {
        self.e_phnum.set(phnum);
    }

    /// Get the `e_shentsize` attribute of the header
    pub fn shentsize(&self) -> usize {
        self.e_shentsize.get()
    }

    /// Set the `e_shentsize` attribute of the header 
    pub fn set_shentsize(&mut self, shentsize: usize) {
        self.e_shentsize.set(shentsize);
    }

    /// Get the `e_shnum` attribute of the header
    pub fn shnum(&self) -> usize {
        self.e_shnum.get()
    }

    /// Set the `e_shnum` attribute of the header 
    pub fn set_shnum(&mut self, shnum: usize) {
        self.e_shnum.set(shnum);
    }

    /// Get the `e_shstrndx` attribute of the header
    pub fn shstrndx(&self) -> usize {
        self.e_shstrndx.get()
    }

    /// Set the `e_shstrndx` attribute of the header 
    pub fn set_shstrndx(&mut self, shstrndx: usize) {
        self.e_shstrndx.set(shstrndx);
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::read;

    #[test]
    fn test_read_file_header() {
        // read file as byte array
        let b = read("assets/libvpf/libvpf.so.4.1").unwrap();

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
        assert_eq!(header.size(),64);
    }

    #[test]
    fn test_write_file_header_with_no_changes() {
        // read file as byte array
        let b = read("assets/libvpf/libvpf.so.4.1").unwrap();

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
        let b = read("assets/libvpf/libvpf.so.4.1").unwrap();

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

        let header = result.unwrap();

        // verify that the re-parsed shentsize is the new value
        assert_eq!(header.shentsize(),123);
    }
}