use num_enum::{IntoPrimitive, TryFromPrimitive, FromPrimitive};
use enumflags2::bitflags;

/// The mode of the ELF file
///
/// This enum is generally parsed from the file header (ei_class) and then 
/// passed to all of the other sections for parsing addresses etc.
#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Default, Debug, Clone, Copy, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum Width {
    /// Mode of the ELF file is 32-bit
    #[default]
    X32 = 0x01,
    /// Mode of the ELF file is 64-bit
    X64 = 0x02 
}

/// The endianness of the ELF file
///
/// This enum is generally parsed from the file header (ei_data) and then 
/// passed to all of the other sections for parsing integers etc.
#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Default, Debug, Clone, Copy, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum Layout {
    /// Little endian (e.g. 0xABCD is represented as 'CD AB')
    #[default]
    Little = 0x01,
    /// Big endian (e.g. 0xABCD is represented as 'AB CD' )
    Big = 0x02
}

/// The type of a program header
///
/// This enum is generally parsed from the program headers (p_type).
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive( Debug, Clone, Copy, PartialEq, IntoPrimitive, TryFromPrimitive)]
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

impl Default for PHType {
    fn default() -> Self { Self::PT_NULL }
}

/// Reserved values for section header indices
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, IntoPrimitive, FromPrimitive)]
pub enum SHIndex {
    #[default]
    /// An undefined, missing, irrelevant, or otherwise meaningless section reference
    SHN_UNDEF = 0x0000,

    /// Provide for ordering in conjunction with the SHF_LINK_ORDER/SHF_ORDERED flags
    SHN_BEFORE = 0xff00,

    /// Provide for ordering in conjunction with the SHF_LINK_ORDER/SHF_ORDERED flags
    SHN_AFTER = 0xff01,

    /// Symbols defined relative to this index are not affected by relocation
    SHN_ABS = 0xfff1,

    /// Sections defined relative to this index are tenative/common symbols
    SHN_COMMON = 0xfff2,

    /// The actual section header index is too large to fit in the containing field 
    /// (actual index is in SHT_SYMTAB_SHNDX section)
    SHN_XINDEX = 0xffff,
}

/// The type of a section header
///
/// This enum is generally parsed from the section headers (sh_type). Most of these section types can be found 
/// in [this](https://docs.oracle.com/cd/E19683-01/817-3677/chapter6-94076/index.html) Oracle documentation, 
/// with the exception of SHT_GNU_HASH and SHT_GNU_LIBLIST which are undocumented. Information regarding these 
/// two section types was found in the freebsd documentation and various blogs, and (an abbreviated explanation) 
/// is included below for reference.
/// 
/// ## SHT_GNU_HASH
/// This does the same job at SHT_HASH, but adds a bloom filter to stop lookup for nonexistent symbols earlier. This provides
/// some performance gains on GNU systems that use this lookup table over the standard one. The structure of the section is 
/// something like the following struct[^1].
/// 
/// ```
/// struct gnu_hash_table<'a> {
///     nbuckets: u32,
///     symoffset: u32,
///     bloom_size: u32,
///     bloom_shift: u32,
///     bloom: &'a [u64], // [bloom_size]; /* u32 for 32-bit binaries */
///     buckets: &'a [u32], //[nbuckets];
///     chain: &'a [u32], //[];
/// };
/// 
/// ```
/// 
/// ## SHT_GNU_LIBLIST
/// According to the freebsd documentation[^2], the SHT_GNU_LIBLIST is a section that contains a series of structs of type
/// Elf32_Lib or Elf64_Lib with the following shape. These entries define external libraries that should be pre-linked
/// when the current binary runs.
/// 
/// ```
/// struct Elf32_Lib {
///     l_name: u32,       /* Name (string table index) */
///     l_time_stamp: u32, /* Timestamp */
///     l_checksum: u32,   /* Checksum */
///     l_version: u32,    /* Interface version */
///     l_flags: u32,      /* Flags */
/// }
/// 
/// struct Elf64_Lib {
///     l_name: u64,
///     l_time_stamp: u64,
///     l_checksum: u64,
///     l_version: u64,
///     l_flags: u64,
/// }
/// ```
/// 
/// [^1]: <https://flapenguin.me/elf-dt-gnu-hash>
/// [^2]: <https://man.freebsd.org/cgi/man.cgi?elf(3)>
#[repr(u32)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, IntoPrimitive, FromPrimitive)]
pub enum SHType {
    /// Section is inactive
    SHT_NULL = 0x00000000,
    /// Information defined by the program
    SHT_PROGBITS = 0x00000001,
    /// Symbol table section
    SHT_SYMTAB = 0x00000002,
    /// String table section
    SHT_STRTAB = 0x00000003,
    /// Relocation table with explicit addends
    SHT_RELA = 0x00000004,
    /// Symbol hash table section
    SHT_HASH = 0x00000005,
    /// Dynamic linking information
    SHT_DYNAMIC = 0x00000006,
    /// General notes about the object file
    SHT_NOTE = 0x00000007,
    /// Section with no content
    SHT_NOBITS = 0x00000008,
    /// Relocation table index
    SHT_REL = 0x00000009,
    /// Reserved section with unspecified semantics
    SHT_SHLIB = 0x0000000A,
    /// Dynamic symbol table section 
    SHT_DYNSYM = 0x0000000B,
    /// Array of pointers to initialization functions
    SHT_INIT_ARRAY = 0x0000000E,
    /// Array of pointers to termination functions
    SHT_FINI_ARRAY = 0x0000000F,
    /// Array of pointers to preinit functions
    SHT_PREINIT_ARRAY = 0x00000010,
    /// Identifies an interrelated group of sections
    SHT_GROUP = 0x00000011,
    /// Extended symbol table index
    SHT_SYMTAB_SHNDX = 0x00000012,
    /// Marker value for section type iteration
    SHT_NUM = 0x00000013,
    /// Low bound for reserved OS-specific semantics
    SHT_LOOS = 0x60000000,
    /// High bound for reserved OS-specific semantics
    SHT_HIOS = 0x6fffffff,
    /// GNU-specific hash table
    SHT_GNU_HASH = 0x6ffffff6,
    /// GNU-specific list of libraries to be pre-linked
    SHT_GNU_LIBLIST = 0x6ffffff7,
    /// ARM unwind info
    SHT_ARM_EXIDX =	0x70000001,
    /// Contains a pre-emption map that allows symbols to be overriden
    SHT_ARM_PREEMPTMAP = 0x70000002,
    /// Contains build attributes for ARM systems
    SHT_ARM_ATTRIBUTES = 0x70000003,
    /// Overlay debug info
    SHT_ARM_DEBUGOVERLAY = 0x70000004,
    /// GDB and overlay integration info
    SHT_ARM_OVERLAYSECTION = 0x70000005,
    /// Unknown catch-all type 
    #[num_enum(catch_all)]
    Unknown(u32),

