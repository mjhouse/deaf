
/*
    Create a getter, setter and accessor:

    ```
    property!(class,ei_class,Width)
    ```

    expands to:

    ```
    // no-mut access to already-parsed value
    pub fn class(&self) -> Width;

    // re-parse a single value from the binary
    pub fn get_class(&self, b: &[u8]) -> Result<Width>;

    // set a new value in the binary
    pub fn set_class(&mut self, b: &mut [u8], v: Width) -> Result<()>;
    ```

*/
#[macro_export]
macro_rules! impl_property {
    ( $n: ident, $f: ident, $v: ident ) => {
        paste::paste!{
            pub fn $n(&self) -> $v {
                self.values.$f.clone()
            }
        
            pub fn [< get_ $n >](&self, b: &[u8]) -> Result<$v> {
                self.$f.get(b)
            }
        
            pub fn [< set_ $n >](&mut self, b: &mut [u8], v: $v) -> Result<()> {
                self.$f.set(b,v.clone())?;
                self.values.$f = v;
                Ok(())
            }
        }
    }
}

// macro_rules! impl_try_from {
//     ( $f: ident, $t: ident, $( $n: pat => $m: ident ),+ ) => {
//         impl TryFrom<$f> for $t {
//             type Error = Error;
//             fn try_from(f: $f) -> Result<Self> {
//                 match f {
//                     $( $n => Ok(Self::$m), )+
//                     _ => Err(Error::ParseError),
//                 }
//             }
//         }
//     }
// }

// macro_rules! impl_try_from_nofail {
//     ( $f: ident, $t: ident, $( $n: pat => $m: ident ),+ ) => {
//         impl TryFrom<$f> for $t {
//             type Error = Error;
//             fn try_from(f: $f) -> Result<Self> {
//                 match f {
//                     $( $n => Ok(Self::$m), )+
//                     v => Ok(Self::Unknown(v)),
//                 }
//             }
//         }
//     }
// }

// macro_rules! impl_into {
//     ( $f: ident, $t: ident, $( $n: expr => $m: ident ),+ ) => {
//         impl Into<$f> for $t {
//             fn into(self) -> $f {
//                 match self {
//                     $( Self::$m => $n, )+
//                 }
//             }
//         }
//     }
// }

// macro_rules! impl_into_nofail {
//     ( $f: ident, $t: ident, $( $n: expr => $m: ident ),+ ) => {
//         impl Into<$f> for $t {
//             fn into(self) -> $f {
//                 match self {
//                     $( Self::$m => $n, )+
//                     Self::Unknown(v) => v
//                 }
//             }
//         }
//     }
// }

// macro_rules! impl_constant {
//     ( $f: ident, $t: ident, [ $( $n: expr => $m: ident ),+ ] ) => {
//         use crate::errors::{Error, Result};
        
//         #[allow(non_camel_case_types)]
//         #[derive(Debug, Clone, Copy, PartialEq)]
//         pub enum $t {
//             $( $m, )+
//         }

//         impl_try_from!($f,$t, $( $n => $m ),+);
//         impl_into!($f,$t, $( $n => $m ),+);
//     }
// }

// macro_rules! impl_constant_nofail {
//     ( $f: ident, $t: ident, [ $( $n: expr => $m: ident ),+ ] ) => {
//         use crate::errors::{Error, Result};

//         #[allow(non_camel_case_types)]
//         #[derive(Debug, Clone, Copy, PartialEq)]
//         pub enum $t {
//             $( $m, )+
//             Unknown($f)
//         }

//         impl_try_from_nofail!($f,$t, $( $n => $m ),+);
//         impl_into_nofail!($f,$t, $( $n => $m ),+);
//     }
// }

macro_rules! impl_constant {
    ( $f: ident, $t: ident, [ $( $n: tt => $m: ident ),+ ] ) => {

        #[allow(non_camel_case_types)]
        #[derive(Debug, Clone, Copy, PartialEq, num_enum::IntoPrimitive, num_enum::TryFromPrimitive)]
        #[repr($f)]
        pub enum $t {
            $( $m ),+
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl_constant!(u32, SHType, [
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

    #[test]
    fn test_convert_constant_to_u32() {
        let sht_null: u32 = SHType::SHT_NULL.into();
        assert_eq!(sht_null, 0u32);
    }

    #[test]
    fn test_convert_constant_from_u32() {
        let sht_null = SHType::try_from(0u32);
        assert!(sht_null.is_ok());
        assert_eq!(sht_null.unwrap(), SHType::SHT_NULL);
    }

}