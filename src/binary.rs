use std::path::Path;
use std::sync::{Arc,Mutex};

use crate::{Section, Segment};
use crate::headers::{FileHeader, SectionHeader, ProgramHeader};
use crate::errors::{Result,Error};
use crate::common::{Width, Layout, Data};
use crate::tables::{StringTable};

/// An ELF formatted binary file
pub struct Binary {
    header: FileHeader,
    data: Data,
    sections: Vec<Section>,
    names: Option<StringTable>
}

impl Binary {

    /// Load a file as an ELF Binary object
    pub fn new<T: AsRef<Path>>(path: T) -> Result<Self> {
        let data = std::fs::read(path.as_ref())?;
        let header = FileHeader::parse(&data)?;

        Ok(Self { 
            header: header, 
            data: Arc::new(Mutex::new(data)),
            sections: Vec::new(),
            names: None,
        })
    }

    /// Get a vector of all sections in the binary
    pub fn sections(&mut self) -> Result<&Vec<Section>> {
        if self.sections.len() == 0 {
            let count = self.shnum()?;
            let offset = self.shoff()?;
            let size = self.shentsize()?;
            let layout = self.layout()?;
            let width = self.width()?;
    
            let data: &[u8] = &self.data.lock()?;
    
            self.sections = SectionHeader::parse_all(
                data,
                count,
                offset,
                size,
                layout,
                width
            ).map(|v| v
                .into_iter()
                .map(|header| Section::new(
                    header,
                    self.data.clone()))
                .collect())
            .unwrap_or(Vec::new());
        }
        Ok(&self.sections)
    }

    // Get a single section by index
    pub fn section(&mut self, index: usize) -> Option<&Section> {
        self.sections()
            .ok()?
            .get(index)
    }

    /// Get a vector of all string tables in the binary
    pub fn section_names(&mut self) -> Result<&StringTable> {
        if self.names.is_none() {
            let index = self.header.shstrndx().ok_or(Error::NotFound)?;
            let section = self.section(index).ok_or(Error::NotFound)?;
            
            let mut table = StringTable::try_from(section.header())?;

            let data: &[u8] = &self.data.lock()?;
            table.read(data);

            self.names = Some(table);
        }
        self.names.as_ref().ok_or(Error::NotFound)
    }

    pub fn section_name(&mut self, index: usize) -> Option<String> {
        self.section_names()
            .ok()?
            .get(index)
            .map(|v| v.string_lossy())
    }

    /// Get the number of section headers in the file
    pub fn shnum(&self) -> Result<usize> {
        self.header.shnum().ok_or(Error::NotFound)
    }

    /// Get the offset of the section header table
    pub fn shoff(&self) -> Result<usize> {
        self.header.shoff().ok_or(Error::NotFound)
    }

    /// Get the size of section headers
    pub fn shentsize(&self) -> Result<usize> {
        self.header.shentsize().ok_or(Error::NotFound)
    }

    /// Get the number of program headers in the file
    pub fn phnum(&self) -> Result<usize> {
        self.header.phnum().ok_or(Error::NotFound)
    }

    /// Get the offset of the program header table
    pub fn phoff(&self) -> Result<usize> {
        self.header.phoff().ok_or(Error::NotFound)
    }

    /// Get the size of program headers
    pub fn phentsize(&self) -> Result<usize> {
        self.header.phentsize().ok_or(Error::NotFound)
    }

    /// Get the layout of the file (little or big endian)
    pub fn layout(&self) -> Result<Layout> {
        self.header.data().ok_or(Error::NotFound)
    }

    /// Get the addressing width of the file (32, 64 etc)
    pub fn width(&self) -> Result<Width> {
        self.header.class().ok_or(Error::NotFound)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_string_table() {
        let mut binary = Binary::new("assets/libjpeg/libjpeg.so.9").unwrap();

        let name = binary.section_name(1);
        assert!(name.is_some());

        let value = name.unwrap();
        assert_eq!(value.as_str(),".symtab");
    }
}