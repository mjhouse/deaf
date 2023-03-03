use crate::errors::{Error, Result};
use crate::common::bytes::Convert;
// use crate::headers::common::constants::{STBind,STType};

#[derive(Clone,Copy)]
pub struct RelocationInfo {
    symbol: u64,
    kind: u8,
}

impl RelocationInfo {

    pub fn empty() -> Self {
        Self { 
            symbol: 0,
            kind: 0
        }
    }

    pub fn new(v: u64) -> Result<Self> {
        Ok(Self {
            symbol: v >> 8,
            kind: v as u8,
        })
    }

    pub fn value(&self) -> u64 {
        (self.symbol << 8) + (self.kind as u64)
    }

    pub fn symbol(&self) -> u64 {
        self.symbol
    }

    pub fn kind(&self) -> u8 {
        self.kind
    }

}

impl Convert<u64> for RelocationInfo {
    fn convert(self) -> Result<u64> { Ok(self.value()) }
}

impl Convert<u32> for RelocationInfo {
    fn convert(self) -> Result<u32> { Ok(self.value().try_into()?) }
}

impl Convert<RelocationInfo> for u64 {
    fn convert(self) -> Result<RelocationInfo> { RelocationInfo::new(self) }
}

impl Convert<RelocationInfo> for u32 {
    fn convert(self) -> Result<RelocationInfo> { RelocationInfo::new(self.into()) }
}

impl std::fmt::Debug for RelocationInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RelocationInfo")
         .field("symbol", &self.symbol())
         .field("kind", &self.kind())
         .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relocation_info_parse_pair() {
        // original value (0xfe000000 + 0x06)
        let value = 0xfe00000006;

        // parse the relocation info from value
        let result = RelocationInfo::new(value);

        // unwrap the relocation result
        assert!(result.is_ok());
        let info = result.unwrap();

        // verify that fields have expected value
        assert_eq!(info.symbol,0xfe000000);
        assert_eq!(info.kind,0x06);
    }

    #[test]
    fn test_relocation_info_parse_zeroes() {
        // original value
        let value = 0x0000000000;

        // parse the relocation info from value
        let result = RelocationInfo::new(value);

        // unwrap the relocation result
        assert!(result.is_ok());
        let info = result.unwrap();

        // verify that both fields are zero
        assert_eq!(info.symbol,0x00);
        assert_eq!(info.kind,0x00);
    }

    #[test]
    fn test_relocation_info_back_to_zeroes() {
        // original value
        let value = 0x0000000000;

        // parse the relocation info and then convert back
        let info = RelocationInfo::new(value).unwrap();
        let result: Result<u64> = info.convert();

        // verify that the result matches the original
        assert!(result.is_ok());
        assert_eq!(result.unwrap(),value);
    }

    #[test]
    fn test_relocation_info_back_to_value() {
        // original value
        let value = 0xfe00000006;

        // parse the relocation info and then convert back
        let info = RelocationInfo::new(value).unwrap();
        let result: Result<u64> = info.convert();

        // verify that the result matches the original
        assert!(result.is_ok());
        assert_eq!(result.unwrap(),value);
    }

}