use crate::common::SHType;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SectionType {
    Null,              // SHT_NULL
    Program,           // SHT_PROGBITS
    Symbols,           // SHT_SYMTAB
    DynamicSymbols,    // SHT_DYNSYM
    ExtendedSymbols,   // SHT_SYMTAB_SHNDX
    Strings,           // SHT_STRTAB
    Relocations,       // SHT_REL
    RelocationsAddend, // SHT_RELA
    Hash,              // SHT_HASH
    GNUHash,           // SHT_GNU_HASH
    Dynamic,           // SHT_DYNAMIC
    Notes,             // SHT_NOTE
    Empty,             // SHT_NOBITS
    Reserved,          // SHT_SHLIB
    GNULibList,        // SHT_GNU_LIBLIST
    InitArray,         // SHT_INIT_ARRAY
    FiniArray,         // SHT_FINI_ARRAY
    PreInitArray,      // SHT_PREINIT_ARRAY
    Group,             // SHT_GROUP
    ReservedTypes,     // SHT_NUM
    Unknown,           // Unknown(u32)
}

impl From<SHType> for SectionType {
    fn from(v: SHType) -> Self {
        match v {
            SHType::SHT_NULL => SectionType::Null,
            SHType::SHT_PROGBITS => SectionType::Program,
            SHType::SHT_SYMTAB => SectionType::Symbols,
            SHType::SHT_STRTAB => SectionType::Strings,
            SHType::SHT_RELA => SectionType::RelocationsAddend,
            SHType::SHT_HASH => SectionType::Hash,
            SHType::SHT_GNU_HASH => SectionType::GNUHash,
            SHType::SHT_DYNAMIC => SectionType::Dynamic,
            SHType::SHT_NOTE => SectionType::Notes,
            SHType::SHT_NOBITS => SectionType::Empty,
            SHType::SHT_REL => SectionType::Relocations,
            SHType::SHT_SHLIB => SectionType::Reserved,
            SHType::SHT_GNU_LIBLIST => SectionType::GNULibList,
            SHType::SHT_DYNSYM => SectionType::DynamicSymbols,
            SHType::SHT_INIT_ARRAY => SectionType::InitArray,
            SHType::SHT_FINI_ARRAY => SectionType::FiniArray,
            SHType::SHT_PREINIT_ARRAY => SectionType::PreInitArray,
            SHType::SHT_GROUP => SectionType::Group,
            SHType::SHT_SYMTAB_SHNDX => SectionType::ExtendedSymbols,
            SHType::SHT_NUM => SectionType::ReservedTypes,
            SHType::SHT_LOOS => SectionType::Unknown,
            _ => SectionType::Unknown,
        }
    }
}

impl From<SectionType> for SHType {
    fn from(v: SectionType) -> Self {
        match v {
            SectionType::Null => SHType::SHT_NULL,
            SectionType::Program => SHType::SHT_PROGBITS,
            SectionType::Symbols => SHType::SHT_SYMTAB,
            SectionType::Strings => SHType::SHT_STRTAB,
            SectionType::RelocationsAddend => SHType::SHT_RELA,
            SectionType::Hash => SHType::SHT_HASH,
            SectionType::GNUHash => SHType::SHT_GNU_HASH,
            SectionType::Dynamic => SHType::SHT_DYNAMIC,
            SectionType::Notes => SHType::SHT_NOTE,
            SectionType::Empty => SHType::SHT_NOBITS,
            SectionType::Relocations => SHType::SHT_REL,
            SectionType::Reserved => SHType::SHT_SHLIB,
            SectionType::GNULibList => SHType::SHT_GNU_LIBLIST,
            SectionType::DynamicSymbols => SHType::SHT_DYNSYM,
            SectionType::InitArray => SHType::SHT_INIT_ARRAY,
            SectionType::FiniArray => SHType::SHT_FINI_ARRAY,
            SectionType::PreInitArray => SHType::SHT_PREINIT_ARRAY,
            SectionType::Group => SHType::SHT_GROUP,
            SectionType::ExtendedSymbols => SHType::SHT_SYMTAB_SHNDX,
            SectionType::ReservedTypes => SHType::SHT_NUM,
            SectionType::Unknown => SHType::Unknown(0),
        }
    }
}