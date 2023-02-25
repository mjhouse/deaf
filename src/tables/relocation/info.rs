use crate::errors::{Error, Result};
use crate::headers::common::bytes::Transmute;
// use crate::headers::common::constants::{STBind,STType};

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

impl Transmute<u64> for RelocationInfo {
    fn transmute(self) -> Result<u64> { Ok(self.value()) }
}

impl Transmute<u32> for RelocationInfo {
    fn transmute(self) -> Result<u32> { Ok(self.value().try_into()?) }
}

impl Transmute<RelocationInfo> for u64 {
    fn transmute(self) -> Result<RelocationInfo> { RelocationInfo::new(self) }
}

impl Transmute<RelocationInfo> for u32 {
    fn transmute(self) -> Result<RelocationInfo> { RelocationInfo::new(self.into()) }
}

impl std::fmt::Debug for RelocationInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RelocationInfo")
         .field("symbol", &self.symbol())
         .field("kind", &self.kind())
         .finish()
    }
}