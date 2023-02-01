// the addressing width of the ELF file
#[derive(Debug, Clone)]
pub enum Width {
    Any,
    X32,
    X64,
}

// the endianness of the ELF file
#[derive(Debug, Clone)]
pub enum Layout {
    Any,
    Little,
    Big,
}
