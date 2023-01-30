use std::ops::Range;

use crate::errors::{Error,Result};
use crate::constants::*;
use crate::parse;

type Value = usize;
type Values = Range<usize>;

const EI_CLASS: Value = 4;
const EI_DATA: Value = 5;
const EI_VERSION: Value = 6;
const EI_OSABI: Value = 7;
const EI_ABIVERSION: Value = 8;
const EI_PAD: Values = 9..16;
const E_TYPE: Values = 17..19;
const E_MACHINE: Values = 20..22;

fn ei_class(bytes: &[u8]) -> Result<u8> {
    parse::element(bytes,EI_CLASS)
}

fn ei_data(bytes: &[u8]) -> Result<u8> {
    parse::element(bytes,EI_DATA)
}

fn ei_version(bytes: &[u8]) -> Result<u8> {
    parse::element(bytes,EI_VERSION)
}

fn ei_osabi(bytes: &[u8]) -> Result<u8> {
    parse::element(bytes,EI_OSABI)
}

fn ei_abiversion(bytes: &[u8]) -> Result<u8> {
    parse::element(bytes,EI_ABIVERSION)
}

fn ei_pad(bytes: &[u8]) -> Result<&[u8]> {
    parse::range(bytes,EI_PAD)
}

fn e_type(bytes: &[u8]) -> Result<&[u8]> {
    parse::range(bytes,E_TYPE)
}

fn e_machine(bytes: &[u8]) -> Result<&[u8]> {
    parse::range(bytes,E_MACHINE)
}

fn file_class(bytes: &[u8]) -> Result<FileClass> {
    match ei_class(bytes)? {
        1 => Ok(FileClass::ClassNone),
        1 => Ok(FileClass::Class32),
        2 => Ok(FileClass::Class64),
        _ => Err(Error::ParseError),
    }
}

fn file_data(bytes: &[u8]) -> Result<FileData> {
    match ei_data(bytes)? {
        1 => Ok(FileData::DataNone),
        1 => Ok(FileData::DataLSB),
        2 => Ok(FileData::DataMSB),
        _ => Err(Error::ParseError),
    }
}


fn elf_version(bytes: &[u8]) -> Result<ElfVersion> {
    match ei_version(bytes)? {
        1 => Ok(ElfVersion::Current),
        _ => Ok(ElfVersion::Unknown),
    }
}

fn os_abi(bytes: &[u8]) -> Result<OsABI> {
    match ei_osabi(bytes)? {
        0x00 => Ok(OsABI::SystemV),      
        0x01 => Ok(OsABI::HPUX),         
        0x02 => Ok(OsABI::NetBSD),       
        0x03 => Ok(OsABI::Linux),        
        0x04 => Ok(OsABI::GNUHurd),      
        0x06 => Ok(OsABI::Solaris),      
        0x07 => Ok(OsABI::AIXMonterey),  
        0x08 => Ok(OsABI::IRIX),         
        0x09 => Ok(OsABI::FreeBSD),      
        0x0A => Ok(OsABI::Tru64),        
        0x0B => Ok(OsABI::NovellModesto),
        0x0C => Ok(OsABI::OpenBSD),      
        0x0D => Ok(OsABI::OpenVMS),      
        0x0E => Ok(OsABI::NonStopKernel),
        0x0F => Ok(OsABI::AROS),         
        0x10 => Ok(OsABI::FenixOS),      
        0x11 => Ok(OsABI::NuxiCloudABI), 
        0x12 => Ok(OsABI::STOpenVOS),
        _ => Err(Error::ParseError),
    }
}

fn os_abi_version(bytes: &[u8]) -> Result<AbiVersion> {
    match ei_abiversion(bytes)? {
        0 => Ok(AbiVersion::Unknown),
        v => Ok(AbiVersion::Value(v)),
    }
}

fn file_type(bytes: &[u8], data: FileData) -> Result<FileType> {
    match parse::parse_u16(e_type(bytes)?, data)? {
        0x00 => Ok(FileType::Unknown),
        0x01 => Ok(FileType::Relocatable),
        0x02 => Ok(FileType::Executable),
        0x03 => Ok(FileType::SharedFile),
        0x04 => Ok(FileType::CoreFile),
        // 0xFE00..0xFEFF => Ok(FileType::ReservedOperating),
        // 0xFF00..0xFFFF => Ok(FileType::ReservedProcessor),
        _ => Err(Error::ParseError),
    }
}