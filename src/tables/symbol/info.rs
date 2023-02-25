use crate::errors::{Error, Result};
use crate::headers::common::bytes::Transmute;
use crate::headers::common::constants::{STBind,STType};

#[derive(Debug,Clone,Copy)]
pub struct SymbolInfo {
    binding: STBind,
    typing: STType,
}

impl SymbolInfo {

    pub fn empty() -> Self {
        Self { 
            binding: STBind::STB_LOCAL,
            typing: STType::STT_NOTYPE
        }
    }

    pub fn new(v: u8) -> Result<Self> {
        let binding = STBind::try_from(v >> 4)?;
        let typing = STType::try_from(v & 0xf)?;
        Ok(Self { binding, typing })
    }

    pub fn value(&self) -> u8 {
        let b: u8 = self.binding.into();
        let t: u8 = self.typing.into();
        (b << 4) | t
    }

    pub fn typing(&self) -> STType {
        self.typing.clone()
    }

    pub fn binding(&self) -> STBind {
        self.binding.clone()
    }

}

impl Transmute<u8> for SymbolInfo {
    fn transmute(self) -> Result<u8> { Ok(self.value()) }
}

impl Transmute<SymbolInfo> for u8 {
    fn transmute(self) -> Result<SymbolInfo> { SymbolInfo::new(self) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_info_parse_pair() {
        let value = 0x21; // STB_WEAK + STT_OBJECT
        let result = SymbolInfo::new(value);

        assert!(result.is_ok());
        let info = result.unwrap();

        assert_eq!(info.binding,STBind::STB_WEAK);
        assert_eq!(info.typing,STType::STT_OBJECT);
    }

    #[test]
    fn test_symbol_info_parse_zeroes() {
        let value = 0x00; // STB_LOCAL + STT_NOTYPE
        let result = SymbolInfo::new(value);

        assert!(result.is_ok());
        let info = result.unwrap();

        assert_eq!(info.binding,STBind::STB_LOCAL);
        assert_eq!(info.typing,STType::STT_NOTYPE);
    }

    #[test]
    fn test_symbol_info_back_to_zeroes() {
        let value = 0x00; // STB_LOCAL + STT_NOTYPE
        let info = SymbolInfo::new(value).unwrap();
        let result: Result<u8> = info.transmute();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(),value);
    }

    #[test]
    fn test_symbol_info_back_to_value() {
        let value = 0x21; // STB_WEAK + STT_OBJECT
        let info = SymbolInfo::new(value).unwrap();
        let result: Result<u8> = info.transmute();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(),value);
    }

}