use crate::common::Data;
use crate::headers::SectionHeader;
use crate::errors::{Error,Result};

/// A Section extracted from an ELF file
#[derive(Debug)]
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

    // pub fn name(&self, binary: &Binary) -> Option<String> {
    //     self.header
    //         .name()
    //         .and_then(|i| binary
    //             .section_name(i as usize))
    // }

    /// Get the body of the section
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

    // fn test_mut(&mut self) -> Result<()> {
    //     let test: &mut Vec<u8> = self.data.lock()?.deref_mut();
    //     Ok(())
    // }

}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::headers::FileHeader;

    // use crate::utilities::read;

    // #[test]
    // fn test_read_section_headers() {
    //     let binary = Binary::new("assets/libvpf/libvpf.so.4.1").unwrap();
    //     let section = binary.section_by_name(".text".into());

    //     dbg!(section);
    // }
}