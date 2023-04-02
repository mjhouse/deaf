use std::path::Path;
use std::sync::{Arc,Mutex};

use crate::{Section,Segment};
use crate::headers::{FileHeader,SectionHeader,ProgramHeader};
use crate::errors::{Result,Error};
use crate::common::{ Width, Layout, Data };

/// An ELF formatted binary file
pub struct Binary {
    header: FileHeader,
    data: Data,
}

impl Binary {

    /// Load a file as an ELF Binary object
    pub fn new<T: AsRef<Path>>(path: T) -> Result<Self> {
        let data = std::fs::read(path.as_ref())?;
        let header = FileHeader::parse(&data)?;
        Ok(Self { header, data: Arc::new(Mutex::new(data)) })
    }

    /// Get a vector of all sections in the binary
    pub fn sections(&self) -> Result<Vec<Section>> {
        let count = self.shnum()?;
        let offset = self.shoff()?;
        let size = self.shentsize()?;
        let layout = self.layout()?;
        let width = self.width()?;

        let data: &[u8] = &self.data.lock()?;

        SectionHeader::parse_all(
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
    }

    /// Get a vector of all segments in the binary
    pub fn segments(&self) -> Result<Vec<Segment>> {
        let count = self.phnum()?;
        let offset = self.phoff()?;
        let size = self.phentsize()?;
        let layout = self.layout()?;
        let width = self.width()?;

        let data: &[u8] = &self.data.lock()?;

        ProgramHeader::parse_all(
            data,
            count,
            offset,
            size,
            layout,
            width
        ).map(|v| v
            .into_iter()
            .map(|header| Segment::new(
                header,
                self.data.clone()))
            .collect())
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