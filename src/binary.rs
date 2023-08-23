use std::path::Path;
use std::fs;

use crate::{Section};
use crate::headers::{FileHeader};
use crate::errors::{Result};
use crate::common::{Layout,Width,SectionType};

/// An ELF formatted binary file
pub struct Binary {
    header: FileHeader,
    sections: Vec<Section>,
}

impl Binary {

    fn empty() -> Self {
        Self { 
            header: FileHeader::new(), 
            sections: Vec::new()
        }
    }

    fn new(header: FileHeader, sections: Vec<Section>) -> Self {
        Self { header, sections }
    }

    pub fn read(&mut self, data: &[u8]) -> Result<usize> {
        self.header = FileHeader::parse(&data)?;

        let count = self.header.shnum();
        let offset = self.header.shoff();
        let size = self.header.shentsize();
        let layout = self.header.layout();
        let width = self.header.width();

        self.sections = Section::read_all(
            &data,
            count,
            offset,
            size,
            layout,
            width
        )?;

        Ok(self.size())
    }

    pub fn write(&self, data: &mut [u8]) -> Result<usize> {
        self.header.write(data)?;
        let offset = self.header.shoff();

        for (index,section) in self.sections.iter().enumerate() {
            section.write(
                data,
                offset,
                index,
            )?;
        }

        Ok(self.size())
    }

    pub fn load<T: AsRef<Path>>(path: T) -> Result<Self> {
        let data = fs::read(path.as_ref())?;
        let mut binary = Binary::empty();        
        binary.read(&data)?;
        Ok(binary)
    }

    pub fn save<T: AsRef<Path>>(&self, path: T) -> Result<usize> {
        let size = self.size();
        let mut data = vec![0;size];
        self.write(&mut data)?;
        fs::write(path, data)?;
        Ok(size)
    }

    pub fn size(&self) -> usize {
        self.header.size() +
        self.sections
            .iter()
            .fold(0,|a,s| a + s.size())
    }

    pub fn sections(&self, kind: SectionType) -> Vec<&Section> {
        self.sections
            .iter()
            .filter(|s| s.is_kind(kind))
            .collect()
    }

    pub fn sections_mut(&mut self, kind: SectionType) -> Vec<&mut Section> {
        self.sections
            .iter_mut()
            .filter(|s| s.is_kind(kind))
            .collect()
    }

    /// Get the number of section headers in the file
    pub fn shnum(&self) -> usize {
        self.header.shnum()
    }

    /// Get the offset of the section header table
    pub fn shoff(&self) -> usize {
        self.header.shoff()
    }

    /// Get the size of section headers
    pub fn shentsize(&self) -> usize {
        self.header.shentsize()
    }

    /// Get the number of program headers in the file
    pub fn phnum(&self) -> usize {
        self.header.phnum()
    }

    /// Get the offset of the program header table
    pub fn phoff(&self) -> usize {
        self.header.phoff()
    }

    /// Get the size of program headers
    pub fn phentsize(&self) -> usize {
        self.header.phentsize()
    }

    /// Get the layout of the file (little or big endian)
    pub fn layout(&self) -> Layout {
        self.header.data()
    }

    /// Get the addressing width of the file (32, 64 etc)
    pub fn width(&self) -> Width {
        self.header.class()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_string_table() {
        let mut binary = Binary::load("assets/libjpeg/libjpeg.so.9").unwrap();

        for section in binary.sections(SectionType::Strings) {
            dbg!(section);
        }

        // let name = binary.section_name(1);
        // assert!(name.is_some());

        // let value = name.unwrap();
        // assert_eq!(value.as_str(),".symtab");
    }
}