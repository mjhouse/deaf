use crate::errors::{Error, Result};
use crate::headers::common::bytes::Convert;
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