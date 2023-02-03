use crate::errors::{Error, Result};

pub const ELF_SIZE_32: usize = 52;
pub const ELF_SIZE_64: usize = 64;

pub const PH_SIZE_32: usize = 32;
pub const PH_SIZE_64: usize = 56;

macro_rules! impl_try_from_no_fail {
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

macro_rules! impl_constant {
    ( $f: ident, $t: ident, $( $n: expr => $m: ident ),+ ) => {
        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, PartialEq)]
        pub enum $t {
            $( $m, )+
        }

        impl_try_from!($f,$t, $( $n => $m ),+);
        impl_into!($f,$t, $( $n => $m ),+);
    }
}

impl_constant!(u8,Width,
    2 => X64,
    1 => X32
);

impl_constant!(u8,Layout,
    2 => Big,
    1 => Little
);

/*
    For GNU_PROPERTY and GNU_EH_FRAME see: 
        https://raw.githubusercontent.com/wiki/hjl-tools/linux-abi/linux-abi-draft.pdf
    Other ELF format info:
        http://www.skyfree.org/linux/references/ELF_Format.pdf
*/
impl_constant!(u32,PHType,
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
    0x6fffffff => PT_HIOS,         //   Uppder bound of OS-specific types
    0x70000000 => PT_LOPROC,       //   Lower bound of processor-specific types
    0x7fffffff => PT_HIPROC        //   Upper bound of processor-specific types
);