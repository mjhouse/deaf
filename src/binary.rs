use std::path::Path;
use crate::{Section};
use crate::headers::{FileHeader};
use crate::errors::{Result};
use crate::common::{Layout,Width};

/// An ELF formatted binary file
pub struct Binary {
    header: FileHeader,
    sections: Vec<Section>,
}

impl Binary {

    fn new(header: FileHeader, sections: Vec<Section>) -> Self {
        Self { header, sections }
    }

    pub fn load<T: AsRef<Path>>(path: T) -> Result<Self> {
        let data = std::fs::read(path.as_ref())?;
        let header = FileHeader::parse(&data)?;

        let count = header.shnum();
        let offset = header.shoff();
        let size = header.shentsize();
        let layout = header.layout();
        let width = header.width();

        let sections = Section::read_all(
            &data,
            count,
            offset,
            size,
            layout,
            width
        )?;

        Ok(Self::new(header,sections))
    }

    pub fn save<T: AsRef<Path>>(&self, path: T) -> Result<usize> {
        let size = self.size();

        let mut data = Vec::new();
        data.reserve_exact(size);

        let offset = self.header.shoff();
        self.header.write(&mut data)?;

        for (index,section) in self.sections.iter().enumerate() {
            section.write(
                &mut data,
                offset,
                index,
            )?;
        }

        std::fs::write(path, data)?;
        Ok(size)
    }

    pub fn size(&self) -> usize {
        self.header.size() +
        self.sections
            .iter()
            .fold(0,|a,s| a + s.size())
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
    // use super::*;

    // #[test]
    // fn test_read_string_table() {
    //     let mut binary = Binary::new("assets/libjpeg/libjpeg.so.9").unwrap();

    //     let name = binary.section_name(1);
    //     assert!(name.is_some());

    //     let value = name.unwrap();
    //     assert_eq!(value.as_str(),".symtab");
    // }
}