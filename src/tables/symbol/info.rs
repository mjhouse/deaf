use crate::errors::{Error, Result};
use crate::headers::common::bytes::Transmute;
use crate::headers::common::constants::{STBind,STType};

#[derive(Clone,Copy)]
pub struct SymbolInfo {
    bind: STBind,
    kind: STType,
}

impl SymbolInfo {

    pub fn empty() -> Self {
        Self { 
            bind: STBind::STB_LOCAL,
            kind: STType::STT_NOTYPE
        }
    }

    pub fn new(v: u8) -> Result<Self> {
        let bind = STBind::try_from(v >> 4)?;
        let kind = STType::try_from(v & 0xf)?;
        Ok(Self { bind, kind })
    }

    pub fn value(&self) -> u8 {
        let b: u8 = self.bind.into();
        let t: u8 = self.kind.into();
        (b << 4) | t
    }

    pub fn kind(&self) -> STType {
        self.kind.clone()
    }

    pub fn bind(&self) -> STBind {
        self.bind.clone()
    }

}

impl Transmute<u8> for SymbolInfo {
    fn transmute(self) -> Result<u8> { Ok(self.value()) }
}

impl Transmute<SymbolInfo> for u8 {
    fn transmute(self) -> Result<SymbolInfo> { SymbolInfo::new(self) }
}

impl std::fmt::Debug for SymbolInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SymbolInfo")
         .field("kind", &self.kind())
         .field("bind", &self.bind())
         .finish()
    }
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

        assert_eq!(info.bind,STBind::STB_WEAK);
        assert_eq!(info.kind,STType::STT_OBJECT);
    }

    #[test]
    fn test_symbol_info_parse_zeroes() {
        let value = 0x00; // STB_LOCAL + STT_NOTYPE
        let result = SymbolInfo::new(value);

        assert!(result.is_ok());
        let info = result.unwrap();

        assert_eq!(info.bind,STBind::STB_LOCAL);
        assert_eq!(info.kind,STType::STT_NOTYPE);
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