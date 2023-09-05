use crate::errors::{Result, Error};
use crate::common::Convert;

/// Representation of the info field in a Relocation record
#[derive(Default,Debug,Clone,Copy,PartialEq)]
pub struct RelocationInfo {
    symbol: u64,
    kind: u8,
}

impl RelocationInfo {

    /// Parse a combined value as an info struct
    fn new(symbol: u64, kind: u8) -> Self {
        Self { symbol, kind }
    }

    /// Get the 'symbol' component of the info struct
    pub fn symbol(&self) -> u64 {
        self.symbol
    }

    /// Set the 'symbol' component of the info struct
    pub fn set_symbol(&mut self, value: u64) {
        self.symbol = value;
    }

    /// Get the 'kind' component of the info struct
    pub fn kind(&self) -> u8 {
        self.kind
    }

    /// Set the 'kind' component of the info struct
    pub fn set_kind(&mut self, value: u8) {
        self.kind = value;
    }

}

impl Convert<u64> for RelocationInfo {
    fn convert(self) -> Result<u64> { 
        Ok((self.symbol as u64) << 32 | self.kind as u64) 
    }
}

impl Convert<u32> for RelocationInfo {
    fn convert(self) -> Result<u32> { 
        Ok((self.symbol as u32) << 8 | self.kind as u32) 
    }
}

impl Convert<RelocationInfo> for u64 {
    fn convert(self) -> Result<RelocationInfo> {
        Ok(RelocationInfo::new(
            self >> 32, 
            self as u8)
        )
    }
}

impl Convert<RelocationInfo> for u32 {
    fn convert(self) -> Result<RelocationInfo> {
        Ok(RelocationInfo::new(
            (self as u64) >> 8, 
            self as u8)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relocation_info_parse_pair_32() {
        // original value (0xfe000000 + 0x06)
        let value: u32 = 0xfe000006;

        // parse the relocation info from value
        let result: Result<RelocationInfo> = value.convert();

        // unwrap the relocation result
        assert!(result.is_ok());
        let info = result.unwrap();

        // verify that fields have expected value
        assert_eq!(info.symbol,0xfe0000);
        assert_eq!(info.kind,0x06);
    }

    #[test]
    fn test_relocation_info_parse_pair_64() {
        // original value (0xfe000000 + 0x06)
        let value: u64 = 0xfe00000006;

        // parse the relocation info from value
        let result: Result<RelocationInfo> = value.convert();

        // unwrap the relocation result
        assert!(result.is_ok());
        let info = result.unwrap();

        // verify that fields have expected value
        assert_eq!(info.symbol,0xfe);
        assert_eq!(info.kind,0x06);
    }

    #[test]
    fn test_relocation_info_parse_zeroes() {
        // original value
        let value: u64 = 0x0000000000;

        // parse the relocation info from value
        let result: Result<RelocationInfo> = value.convert();

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
        let value: u64 = 0x0000000000;

        // parse the relocation info and then convert back
        let info: RelocationInfo = value.convert().unwrap();
        let result: Result<u64> = info.convert();

        // verify that the result matches the original
        assert!(result.is_ok());
        assert_eq!(result.unwrap(),value);
    }

    #[test]
    fn test_relocation_info_back_to_value_32() {
        // original value
        let value: u32 = 0xfe000006;

        // parse the relocation info and then convert back
        let info: RelocationInfo = value.convert().unwrap();
        let result: Result<u32> = info.convert();

        // verify that the result matches the original
        assert!(result.is_ok());
        assert_eq!(result.unwrap(),value);
    }

    #[test]
    fn test_relocation_info_back_to_value_64() {
        // original value
        let value: u64 = 0xfe00000006;

        // parse the relocation info and then convert back
        let info: RelocationInfo = value.convert().unwrap();
        let result: Result<u64> = info.convert();

        // verify that the result matches the original
        assert!(result.is_ok());
        assert_eq!(result.unwrap(),value);
    }

}