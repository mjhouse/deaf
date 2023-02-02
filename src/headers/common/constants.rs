use crate::errors::{Error, Result};

pub const ELF_SIZE_32: usize = 52;
pub const ELF_SIZE_64: usize = 64;

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