use std::ops::Range;
use crate::field::Field;

pub const FH_SIZE_32:    usize = 52;
pub const FH_SIZE_64:    usize = 64;

pub const EI_MAGIC:       (usize,usize) = (0x00,0x04);
pub const EI_CLASS:       (usize,usize) = (0x04,0x05);
pub const EI_DATA:        (usize,usize) = (0x05,0x06);
pub const EI_VERSION:     (usize,usize) = (0x06,0x07);
pub const EI_OSABI:       (usize,usize) = (0x07,0x08);
pub const EI_ABIVERSION:  (usize,usize) = (0x08,0x09);
pub const EI_PAD:         (usize,usize) = (0x09,0x10);

pub const E_TYPE:         (usize,usize) = (0x10,0x12);
pub const E_MACHINE:      (usize,usize) = (0x12,0x14);
pub const E_VERSION:      (usize,usize) = (0x14,0x18);

pub const E_ENTRY_32:     (usize,usize) = (0x18,0x1C);
pub const E_ENTRY_64:     (usize,usize) = (0x18,0x20);

pub const E_PHOFF_32:     (usize,usize) = (0x1C,0x20);
pub const E_PHOFF_64:     (usize,usize) = (0x20,0x28);

pub const E_SHOFF_32:     (usize,usize) = (0x20,0x24);
pub const E_SHOFF_64:     (usize,usize) = (0x28,0x30);

pub const E_FLAGS_32:     (usize,usize) = (0x24,0x28);
pub const E_FLAGS_64:     (usize,usize) = (0x30,0x34);

pub const E_EHSIZE_32:    (usize,usize) = (0x28,0x2A);
pub const E_EHSIZE_64:    (usize,usize) = (0x34,0x36);

pub const E_PHENTSIZE_32: (usize,usize) = (0x2A,0x2C);
pub const E_PHENTSIZE_64: (usize,usize) = (0x36,0x38);

pub const E_PHNUM_32:     (usize,usize) = (0x2C,0x2E);
pub const E_PHNUM_64:     (usize,usize) = (0x38,0x3A);

pub const E_SHENTSIZE_32: (usize,usize) = (0x2E,0x30);
pub const E_SHENTSIZE_64: (usize,usize) = (0x3A,0x3C);

pub const E_SHNUM_32:     (usize,usize) = (0x30,0x32);
pub const E_SHNUM_64:     (usize,usize) = (0x3C,0x3E);

pub const E_SHSTRNDX_32:  (usize,usize) = (0x32,0x34);
pub const E_SHSTRNDX_64:  (usize,usize) = (0x3E,0x40);

// type defs for specific fields from the program header
pub type FileMagicField   = Field<FileMagic,{EI_MAGIC.0},{EI_MAGIC.1}>;
pub type FileClassField   = Field<FileClass,{EI_CLASS.0},{EI_CLASS.1}>;
pub type FileLayoutField  = Field<FileLayout,{EI_DATA.0},{EI_DATA.1}>;
pub type FileVersionField = Field<FileVersion,{EI_VERSION.0},{EI_VERSION.1}>;
pub type ABITypeField     = Field<ABIType,{EI_OSABI.0},{EI_OSABI.1}>;
pub type ABIVersionField  = Field<ABIVersion,{EI_ABIVERSION.0},{EI_ABIVERSION.1}>;

// trait for enum field values that can convert to/from slices
pub trait FieldValue {

    fn from_slice(b: &[u8]) -> Self;

    fn into_slice(&self) -> &[u8];

}

macro_rules! impl_field_value_from {
    ( $enum: ident, [ $( $value:ident => $raw:pat ),* ]) => {
        fn from_slice(b: &[u8]) -> Self {
            match b {
                $( &$raw => Self::$value, )*
                v => Self::Unknown(v.into()),
            }
        }
    }
}

macro_rules! impl_field_value_into {
    ( $enum: ident, [ $( $value:ident => $raw:expr ),* ]) => {
        fn into_slice(&self) -> &[u8] {
            match self {
                $( Self::$value => &$raw, )*
                Self::Unknown(v) => &v,
            }
        }
    }
}

