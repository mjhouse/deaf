// address size of target system
pub enum FileClass {
    ClassNone, // 0
    Class32,   // 1
    Class64,   // 2
}

// endianness of the elf file
pub enum FileData {
    DataNone, // 0
    DataLSB,  // 1 (little-endian)
    DataMSB,  // 2 (big-endian)
}

// should always be `1`for current
pub enum ElfVersion {
    Unknown, // _
    Current, // 1
}

// the ABI used by this elf file
pub enum OsABI {
    SystemV,       // 0x00
    HPUX,          // 0x01
    NetBSD,        // 0x02
    Linux,         // 0x03
    GNUHurd,       // 0x04
    Solaris,       // 0x06
    AIXMonterey,   // 0x07
    IRIX,          // 0x08
    FreeBSD,       // 0x09
    Tru64,         // 0x0A
    NovellModesto, // 0x0B
    OpenBSD,       // 0x0C
    OpenVMS,       // 0x0D
    NonStopKernel, // 0x0E
    AROS,          // 0x0F
    FenixOS,       // 0x10
    NuxiCloudABI,  // 0x11
    STOpenVOS,     // 0x12
}

// only used by glibc >= 2.12,
// specifies version for an ABI
pub enum AbiVersion {
    Unknown,   // 0
    Value(u8), // _
}

// type of ELF file- executable, lib etc.
pub enum FileType {
    Unknown,           // 0x00
    Relocatable,       // 0x01
    Executable,        // 0x02
    SharedFile,        // 0x03
    CoreFile,          // 0x04
    ReservedOperating, // 0xFE00 - 0xFEFF
    ReservedProcessor, // 0xFF00 - 0xFFFF
}

// target instruction set architecture
pub enum Machine {
    None,              // 0x00
    AttWe32100,        // 0x01
    SPARC,             // 0x02
    X86,               // 0x03
    M68k,              // 0x04
    M88k,              // 0x05
    IntelMCU,          // 0x06
    Intel80860,        // 0x07
    MIPS,              // 0x08
    IBMSystem370,      // 0x09
    MIPSRs3k,          // 0x0A
    // Reserved           0x0B - 0x0D
    HPPARISC,          // 0x0E
    // Reserved           0x0F
    Intel80960,        // 0x13
    PowerPC32,         // 0x14
    PowerPC64,         // 0x15
    S390,              // 0x16
    IBMSPUSPC,         // 0x17
    // Reserved           0x18 - 0x23
    NECV800,           // 0x24
    FujitsuFR20,       // 0x25
    TRWRH32,           // 0x26
    MotorolaRCE,       // 0x27
    Arm32,             // 0x28
    DigitalAlpha,      // 0x29
    SuperH,            // 0x2A
    SPARCv9,           // 0x2B
    SiemensTriCore,    // 0x2C
    ArgonautRISCCore,  // 0x2D
    HitachiH8300,      // 0x2E
    HitachiH8300H,     // 0x2F
    HitachiH8S,        // 0x30
    HitachiH8500,      // 0x31
    IA64,              // 0x32
    StanfordMIPSX,     // 0x33
    MotorolaColdFire,  // 0x34
    MotorolaM68HC12,   // 0x35
    FujitsuMMA,        // 0x36
    SiemensPCP,        // 0x37
    SonynCPURISC,      // 0x38
    DensoNDR1,         // 0x39
    MotorolaStarCore,  // 0x3A
    ToyotaME16,        // 0x3B
    STMST100,          // 0x3C
    ALCTinyJ,          // 0x3D
    AMDx8664,          // 0x3E
    SonyDSP,           // 0x3F
    DECPDP10,          // 0x40
    DECPDP11,          // 0x41
    SiemensFX66,       // 0x42
    STMST9Plus816,     // 0x43
    STMST78Bit,        // 0x44
    MotorolaMC68HC16,  // 0x45
    MotorolaMC68HC11,  // 0x46
    MotorolaMC68HC08,  // 0x47
    MotorolaMC68HC05,  // 0x48
    SiliconGSVx,       // 0x49
    STMST198Bit,       // 0x4A
    DigitalVAX,        // 0x4B
    AxisC32Bit,        // 0x4C
    InfineonT32Bit,    // 0x4D
    Element1464BitDSP, // 0x4E
    LSILogic16BitDSP,  // 0x4F
    TMS320C6000,       // 0x8C
    MCSTElbrusE2k,     // 0xAF
    AArch64,           // 0xB7
    ZilogZ80,          // 0xDC
    RISCV,             // 0xF3
    BerkeleyPF,        // 0xF7
    WDC65C816,         // 0x101
    Unknown(u16),      // _
}