use crate::common::Data;
use crate::binary::Binary;
use crate::headers::SectionHeader;
use crate::errors::{Error,Result};
use std::ops::DerefMut;

/// A Section extracted from an ELF file
pub struct Section {
    header: SectionHeader,
    data: Data
}

impl Section {

    /// Create a new segment from a program header
    pub fn new(header: SectionHeader, data: Data) -> Self {
        Self { header, data }
    }

    pub fn header(&self) -> &SectionHeader {
        &self.header
    }

    pub fn header_mut(&mut self) -> &mut SectionHeader {
        &mut self.header
    }

    pub fn name(&self, binary: &Binary) -> String {
        // parse string buffers

        // get self 

        unimplemented!();
    }

    /// Get the body of the section
    pub fn body(&self) -> Result<Vec<u8>> {
        let data = &self.data.lock()?;

        let size = self
            .header
            .body_size()
            .ok_or(Error::MalformedDataError)?;

        let offset = self
            .header
            .offset()
            .ok_or(Error::MalformedDataError)?;

        let start = offset;
        let end = start + size;

        if end < data.len() {
            Ok(data[start..end].into())
        } else {
            Err(Error::OutOfBoundsError)
        }
    }

    fn test_mut(&mut self) -> Result<()> {
        let test: &mut Vec<u8> = self.data.lock()?.deref_mut();
        Ok(())
    }

}