use num_enum::{IntoPrimitive, TryFromPrimitive, FromPrimitive};
use enumflags2::{bitflags, make_bitflags, BitFlags};
use crate::errors::{Error, Result};

// global const sizes for various widths
pub const FH_SIZE_32: usize = 52;
pub const FH_SIZE_64: usize = 64;

pub const PH_SIZE_32: usize = 32;
pub const PH_SIZE_64: usize = 56;

pub const SH_SIZE_32: usize = 40;
pub const SH_SIZE_64: usize = 64;

pub const ST_SIZE_32: usize = 16;
pub const ST_SIZE_64: usize = 24;

pub const RT_SIZE_32: usize = 16;
pub const RT_SIZE_64: usize = 24;

pub mod sizes {
    use super::*;

    pub fn file_header(width: Width) -> usize {
        match width {
            Width::X32 => FH_SIZE_32,
            Width::X64 => FH_SIZE_64,
        }
    }

    pub fn program_header(width: Width) -> usize {
        match width {
            Width::X32 => PH_SIZE_32,
            Width::X64 => PH_SIZE_64,
        }
    }

    pub fn section_header(width: Width) -> usize {
        match width {
            Width::X32 => SH_SIZE_32,
            Width::X64 => SH_SIZE_64,
        }
    }

    pub fn symbol_table(width: Width) -> usize {
        match width {
            Width::X32 => ST_SIZE_32,
            Width::X64 => ST_SIZE_64,
        }
    }

}

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum Width {
    X32 = 0x01, // Little endian (e.g. 0xABCD is represented as 'CD AB')
    X64 = 0x02  // Big endian (e.g. 0xABCD is represented as 'AB CD' )
}

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum Layout {
    Little = 0x01, // Little endian (e.g. 0xABCD is represented as 'CD AB')
    Big = 0x02     // Big endian (e.g. 0xABCD is represented as 'AB CD' )
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum PHType {
    PT_NULL = 0x00000000,         // 	Program header table entry unused.
    PT_LOAD = 0x00000001,         // 	Loadable segment.
    PT_DYNAMIC = 0x00000002,      // 	Dynamic linking information.
    PT_INTERP = 0x00000003,       // 	Interpreter information.
    PT_NOTE = 0x00000004,         // 	Auxiliary information.
    PT_SHLIB = 0x00000005,        // 	Reserved.
    PT_PHDR = 0x00000006,         // 	Segment containing program header table itself.
    PT_TLS = 0x00000007,          // 	Thread-Local Storage template.
    PT_LOOS = 0x60000000,         //   Lower bound of OS-specific types
    PT_GNU_EH_FRAME = 0x6474e550, //   OS-specific location of .eh_frame section for stack unwinding
    PT_GNU_PROPERTY = 0x6474e553, //   OS-specific location of .note.gnu.property section  for special loader notes
    PT_GNU_STACK = 0x6474e551,    //   OS-specific location of stack segment?
    GNU_RELRO = 0x6474e552,       //   OS-specific segment to be made read-only after linking

    // add other os-specific types here

    PT_HIOS = 0x6fffffff,         //   Uppder bound of OS-specific types
    PT_LOPROC = 0x70000000,       //   Lower bound of processor-specific types

    // add other processor specific types here

    PT_HIPROC = 0x7fffffff,       //   Upper bound of processor-specific types
    #[num_enum(catch_all)]
    Unknown(u32)
}

#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, IntoPrimitive, FromPrimitive)]
pub enum SHType {
    SHT_NULL = 0x00000000,
    SHT_PROGBITS = 0x00000001,
    SHT_SYMTAB = 0x00000002,
    SHT_STRTAB = 0x00000003,
    SHT_RELA = 0x00000004,
    SHT_HASH = 0x00000005,
    SHT_DYNAMIC = 0x00000006,
    SHT_NOTE = 0x00000007,
    SHT_NOBITS = 0x00000008,
    SHT_REL = 0x00000009,
    SHT_SHLIB = 0x0000000A,
    SHT_DYNSYM = 0x0000000B,
    SHT_INIT_ARRAY = 0x0000000E,
    SHT_FINI_ARRAY = 0x0000000F,
    SHT_PREINIT_ARRAY = 0x00000010,
    SHT_GROUP = 0x00000011,
    SHT_SYMTAB_SHNDX = 0x00000012,
    SHT_NUM = 0x00000013,
    SHT_LOOS = 0x60000000,
    #[num_enum(catch_all)]
    Unknown(u32),
}

#[bitflags]
#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SHFlags {
    SHF_WRITE            = 0x00000001, //   Contains data that is writable during process execution. 
    SHF_ALLOC            = 0x00000002, //   Occupies memory during process execution.
    SHF_EXECINSTR        = 0x00000004, //   Contains executable machine instructions. 
    SHF_MERGE            = 0x00000010, //   Identifies a section containing data that may be merged to eliminate duplication
    SHF_STRINGS          = 0x00000020, //   Identifies a section that consists of null-terminated character strings
    SHF_INFO_LINK        = 0x00000040, //   This section headers sh_info field holds a section header table index.
    SHF_LINK_ORDER       = 0x00000080, //   This section adds special ordering requirements to the link-editor
    SHF_OS_NONCONFORMING = 0x00000100, //   This section requires special OS-specific processing
    SHF_GROUP            = 0x00000200, //   This section is a member, perhaps the only one, of a section group
    SHF_TLS              = 0x00000400, //   This section holds thread-local storage
    // SHF_MASKOS           = 0x0ff00000, //   All bits included in this mask are reserved for operating system-specific semantics.
    SHF_ORDERED          = 0x40000000, //   This section requires ordering in relation to other sections of the same type
    SHF_EXCLUDE          = 0x80000000, //   This section is excluded from input to the link-edit of an executable or shared object
    // SHF_MASKPROC         = 0xf0000000, //   Reserved for processor-specific semantics. 
}

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum STBind {
    STB_LOCAL   = 0x00,
    STB_GLOBAL  = 0x01,
    STB_WEAK    = 0x02,
}

#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum STType {
    STT_NOTYPE  = 0x00,
    STT_OBJECT  = 0x01,
    STT_FUNC    = 0x02,
    STT_SECTION = 0x03,
    STT_FILE    = 0x04,
    STT_COMMON  = 0x05,
    STT_TLS     = 0x06,
}