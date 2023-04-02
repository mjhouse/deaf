use crate::headers::ProgramHeader;
use crate::errors::{Error,Result};

/// A Segment extracted from an ELF file
pub struct Segment {
    header: ProgramHeader
}

impl Segment {

    /// Create a new segment from a program header
    pub fn new(header: ProgramHeader) -> Self {
        Self { header }
    }

    /// Get the body of the segment given a byte buffer
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