use crate::common::Data;
use crate::headers::ProgramHeader;
use crate::errors::{Error,Result};

/// A Segment extracted from an ELF file
pub struct Segment {
    header: ProgramHeader,
    data: Data
}

impl Segment {

    /// Create a new segment from a program header
    pub fn new(header: ProgramHeader, data: Data) -> Self {
        Self { header, data }
    }

    /// Get the body of the segment given a byte buffer
    pub fn body(&self) -> Result<Vec<u8>> {
        let data = &self.data.lock()?;

        let size = self
            .header
            .body_size();

        let offset = self
            .header
            .offset();

        let start = offset;
        let end = start + size;

        if end < data.len() {
            Ok(data[start..end].into())
        } else {
            Err(Error::OutOfBoundsError)
        }
    }

}