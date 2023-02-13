use crate::errors::{Error, Result};
use crate::headers::common::bytes::{AsOutput,AsInput};
use crate::headers::common::constants::{STBind,STType};

pub struct SymbolInfo {
    binding: STBind,
    typing: STType,
}

impl SymbolInfo {

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

}

impl AsInput<u8> for SymbolInfo {
    fn as_input(self) -> Result<u8> { Ok(self.value()) }
}

impl AsOutput<u8> for SymbolInfo {
    fn as_output(a: u8) -> Result<Self> { Self::new(a) }
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
        let result = info.as_input();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(),value);
    }

    #[test]
    fn test_symbol_info_back_to_value() {
        let value = 0x21; // STB_WEAK + STT_OBJECT
        let info = SymbolInfo::new(value).unwrap();
        let result = info.as_input();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(),value);
    }

}