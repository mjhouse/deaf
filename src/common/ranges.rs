use std::ops::Range;
use crate::common::Width;

/// Leading byte of the ELF "magic number", should always be `0x7F`
pub const EI_KEY: Ranges = Ranges::new(0x00..0x01,0x00..0x01); // u8

/// The ASCII representation of the letters ELF
pub const EI_MAGIC: Ranges = Ranges::new(0x01..0x04,0x01..0x04); // String

/// The format of the file (1 = 32-bit, 2 = 64-bit)
pub const EI_CLASS: Ranges = Ranges::new(0x04..0x05,0x04..0x05); // u8

/// Endianness of the file (1 = little, 2 = big)
pub const EI_DATA: Ranges = Ranges::new(0x05..0x06,0x05..0x06); // u8

/// Version of ELF format (always 1)
pub const EI_VERSION: Ranges = Ranges::new(0x06..0x07,0x06..0x07); // u8

/// Identifies the target operating system ABI
pub const EI_OSABI: Ranges = Ranges::new(0x07..0x08,0x07..0x08); // u8

/// Further identifies specific version of ABI
pub const EI_ABIVERSION: Ranges = Ranges::new(0x08..0x09,0x08..0x09); // u8

/// Padding bytes that should always be filled with zeroes
pub const EI_PAD: Ranges = Ranges::new(0x09..0x10,0x09..0x10); // u64

/// The type of the object file
pub const E_TYPE: Ranges = Ranges::new(0x10..0x12,0x10..0x12); // u16

/// Specifies target instruction set architecture
pub const E_MACHINE: Ranges = Ranges::new(0x12..0x14,0x12..0x14); // u16

/// Set to 1 for the original ELF format
pub const E_VERSION: Ranges = Ranges::new(0x14..0x18,0x14..0x18); // u32

/// Offset of the entry point for execution (0 if none)
pub const E_ENTRY: Ranges = Ranges::new(0x18..0x1C, 0x18..0x20); // u32 / u64

/// Offset of the program header within the file
pub const E_PHOFF: Ranges = Ranges::new(0x1C..0x20, 0x20..0x28); // u32 / u64

/// Offset of the section header table
pub const E_SHOFF: Ranges = Ranges::new(0x20..0x24, 0x28..0x30); // u32 / u64

/// Target architecture-specific flags
pub const E_FLAGS: Ranges = Ranges::new(0x24..0x28, 0x30..0x34); // u32 / u32

/// Size of the header
pub const E_EHSIZE: Ranges = Ranges::new(0x28..0x2A, 0x34..0x36); // u16 / u16

/// Size of an entry in the program header
pub const E_PHENTSIZE: Ranges = Ranges::new(0x2A..0x2C, 0x36..0x38); // u16 / u16

/// Number of items in the program header
pub const E_PHNUM: Ranges = Ranges::new(0x2C..0x2E, 0x38..0x3A); // u16 / u16

/// Size of an entry in the section header
pub const E_SHENTSIZE: Ranges = Ranges::new(0x2E..0x30, 0x3A..0x3C); // u16 / u16

/// Number of items in the section header
pub const E_SHNUM: Ranges = Ranges::new(0x30..0x32, 0x3C..0x3E); // u16 / u16

/// Index of the section header that contains section names
pub const E_SHSTRNDX: Ranges = Ranges::new(0x32..0x34, 0x3E..0x40); // u16 / u16

/// Type of the segment
pub const P_TYPE: Ranges = Ranges::new(0x00..0x04,0x00..0x04);

/// Segment-specific flags
pub const P_FLAGS: Ranges = Ranges::new(0x18..0x1C,0x04..0x08);

/// Offset of the segment in the file
pub const P_OFFSET: Ranges = Ranges::new(0x04..0x08,0x08..0x10);

/// Virtual address of the segment in memory
pub const P_VADDR: Ranges = Ranges::new(0x08..0x0C,0x10..0x18);

/// Physical address of the segment
pub const P_PADDR: Ranges = Ranges::new(0x0C..0x10,0x18..0x20);

/// Size in bytes of the segment in the file
pub const P_FILESZ: Ranges = Ranges::new(0x10..0x14,0x20..0x28);

/// Size in bytes of the segment in memory
pub const P_MEMSZ: Ranges = Ranges::new(0x14..0x18,0x28..0x30);

/// Alignment of the file in memory (0/1 = no alignment, otherwise positive power of 2)
pub const P_ALIGN: Ranges = Ranges::new(0x1C..0x20,0x30..0x38);

/// Offset to a string name in the string table
pub const SH_NAME: Ranges = Ranges::new(0x00..0x04,0x00..0x04);

/// The type of the section header
pub const SH_TYPE: Ranges = Ranges::new(0x04..0x08,0x04..0x08);