macro_rules! impl_field_value {
    ( $enum: ident, [ $( $value:ident => $raw:tt ),* ] ) => {
        #[derive(Debug,PartialEq)]
        pub enum $enum {
            $( $value, )*
            Unknown(Box<[u8]>)
        }

        impl FieldValue for $enum {
            impl_field_value_from!($enum,[ $( $value => $raw ),* ]);
            impl_field_value_into!($enum,[ $( $value => $raw ),* ]);
        }

        impl $enum {

            pub fn is_unknown(&self) -> bool {
                match self {
                    Self::Unknown(_) => true,
                    _ => false
                }
            }

        }
    }
}

// address size of target system
impl_field_value!(FileMagic,[
    Valid => [ 0x7F, b'E', b'L', b'F' ]
]);

// address size of target system
impl_field_value!(FileClass,[
    None          => [ 0x00 ],
    X32Bit        => [ 0x01 ],
    X64Bit        => [ 0x02 ]
]);

// endianness of the elf file
impl_field_value!(FileLayout,[
    None          => [ 0x00 ],
    Lsb           => [ 0x01 ],
    Msb           => [ 0x02 ]
]);

// should always be `1`for current
impl_field_value!(FileVersion,[
    Current       => [ 0x01 ]
]);

// the ABI used by this elf file
impl_field_value!(ABIType,[
    SystemV       => [ 0x00 ],
    HPUX          => [ 0x01 ],
    NetBSD        => [ 0x02 ],
    Linux         => [ 0x03 ],
    GNUHurd       => [ 0x04 ],
    Solaris       => [ 0x06 ],
    AIXMonterey   => [ 0x07 ],
    IRIX          => [ 0x08 ],
    FreeBSD       => [ 0x09 ],
    Tru64         => [ 0x0A ],
    NovellModesto => [ 0x0B ],
    OpenBSD       => [ 0x0C ],
    OpenVMS       => [ 0x0D ],
    NonStopKernel => [ 0x0E ],
    AROS          => [ 0x0F ],
    FenixOS       => [ 0x10 ],
    NuxiCloudABI  => [ 0x11 ],
    STOpenVOS     => [ 0x12 ]
]);

// only used by glibc >= 2.12
impl_field_value!(ABIVersion,[]);

// // type of ELF file- executable, lib etc.
// impl_field_value!(FileTypeEx,[
//     Relocatable       => [ 0x01 ], // 0x01
//     Executable        => [ 0x02 ], // 0x02
//     SharedFile        => [ 0x03 ], // 0x03
//     CoreFile          => [ 0x04 ]  // 0x04
//     // ReservedOperating // 0xFE00 - 0xFEFF
//     // ReservedProcessor // 0xFF00 - 0xFFFF
// ]);