    // SHT_VERDEF = 0x6ffffffd,
    // SHT_VERNEED = 0x6ffffffe,
    // SHT_VERSYM = 0x6fffffff,
}

impl Default for SHType {
    fn default() -> Self { Self::SHT_NULL }
}

/// The flags of a particular section header
///
/// This enum is generally parsed from the section headers (sh_flags).
#[bitflags]
#[repr(u64)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SHFlags {
    /// Data is writable during process execution
    SHF_WRITE = 0x00000001,
    /// Occupies memory during process execution
    SHF_ALLOC = 0x00000002,
    /// Contains executable machine instructions
    SHF_EXECINSTR = 0x00000004,
    /// Section has data that may be merged to eliminate duplication
    SHF_MERGE = 0x00000010,
    /// Section that consists of null-terminated character strings
    SHF_STRINGS = 0x00000020,
    /// The section header holds a section header table index
    SHF_INFO_LINK = 0x00000040,
    /// Add special ordering requirements to the link-editor
    SHF_LINK_ORDER = 0x00000080,
    /// Section requires special OS-specific processing
    SHF_OS_NONCONFORMING = 0x00000100,
    /// Section is a member, perhaps the only one, of a section group
    SHF_GROUP = 0x00000200,
    /// Section holds thread-local storage
    SHF_TLS = 0x00000400,
    /// Section requires ordering in relation to other sections of the same type
    SHF_ORDERED = 0x40000000,
    /// Section is excluded from input to the link-edit of an executable or shared object
    SHF_EXCLUDE = 0x80000000,
    // SHF_MASKPROC = 0xf0000000, // Mask of bits reserved for processor-specific semantics. 
    // SHF_MASKOS = 0x0ff00000, // Mask of bits reserved for OS-specific semantics.
}

/// The binding of a symbol entry from a static or dynamic symbol table
///
/// This enum is parsed from the upper 4 bits of the 1-byte 'st_info' 
/// field include in each entry of the symbol table.
#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Default, Debug, Clone, Copy, PartialEq, IntoPrimitive, FromPrimitive)]
pub enum STBind {
    /// Symbol is not visible outside the object file containing its definition
    #[default]
    STB_LOCAL = 0x00,
    /// Symbols is visible to all object files being combined
    STB_GLOBAL = 0x01,
    /// Same as STB_GLOBAL, but lower precedence
    STB_WEAK = 0x02,
}

/// The type of a symbol entry from a static or dynamic symbol table
///
/// This enum is parsed from the lower 4 bits of the 1-byte 'st_info' 
/// field include in each entry of the symbol table.
#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Default, Debug, Clone, Copy, PartialEq, IntoPrimitive, FromPrimitive)]
pub enum STType {
    /// The symbol type is not specified
    #[default]
    STT_NOTYPE = 0x00,
    /// Symbol is associated with a data object, such as a variable, an array, etc.
    STT_OBJECT = 0x01,
    /// Symbol is associated with a function or other executable code
    STT_FUNC = 0x02,
    /// Symbol is associated with a section
    STT_SECTION = 0x03,
    /// Symbol gives the name of an associated source file
    STT_FILE = 0x04,
    /// An uninitialized common block, treated the same as [STT_OBJECT](STType::STT_OBJECT)
    STT_COMMON = 0x05,
    /// Symbol is a thread local storage template (value is offset in [SHF_TLS](SHFlags::SHF_TLS) section)
    STT_TLS = 0x06,
}

/// The binding of a symbol entry from a static or dynamic symbol table
///
/// This enum is parsed entries in the symbol table.
#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Default, Debug, Clone, Copy, PartialEq, IntoPrimitive, FromPrimitive)]
pub enum STVisibility {
    #[default]
    /// The visibility is as specified by the symbol binding type
    STV_DEFAULT   = 0x00,

    /// This visibility attribute is currently reserved
    STV_INTERNAL  = 0x01,

    /// This symbol is protected and not externally visible
    STV_HIDDEN    = 0x02,

    /// External references must be resolved externally
    STV_PROTECTED = 0x03
}