use crate::errors::{Error, Result};

// global const section sizes for various widths
pub const FH_SIZE_32: usize = 52;
pub const FH_SIZE_64: usize = 64;

pub const PH_SIZE_32: usize = 32;
pub const PH_SIZE_64: usize = 56;

pub const SH_SIZE_32: usize = 40;
pub const SH_SIZE_64: usize = 64;

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

}

macro_rules! impl_try_from {
    ( $f: ident, $t: ident, $( $n: pat => $m: ident ),+ ) => {
        impl TryFrom<$f> for $t {
            type Error = Error;
            fn try_from(f: $f) -> Result<Self> {
                match f {
                    $( $n => Ok(Self::$m), )+
                    _ => Err(Error::ParseError),
                }
            }
        }
    }
}

macro_rules! impl_try_from_nofail {
    ( $f: ident, $t: ident, $( $n: pat => $m: ident ),+ ) => {
        impl TryFrom<$f> for $t {
            type Error = Error;
            fn try_from(f: $f) -> Result<Self> {
                match f {
                    $( $n => Ok(Self::$m), )+
                    v => Ok(Self::Unknown(v)),
                }
            }
        }
    }
}

macro_rules! impl_into {
    ( $f: ident, $t: ident, $( $n: expr => $m: ident ),+ ) => {
        impl Into<$f> for $t {
            fn into(self) -> $f {
                match self {
                    $( Self::$m => $n, )+
                }
            }
        }
    }
}

macro_rules! impl_into_nofail {
    ( $f: ident, $t: ident, $( $n: expr => $m: ident ),+ ) => {
        impl Into<$f> for $t {
            fn into(self) -> $f {
                match self {
                    $( Self::$m => $n, )+
                    Self::Unknown(v) => v
                }
            }
        }
    }
}

macro_rules! impl_constant {
    ( $f: ident, $t: ident, [ $( $n: expr => $m: ident ),+ ] ) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum $t {
            $( $m, )+
        }

        impl_try_from!($f,$t, $( $n => $m ),+);
        impl_into!($f,$t, $( $n => $m ),+);
    }
}

macro_rules! impl_constant_nofail {
    ( $f: ident, $t: ident, [ $( $n: expr => $m: ident ),+ ] ) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum $t {
            $( $m, )+
            Unknown($f)
        }

        impl_try_from_nofail!($f,$t, $( $n => $m ),+);
        impl_into_nofail!($f,$t, $( $n => $m ),+);
    }
}

impl_constant!(u8, Width, [
    2 => X64,   // 64-bit address widths
    1 => X32    // 32-bit address widths
]);

impl_constant!(u8, Layout, [
    2 => Big,   // Big endian (e.g. 0xABCD is represented as 'AB CD' )
    1 => Little // Little endian (e.g. 0xABCD is represented as 'CD AB')
]);

impl_constant_nofail!(u32, PHType, [
    0x00000000 => PT_NULL,         // 	Program header table entry unused.
    0x00000001 => PT_LOAD,         // 	Loadable segment.
    0x00000002 => PT_DYNAMIC,      // 	Dynamic linking information.
    0x00000003 => PT_INTERP,       // 	Interpreter information.
    0x00000004 => PT_NOTE,         // 	Auxiliary information.
    0x00000005 => PT_SHLIB,        // 	Reserved.
    0x00000006 => PT_PHDR,         // 	Segment containing program header table itself.
    0x00000007 => PT_TLS,          // 	Thread-Local Storage template.
    0x60000000 => PT_LOOS,         //   Lower bound of OS-specific types
    0x6474e550 => PT_GNU_EH_FRAME, //   OS-specific location of .eh_frame section for stack unwinding
    0x6474e553 => PT_GNU_PROPERTY, //   OS-specific location of .note.gnu.property section  for special loader notes
    0x6474e551 => PT_GNU_STACK,    //   OS-specific location of stack segment?
    0x6474e552 => GNU_RELRO,       //   OS-specific segment to be made read-only after linking

    // add other os-specific types here

    0x6fffffff => PT_HIOS,         //   Uppder bound of OS-specific types
    0x70000000 => PT_LOPROC,       //   Lower bound of processor-specific types

    // add other processor specific types here

    0x7fffffff => PT_HIPROC        //   Upper bound of processor-specific types
]);

impl_constant_nofail!(u32, SHType, [
    0x00000000 => SHT_NULL,          //   Section header table entry unused
    0x00000001 => SHT_PROGBITS,      //   Program data
    0x00000002 => SHT_SYMTAB,        //   Symbol table
    0x00000003 => SHT_STRTAB,        //   String table
    0x00000004 => SHT_RELA,          //   Relocation entries with addends
    0x00000005 => SHT_HASH,          //   Symbol hash table
    0x00000006 => SHT_DYNAMIC,       //   Dynamic linking information
    0x00000007 => SHT_NOTE,          //   Notes
    0x00000008 => SHT_NOBITS,        //   Program space with no data (bss)
    0x00000009 => SHT_REL,           //   Relocation entries, no addends
    0x0000000A => SHT_SHLIB,         //   Reserved
    0x0000000B => SHT_DYNSYM,        //   Dynamic linker symbol table
    0x0000000E => SHT_INIT_ARRAY,    //   Array of constructors
    0x0000000F => SHT_FINI_ARRAY,    //   Array of destructors
    0x00000010 => SHT_PREINIT_ARRAY, //   Array of pre-constructors
    0x00000011 => SHT_GROUP,         //   Section group
    0x00000012 => SHT_SYMTAB_SHNDX,  //   Extended section indices
    0x00000013 => SHT_NUM,           //   Number of defined types.
    0x60000000 => SHT_LOOS           //   Start OS-specific. 
]);