use crate::headers::SectionHeader;
use crate::errors::{Error,Result};

pub struct Section {
    header: SectionHeader
}

impl Section {

    /// Create a new section 
    pub fn new(header: SectionHeader) -> Self {
        Self { header }
    }

    /// Get the body of the section given a byte buffer
    pub fn body<'a>(&self, bytes: &'a [u8]) -> Result<&'a [u8]> {
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

        if end < bytes.len() {
            Ok(&bytes[start..end])
        } else {
            Err(Error::OutOfBoundsError)
        }
    }

}