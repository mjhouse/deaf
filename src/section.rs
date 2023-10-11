use crate::headers::SectionHeader;
use crate::errors::{Result,Error};
use crate::common::{Layout,Width,SectionType,Update,Updateable};

/// A Section extracted from an ELF file
#[derive(Debug,Clone)]
pub struct Section {
    name: String,
    header: SectionHeader,
    data: Vec<u8>,
}

impl Section {

    /// Create a new section from a program header
    pub fn new(header: SectionHeader) -> Self {
        Self { 
            name: String::new(),
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
        
        section.data = data
            .get(start..end)
            .unwrap_or_default()
            .into();
        
        Ok(section)
    }

    pub fn write(&self, data: &mut [u8], offset: usize, index: usize) -> Result<usize> {
        self.header.write(&mut data[offset..])?;

        let offset = self.header.offset();
        let size   = self.header.body_size();
        let start  = offset + index * size;
        let end    = start + size;
        
        data.get_mut(start..end)
            .ok_or(Error::OutOfBoundsError)?
            .copy_from_slice(&self.data);

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

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn name(&self) -> String {
        self.name.clone()
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

    pub fn slice_unchecked(&self, offset: usize, size: usize) -> &[u8] {
        &self.data[offset..offset + size]
    }

    pub fn slice_mut_unchecked(&mut self, offset: usize, size: usize) -> &mut [u8] {
        &mut self.data[offset..offset + size]
    }

    pub fn slice(&self, offset: usize, size: usize) -> Result<&[u8]> {
        if offset + size <= self.data.len() {
            Ok(self.slice_unchecked(offset, size))
        } else {
            Err(Error::OutOfBoundsError)
        }
    }

    pub fn slice_mut(&mut self, offset: usize, size: usize) -> Result<&mut [u8]> {
        if offset + size <= self.data.len() {
            Ok(self.slice_mut_unchecked(offset, size))
        } else {
            Err(Error::OutOfBoundsError)
        }
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }

    pub fn size(&self) -> usize {
        self.header.size() +
        self.data.len()
    }

    pub fn name_index(&self) -> usize {
        self.header.name_index() as usize
    }

    pub fn body_size(&self) -> usize {
        self.header.body_size()
    }

    pub fn set_body_size(&mut self, body_size: usize) {
        self.header.set_body_size(body_size);
    }

    pub fn entity_size(&self) -> usize {
        self.header.entsize()
    }

    pub fn entity_count(&self) -> usize {
        self.body_size() / self.entity_size()
    }

    pub fn address(&self) -> usize {
        self.header.address() as usize
    }

    pub fn offset(&self) -> usize {
        self.header.offset()
    }

    pub fn start(&self) -> usize {
        self.offset()
    }

    pub fn end(&self) -> usize {
        self.offset() + self.body_size()
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

impl Updateable for Section {
    fn update(&mut self) {
        self.header.update();
        Update::apply(self);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::headers::{FileHeader,SectionHeader};
    use crate::common::SHType;
    use crate::utilities::read;

    #[test]
    fn test_section_read() {
        let b = read("assets/libjpeg/libjpeg.so.9").unwrap();

        let file_header = FileHeader::parse(&b).unwrap();

        let count = file_header.shnum();
        let offset = file_header.shoff();
        let size = file_header.shentsize();
        let layout = file_header.data();
        let width = file_header.class();
        
        let headers = SectionHeader::parse_all(
            &b,
            count,
            offset,
            size,
            layout,
            width).unwrap();

        let header: SectionHeader = headers
            .iter()
            .find(|&h| h
                .kind() == SHType::SHT_RELA)
            .unwrap()
            .clone();

        let result = Section::read(header,&b);
        assert!(result.is_ok())

    }
}