// // target instruction set architecture
// pub enum Machine {
//     None,              // 0x00
//     AttWe32100,        // 0x01
//     SPARC,             // 0x02
//     X86,               // 0x03
//     M68k,              // 0x04
//     M88k,              // 0x05
//     IntelMCU,          // 0x06
//     Intel80860,        // 0x07
//     MIPS,              // 0x08
//     IBMSystem370,      // 0x09
//     MIPSRs3k,          // 0x0A
//     // Reserved           0x0B - 0x0D
//     HPPARISC,          // 0x0E
//     // Reserved           0x0F
//     Intel80960,        // 0x13
//     PowerPC32,         // 0x14
//     PowerPC64,         // 0x15
//     S390,              // 0x16
//     IBMSPUSPC,         // 0x17
//     // Reserved           0x18 - 0x23
//     NECV800,           // 0x24
//     FujitsuFR20,       // 0x25
//     TRWRH32,           // 0x26
//     MotorolaRCE,       // 0x27
//     Arm32,             // 0x28
//     DigitalAlpha,      // 0x29
//     SuperH,            // 0x2A
//     SPARCv9,           // 0x2B
//     SiemensTriCore,    // 0x2C
//     ArgonautRISCCore,  // 0x2D
//     HitachiH8300,      // 0x2E
//     HitachiH8300H,     // 0x2F
//     HitachiH8S,        // 0x30
//     HitachiH8500,      // 0x31
//     IA64,              // 0x32
//     StanfordMIPSX,     // 0x33
//     MotorolaColdFire,  // 0x34
//     MotorolaM68HC12,   // 0x35
//     FujitsuMMA,        // 0x36
//     SiemensPCP,        // 0x37
//     SonynCPURISC,      // 0x38
//     DensoNDR1,         // 0x39
//     MotorolaStarCore,  // 0x3A
//     ToyotaME16,        // 0x3B
//     STMST100,          // 0x3C
//     ALCTinyJ,          // 0x3D
//     AMDx8664,          // 0x3E
//     SonyDSP,           // 0x3F
//     DECPDP10,          // 0x40
//     DECPDP11,          // 0x41
//     SiemensFX66,       // 0x42
//     STMST9Plus816,     // 0x43
//     STMST78Bit,        // 0x44
//     MotorolaMC68HC16,  // 0x45
//     MotorolaMC68HC11,  // 0x46
//     MotorolaMC68HC08,  // 0x47
//     MotorolaMC68HC05,  // 0x48
//     SiliconGSVx,       // 0x49
//     STMST198Bit,       // 0x4A
//     DigitalVAX,        // 0x4B
//     AxisC32Bit,        // 0x4C
//     InfineonT32Bit,    // 0x4D
//     Element1464BitDSP, // 0x4E
//     LSILogic16BitDSP,  // 0x4F
//     TMS320C6000,       // 0x8C
//     MCSTElbrusE2k,     // 0xAF
//     AArch64,           // 0xB7
//     ZilogZ80,          // 0xDC
//     RISCV,             // 0xF3
//     BerkeleyPF,        // 0xF7
//     WDC65C816,         // 0x101
//     Unknown(u16),      // _
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_from_binary_filetype_none() {
        let bytes = [ 0x0, 0x1, 0x2, 0x3, 0x0 ];
        let field = FileClassField::new();
        let value = field.read(&bytes).unwrap();
        assert_eq!(value,FileClass::None);
    }

    #[test]
    fn test_read_from_binary_filetype_32() {
        let bytes = [ 0x0, 0x1, 0x2, 0x3, 0x1 ];
        let field = FileClassField::new();
        let value = field.read(&bytes).unwrap();
        assert_eq!(value,FileClass::X32Bit);
    }

    #[test]
    fn test_read_from_binary_filetype_64() {
        let bytes = [ 0x0, 0x1, 0x2, 0x3, 0x2 ];
        let field = FileClassField::new();
        let value = field.read(&bytes).unwrap();
        assert_eq!(value,FileClass::X64Bit);
    }

    #[test]
    fn test_read_from_binary_filetype_unknown() {
        let bytes = [ 0x0, 0x1, 0x2, 0x3, 0x3 ];
        let field = FileClassField::new();
        let value = field.read(&bytes).unwrap();
        assert!(value.is_unknown());
    }

    #[test]
    fn test_read_from_binary_filelayout_none() {
        let bytes = [ 0x0, 0x1, 0x2, 0x3, 0x0, 0x0 ];
        let field = FileLayoutField::new();
        let value = field.read(&bytes).unwrap();
        assert_eq!(value,FileLayout::None);
    }

    #[test]
    fn test_read_from_binary_filelayout_lsb() {
        let bytes = [ 0x0, 0x1, 0x2, 0x3, 0x0, 0x1 ];
        let field = FileLayoutField::new();
        let value = field.read(&bytes).unwrap();
        assert_eq!(value,FileLayout::Lsb);
    }

    #[test]
    fn test_read_from_binary_filelayout_msb() {
        let bytes = [ 0x0, 0x1, 0x2, 0x3, 0x0, 0x2 ];
        let field = FileLayoutField::new();
        let value = field.read(&bytes).unwrap();
        assert_eq!(value,FileLayout::Msb);
    }

    #[test]
    fn test_read_from_binary_filelayout_unknown() {
        let bytes = [ 0x0, 0x1, 0x2, 0x3, 0x0, 0x3 ];
        let field = FileLayoutField::new();
        let value = field.read(&bytes).unwrap();
        assert!(value.is_unknown());
    }

    #[test]
    fn test_read_from_binary_fileversion_current() {
        let bytes = [ 0x0, 0x1, 0x2, 0x3, 0x0, 0x0, 0x1 ];
        let field = FileVersionField::new();
        let value = field.read(&bytes).unwrap();
        assert_eq!(value,FileVersion::Current);
    }

    #[test]
    fn test_read_from_binary_fileversion_unknown() {
        let bytes = [ 0x0, 0x1, 0x2, 0x3, 0x0, 0x0, 0x2 ];
        let field = FileVersionField::new();
        let value = field.read(&bytes).unwrap();
        assert!(value.is_unknown());
    }
}