/// Attributes of the section header
pub const SH_FLAGS: Ranges = Ranges::new(0x08..0x0C,0x08..0x10);

/// Virtual address of the section in memory
pub const SH_ADDR: Ranges = Ranges::new(0x0C..0x10,0x10..0x18);

/// Offset of the section in the file image
pub const SH_OFFSET: Ranges = Ranges::new(0x10..0x14,0x18..0x20);

/// The size of the section
pub const SH_SIZE: Ranges = Ranges::new(0x14..0x18,0x20..0x28);

/// Section index of an associated section
pub const SH_LINK: Ranges = Ranges::new(0x18..0x1C,0x28..0x2C);

/// Extra information about the section
pub const SH_INFO: Ranges = Ranges::new(0x1C..0x20,0x2C..0x30);

/// Required alignment of the section (must be power of 2)
pub const SH_ADDRALIGN: Ranges = Ranges::new(0x20..0x24,0x30..0x38);

/// Contains size of entries in bytes, if the section has entries
pub const SH_ENTSIZE: Ranges = Ranges::new(0x24..0x28,0x38..0x40);

/// Index of the symbols name in the string table
pub const ST_NAME: Ranges = Ranges::new(0x00..0x04,0x00..0x04); // u32 / u32

/// The value of the symbol table entry
pub const ST_VALUE: Ranges = Ranges::new(0x04..0x08,0x08..0x10); // u32 / u64

/// The size in bytes of the symbol data
pub const ST_SIZE: Ranges = Ranges::new(0x08..0x0c,0x10..0x18); // u32 / u64

/// Type and binding attributes of the symbol
pub const ST_INFO: Ranges = Ranges::new(0x0c..0x0d,0x04..0x05); // u8  / u8

/// Visibility of the symbol
pub const ST_OTHER: Ranges = Ranges::new(0x0d..0x0e,0x05..0x06); // u8  / u8

/// Index of the symbol's section in the section header table
pub const ST_SHNDX: Ranges = Ranges::new(0x0e..0x10,0x06..0x08); // u16 / u16

/// Offset of the relocation entry
pub const RT_OFFSET: Ranges = Ranges::new(0x00..0x04,0x00..0x08); // u32 / u64

/// Relocation info for relocation entry
pub const RT_INFO: Ranges = Ranges::new(0x04..0x08,0x08..0x10); // u32 / u64

/// Addend of the relocation, if found
pub const RT_ADDEND: Ranges = Ranges::new(0x08..0x0C,0x10..0x18); // i32 / i64

/// Range for extracting addresses from init/preinit/fini tables
pub const ADDRESS: Ranges = Ranges::new(0x00..0x04,0x00..0x08); // u32 / u64

/// This struct maintains several ranges and returns
/// them depending on the current width.
#[derive(Debug,Clone)]
pub struct Ranges {
    pub width: Width,
    pub x32: Range<usize>,
    pub x64: Range<usize>,
}

impl Ranges {

    /// Create a new set of ranges given a 32- and 64-bit Range
    pub const fn new(a: Range<usize>, b: Range<usize>) -> Self {
        Self {
            width: Width::X32,
            x32: a,
            x64: b,
        }
    }

    /// Create an empty set of ranges
    pub const fn empty() -> Self {
        Self {
            width: Width::X32,
            x32: 0x00..0x00,
            x64: 0x00..0x00,
        }
    }

    /// Get a reference to the current Range
    pub fn at(&self) -> &Range<usize> {
        match self.width {
            Width::X32 => &self.x32,
            Width::X64 => &self.x64,
        }
    }

    /// Get at mutable reference to the current Range
    pub fn at_mut(&mut self) -> &mut Range<usize> {
        match self.width {
            Width::X32 => &mut self.x32,
            Width::X64 => &mut self.x64,
        }
    }

    /// Get the active Range depending on the current Width
    pub fn get(&self) -> Range<usize> {
        self.at().clone()
    }

    /// Get the expected size in bytes of the current range
    pub fn size(&self) -> usize {
        self.get().len()
    }

}

/// Implementation of Into so we can use Ranges to slice
impl Into<Range<usize>> for &Ranges {
    fn into(self) -> Range<usize> {
        self.get()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_range_get() {
        let ranges = Ranges::new(0x01..0x04, 0x01..0x04);
        let check = ranges.get();
        assert_eq!(check.start,0x01);
        assert_eq!(check.end,0x04);
    }

    #[test]
    fn test_complex_range_change_and_get() {
        let mut ranges = Ranges::new(0x00..0x04, 0x00..0x08);

        ranges.width = Width::X64;
        let range1 = ranges.get();

        ranges.width = Width::X32;
        let range2 = ranges.get();

        assert_eq!(range1.end,0x08);
        assert_eq!(range2.end,0x04);
    }
}