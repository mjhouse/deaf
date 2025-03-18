use crate::common::{ByteDelimiter,Layout,IntoBytes,FromBytes};
use crate::tables::TableItem;
use crate::errors::{Error,Result};
use std::ffi::CString;

/// A string item found in string tables
#[derive(Default,Debug,Clone,PartialEq)]
pub struct StringItem {
    value: CString,
}

impl StringItem {

    /// Get the string value of the table item
    ///
    /// This method will fail if the string is not 
    /// valid UTF-8
    pub fn string(&self) -> Result<String> {
        Ok(self.value.clone().into_string()?.into())
    }

    /// Get the string value of the table item
    ///
    /// This method will replace invalid characters
    /// with U+FFFD (REPLACEMENT CHARACTER)
    pub fn string_lossy(&self) -> String {
        self.value.to_string_lossy().into()
    }

    /// Set the string value of the table item
    ///
    /// This method will fail if the string is not
    /// valid UTF-8
    pub fn set_string(&mut self, value: String) -> Result<()> {
        self.value = CString::new(value.as_bytes())?;
        Ok(())
    }

}

impl TableItem for StringItem {

    // Override the default implementation
    fn delimiter(_: usize) -> ByteDelimiter {
        ByteDelimiter::Value(b'\0')
    }

    fn read(&mut self, b: &[u8]) -> Result<()> {
        self.value = CString::from_bytes(b,Layout::Little)?;
        Ok(())
    }

    fn write(&self, b: &mut [u8]) -> Result<()> {
        self.value.to_bytes(b, Layout::Little)
    }

    fn size(&self) -> usize {
        self.value.as_bytes_with_nul().len()
    }

}

impl TryFrom<String> for StringItem {
    type Error = Error;
    fn try_from(v: String) -> Result<Self> {
        let mut item = StringItem::default();
        item.set_string(v)?;
        Ok(item)
    }
}

impl TryFrom<&str> for StringItem {
    type Error = Error;
    fn try_from(v: &str) -> Result<Self> {
        String::from(v).try_into()
    }
}