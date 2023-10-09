use crate::errors::{Error,Result};
use crate::common::{Convert, STBind, STType};

const TYPE_MASK:  u8 = 0x0f;
const TYPE_SHIFT: u8 = 0;

const BIND_MASK:  u8 = 0xf0;
const BIND_SHIFT: u8 = 4;

/// Representation of the info field in a Symbol record
#[derive(Default,Debug,Clone,Copy,PartialEq)]
pub struct SymbolInfo {
    bind: STBind,
    kind: STType,
}

impl SymbolInfo {

    /// Create a new SymbolInfo struct with default values
    pub fn new() -> Self {
        Self {
            bind: STBind::default(),
            kind: STType::default()
        }
    }

    /// Get the 'kind' component of the info struct
    pub fn kind(&self) -> STType {
        self.kind.clone()
    }

    /// Set the 'kind' component of the info struct
    pub fn set_kind(&mut self, kind: STType) {
        self.kind = kind;
    }

    /// Builder method for setting the 'kind' component of the info struct
    pub fn with_kind(mut self, kind: STType) -> Self {
        self.set_kind(kind);
        self
    }

    /// Get the 'bind' component of the info struct
    pub fn bind(&self) -> STBind {
        self.bind.clone()
    }

    /// Set the 'bind' component of the info struct
    pub fn set_bind(&mut self, bind: STBind) {
        self.bind = bind;
    }

    /// Builder method for setting the 'bind' component of the info struct
    pub fn with_bind(mut self, bind: STBind) -> Self {
        self.set_bind(bind);
        self
    }

}

impl TryFrom<u8> for SymbolInfo {
    type Error = Error;

    fn try_from(v: u8) -> Result<Self> {
        let bind = STBind::try_from((v & BIND_MASK) >> BIND_SHIFT)?;
        let kind = STType::try_from((v & TYPE_MASK) >> TYPE_SHIFT)?;
        Ok(Self { bind, kind })
    }
} 

impl From<SymbolInfo> for u8 {

    fn from(v: SymbolInfo) -> Self {
        let b: u8 = v.bind().into();
        let t: u8 = v.kind().into();
        (b << BIND_SHIFT) | (t << TYPE_SHIFT)
    }

}

impl Convert<u8> for SymbolInfo {
    fn convert(self) -> Result<u8> { Ok(self.into()) }
}

impl Convert<SymbolInfo> for u8 {
    fn convert(self) -> Result<SymbolInfo> { self.try_into() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_info_parse_pair() {
        let value = 0x21; // STB_WEAK + STT_OBJECT
        let result = SymbolInfo::try_from(value);

        assert!(result.is_ok());
        let info = result.unwrap();

        assert_eq!(info.bind,STBind::STB_WEAK);
        assert_eq!(info.kind,STType::STT_OBJECT);
    }

    #[test]
    fn test_symbol_info_parse_zeroes() {
        let value = 0x00; // STB_LOCAL + STT_NOTYPE
        let result = SymbolInfo::try_from(value);

        assert!(result.is_ok());
        let info = result.unwrap();

        assert_eq!(info.bind,STBind::STB_LOCAL);
        assert_eq!(info.kind,STType::STT_NOTYPE);
    }

    #[test]
    fn test_symbol_info_back_to_zeroes() {
        let value = 0x00; // STB_LOCAL + STT_NOTYPE
        let info = SymbolInfo::try_from(value).unwrap();
        let result: Result<u8> = info.convert();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(),value);
    }

    #[test]
    fn test_symbol_info_back_to_value() {
        let value = 0x21; // STB_LOCAL + STT_OBJECT
        let info = SymbolInfo::try_from(value).unwrap();
        let result: Result<u8> = info.convert();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(),value);
    }

}