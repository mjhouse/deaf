use crate::headers::SectionHeader;
use crate::errors::{Result};
use crate::common::{Layout,Width,SectionType};

/// A Section extracted from an ELF file
#[derive(Debug)]
pub struct Section {
    header: SectionHeader,
    data: Vec<u8>,
}

impl Section {

    /// Create a new section from a program header
    pub fn new(header: SectionHeader) -> Self {
        Self { 
            header: header, 
            data: Vec::new()
        }
    }

    pub fn read(header: SectionHeader, data: &[u8]) -> Result<Self> {
        let offset = header.offset();
        let size   = header.body_size();
        let start  = offset;
        let end    = start + size;

        let mut section = Section::new(header);
        section.data = data[start..end].into();
        Ok(section)
    }

    pub fn write(&self, data: &mut [u8], offset: usize, index: usize) -> Result<usize> {
        self.header.write(&mut data[offset..])?;

        let offset = self.header.offset();
        let size   = self.header.body_size();
        let start  = offset + index * size;
        let end    = start + size;
        
        data[start..end].copy_from_slice(&self.data);
        Ok(self.size())
    }

    /// Parse all sections for a byte array given count, offset etc.
    pub fn read_all(data: &[u8], count: usize, offset: usize, size: usize, layout: Layout, width: Width) -> Result<Vec<Section>> {
        (0..count)
            .into_iter()
            .map(|i| offset + i * size)
            .map(|i| SectionHeader::parse(
                &data[i..],
                layout,
                width))
            .map(|r| r
                .and_then(|h| Section::read(
                    h,
                    data
                )))
            .collect()
    }

    pub fn header(&self) -> &SectionHeader {
        &self.header
    }

    pub fn header_mut(&mut self) -> &mut SectionHeader {
        &mut self.header
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    pub fn size(&self) -> usize {
        self.header.size() +
        self.data.len()
    }

    pub fn name(&self) -> usize {
        self.header.name() as usize
    }

    pub fn body_size(&self) -> usize {
        self.header.body_size()
    }

    pub fn entity_size(&self) -> usize {
        self.header.entsize()
    }

    pub fn offset(&self) -> usize {
        self.header.offset()
    }

    pub fn layout(&self) -> Layout {
        self.header.layout()
    }

    pub fn width(&self) -> Width {
        self.header.width()
    }

    pub fn kind(&self) -> SectionType {
        self.header.kind().into()
    }

    pub fn is_kind(&self, kind: SectionType) -> bool {
        kind == self.kind()
    }

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