use crate::common::SHType;

/// The type of a section (string table, relocation table etc.)
/// 
/// Refer to the [SHType] enum for more information about specific values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SectionType {
    /// Section is inactive (SHT_NULL)
    Null,
    /// Information defined by the program (SHT_PROGBITS)
    Program,
    /// Symbol table section (SHT_SYMTAB)
    Symbols,
    /// Dynamic symbol table section (SHT_DYNSYM)
    DynamicSymbols,
    /// Extended symbol table index (SHT_SYMTAB_SHNDX)
    ExtendedSymbols,
    /// String table section (SHT_STRTAB)
    Strings,
    /// Relocation table index (SHT_REL)
    Relocations,
    /// Relocation table with explicit addends (SHT_RELA)
    RelocationsAddend,
    /// Symbol hash table section (SHT_HASH)
    Hash,
    /// GNU-specific hash table (SHT_GNU_HASH)
    GNUHash,
    /// Dynamic linking information (SHT_DYNAMIC)
    Dynamic,
    /// General notes about the object file (SHT_NOTE)
    Notes,
    /// Section with no content (SHT_NOBITS)
    Empty,
    /// Reserved section with unspecified semantics (SHT_SHLIB)
    Reserved,
    /// GNU-specific list of libraries to be pre-linked (SHT_GNU_LIBLIST)
    GNULibList,
    /// Array of pointers to initialization functions (SHT_INIT_ARRAY)
    InitArray,
    /// Array of pointers to termination functions (SHT_FINI_ARRAY)
    FiniArray,
    /// Array of pointers to preinit functions (SHT_PREINIT_ARRAY)
    PreInitArray,
    /// Identifies an interrelated group of sections (SHT_GROUP)
    Group,
    /// Unknown section type (SHT_NUM, SHT_LOOS, etc.)
    Unknown,
    /// Any section type
    Any,
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
            SectionType::Unknown => SHType::Unknown(0),
            SectionType::Any => SHType::Unknown(0),
        }
    }
}