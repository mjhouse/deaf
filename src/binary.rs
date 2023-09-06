use std::path::Path;
use std::fs;

use crate::Section;
use crate::tables::{Table,TableView,StringItem};
use crate::headers::FileHeader;
use crate::errors::{Error,Result};
use crate::common::{
    Layout,
    Width,
    SectionType,
    Updateable,
    Update,
    All
};

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

    pub fn section(&self, index: usize) -> Result<&Section> {
        self.sections
            .get(index)
            .ok_or(Error::NotFound)
    }

    pub fn section_mut(&mut self, index: usize) -> Result<&mut Section> {
        self.sections
            .get_mut(index)
            .ok_or(Error::NotFound)
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

    pub fn section_name(&self, offset: usize) -> Result<String> {
        self.section(self.header.shstrndx())
            .and_then(Table::<StringItem>::try_from)
            .and_then(|t| t
                .at_offset(offset)
                .and_then(|e| e.string()))
    }

    pub fn section_names(&self) -> Result<Vec<String>> {
        self.section(self.header.shstrndx())
            .and_then(Table::<StringItem>::try_from)
            .and_then(|t| t
                .items())
            .and_then(|v| v
                .iter()
                .map(|e| e.string())
                .collect())
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

    pub fn shstrndx(&self) -> usize {
        self.header.shstrndx()
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

impl Updateable for Binary {
    fn update(&mut self) {
        self.header.update();
        self.sections.update();
        Update::<All>::clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_string_table() {
        let binary = Binary::load("assets/libjpeg/libjpeg.so.9").unwrap();

        let names = binary
            .sections(SectionType::Strings)
            .iter()
            .map(|s| s.name())
            .map(|i| binary.section_name(i))
            .collect::<Result<Vec<String>>>()
            .unwrap();

        assert_eq!(names[0].as_str(),".dynstr");
        assert_eq!(names[1].as_str(),".shstrtab");
        assert_eq!(names[2].as_str(),".strtab");
    }

    #[test]
    fn test_display_sections() {
        let path = "assets/libvpf/libvpf.so.4.1";
        let binary = Binary::load(path).unwrap();

        for (i,section) in binary.sections.iter().enumerate() {
            let kind = section.kind();
            let index = section.name();
            let name = binary.section_name(index).unwrap();

            println!("{}: {} (kind={:?})",i,name,kind);
        }
    }

}