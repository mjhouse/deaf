use crate::errors::{Error,Result};
use crate::constants::FileData;
use std::ops::Range;

pub fn range(bytes: &[u8], range: Range<usize>) -> Result<&[u8]> {
    if bytes.len() > range.end {
        Ok(&bytes[range])
    } else {
        Err(Error::ParseError)
    }
}

pub fn element(bytes: &[u8], index: usize) -> Result<u8> {
    if bytes.len() > index {
        Ok(bytes[index])
    } else {
        Err(Error::ParseError)
    }
}

pub fn le_u16(bytes: &[u8]) -> Result<u16> {
    Ok(u16::from_le_bytes(bytes.try_into()?))
}

pub fn be_u16(bytes: &[u8]) -> Result<u16> {
    Ok(u16::from_be_bytes(bytes.try_into()?))
}

pub fn parse_u16(bytes: &[u8], data: FileData) -> Result<u16> {
    match data {
        FileData::DataLSB => le_u16(bytes),
        FileData::DataMSB => be_u16(bytes),
        _ => Err(Error::ParseError)
    }
}