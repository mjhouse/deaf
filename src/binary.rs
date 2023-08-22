use std::path::Path;
use std::sync::{Arc,Mutex};
use core::ops::Range;

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

    pub fn read(&mut self) -> Result<()> {
        // let count = self.header.shnum()?;
        // let offset = self.header.shoff()?;
        // let size = self.header.shentsize()?;
        // let layout = self.header.layout();
        // let width = self.header.width();
        // let index = self.header.shstrndx()?;

        // let data: &[u8] = &self.data.lock()?;

        // let headers = SectionHeader::parse_all(
        //     data,
        //     count,
        //     offset,
        //     size,
        //     layout,
        //     width
        // )

        // let mut table = StringTable::try_from(index)?;
        // table.parse(data);

        // let sections = headers
        //     .iter()
        //     .map(|h| Section::new(
        //         h,
        //         data
        //     ))
        //     .collect();

        Ok(())
    }

    pub fn write(&self) -> Result<()> {
        Ok(())
    }

    // /// Get a vector of all section headers in the binary
    // pub fn headers(&self) -> Result<Vec<SectionHeader>> {
    //     let count = self.shnum()?;
    //     let offset = self.shoff()?;
    //     let size = self.shentsize()?;
    //     let layout = self.layout()?;
    //     let width = self.width()?;

    //     let data: &[u8] = &self.data.lock()?;

    //     SectionHeader::parse_all(
    //         data,
    //         count,
    //         offset,
    //         size,
    //         layout,
    //         width
    //     )
    // }

    // /// Get a vector of all sections in the binary
    // pub fn sections(&self) -> Result<Vec<Section>> {
    //     Ok(self.headers()?
    //         .into_iter()
    //         .map(|h| Section::new(
    //             h,
    //             self.data.clone()))
    //         .collect())
    // }

    // // Get a single section by index
    // pub fn section_header(&self, index: usize) -> Option<SectionHeader> {
    //     self.headers()
    //         .ok()?
    //         .into_iter()
    //         .nth(index)
    // }

    // // Get a single section by index
    // pub fn section_by_index(&self, index: usize) -> Option<Section> {
    //     self.sections()
    //         .ok()?
    //         .into_iter()
    //         .nth(index)
    // }

    // // Get a single section by index
    // pub fn section_by_name(&self, name: String) -> Option<Section> {
    //     self.sections()
    //         .ok()?
    //         .into_iter()
    //         .filter(|s| s.name(self) == Some(name.clone()))
    //         .collect::<Vec<Section>>()
    //         .pop()
    // }

    // /// Get a vector of all string tables in the binary
    // pub fn section_names(&self) -> Result<StringTable> {
    //     self.header
    //         .shstrndx()
    //         .and_then(|i| self.section_header(i))
    //         .ok_or(Error::NotFound)
    //         .and_then(StringTable::try_from)
    //         .and_then(|t| t.parse(&self.data))
    // }

    // pub fn section_name(&self, index: usize) -> Option<String> {
    //     self.section_names()
    //         .ok()?
    //         .get(index)
    //         .cloned()
    //         .map(|v| v.string_lossy())
    // }

    // /// Get the number of section headers in the file
    // pub fn shnum(&self) -> Result<usize> {
    //     self.header.shnum().ok_or(Error::NotFound)
    // }

    // /// Get the offset of the section header table
    // pub fn shoff(&self) -> Result<usize> {
    //     self.header.shoff().ok_or(Error::NotFound)
    // }

    // /// Get the size of section headers
    // pub fn shentsize(&self) -> Result<usize> {
    //     self.header.shentsize().ok_or(Error::NotFound)
    // }

    // /// Get the number of program headers in the file
    // pub fn phnum(&self) -> Result<usize> {
    //     self.header.phnum().ok_or(Error::NotFound)
    // }

    // /// Get the offset of the program header table
    // pub fn phoff(&self) -> Result<usize> {
    //     self.header.phoff().ok_or(Error::NotFound)
    // }

    // /// Get the size of program headers
    // pub fn phentsize(&self) -> Result<usize> {
    //     self.header.phentsize().ok_or(Error::NotFound)
    // }

    // /// Get the layout of the file (little or big endian)
    // pub fn layout(&self) -> Result<Layout> {
    //     self.header.data().ok_or(Error::NotFound)
    // }

    // /// Get the addressing width of the file (32, 64 etc)
    // pub fn width(&self) -> Result<Width> {
    //     self.header.class().ok_or(Error::NotFound)
    // }

}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_read_string_table() {
    //     let mut binary = Binary::new("assets/libjpeg/libjpeg.so.9").unwrap();

    //     let name = binary.section_name(1);
    //     assert!(name.is_some());

    //     let value = name.unwrap();
    //     assert_eq!(value.as_str(),".symtab");
    // }
}