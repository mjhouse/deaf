pub const ELF_SIZE_32:    usize = 52;
pub const ELF_SIZE_64:    usize = 64;

// pub const EI_MAGIC:       Range<usize> = 0x00..0x04;
// pub const EI_CLASS:       Range<usize> = 0x04..0x05;
// pub const EI_DATA:        Range<usize> = 0x05..0x06;
// pub const EI_VERSION:     Range<usize> = 0x06..0x07;
// pub const EI_OSABI:       Range<usize> = 0x07..0x08;
// pub const EI_ABIVERSION:  Range<usize> = 0x08..0x09;
// pub const EI_PAD:         Range<usize> = 0x09..0x10;
// pub const E_TYPE:         Range<usize> = 0x10..0x12;
// pub const E_MACHINE:      Range<usize> = 0x12..0x14;
// pub const E_VERSION:      Range<usize> = 0x14..0x18;
// pub const E_ENTRY_32:     Range<usize> = 0x18..0x1C;
// pub const E_ENTRY_64:     Range<usize> = 0x18..0x20;
// pub const E_PHOFF_32:     Range<usize> = 0x1C..0x20;
// pub const E_PHOFF_64:     Range<usize> = 0x20..0x28;
// pub const E_SHOFF_32:     Range<usize> = 0x20..0x24;
// pub const E_SHOFF_64:     Range<usize> = 0x28..0x30;
// pub const E_FLAGS_32:     Range<usize> = 0x24..0x28;
// pub const E_FLAGS_64:     Range<usize> = 0x30..0x34;
// pub const E_EHSIZE_32:    Range<usize> = 0x28..0x2A;
// pub const E_EHSIZE_64:    Range<usize> = 0x34..0x36;
// pub const E_PHENTSIZE_32: Range<usize> = 0x2A..0x2C;
// pub const E_PHENTSIZE_64: Range<usize> = 0x36..0x38;
// pub const E_PHNUM_32:     Range<usize> = 0x2C..0x2E;
// pub const E_PHNUM_64:     Range<usize> = 0x38..0x3A;
// pub const E_SHENTSIZE_32: Range<usize> = 0x2E..0x30;
// pub const E_SHENTSIZE_64: Range<usize> = 0x3A..0x3C;
// pub const E_SHNUM_32:     Range<usize> = 0x30..0x32;
// pub const E_SHNUM_64:     Range<usize> = 0x3C..0x3E;
// pub const E_SHSTRNDX_32:  Range<usize> = 0x32..0x34;
// pub const E_SHSTRNDX_64:  Range<usize> = 0x3E..0x40;

// the addressing width of the ELF file
#[derive(Debug, Clone)]
pub enum Width {
    Any,
    X32,
    X64,
}

// the endianness of the ELF file
#[derive(Debug, Clone)]
pub enum Layout {
    Any,
    Little,
    Big,
}
