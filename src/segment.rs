use std::cmp::{max, min};

use crate::{common::{Layout, SectionType, Width}, headers::ProgramHeader, Binary, Section};
use crate::errors::Result;

/// A Segment extracted from an ELF file
#[derive(Debug,Clone)]
pub struct Segment {
    header: ProgramHeader,
}

impl Segment {

    /// Create a new segment from a program header
    pub fn new(header: ProgramHeader) -> Self {
        Self { header }
    }

    /// Read all segments from the given data
    pub fn read_all(
        data: &[u8], 
        count: usize, 
        offset: usize, 
        size: usize, 
        layout: Layout, 
        width: Width
    ) -> Result<Vec<Self>> {
        (0..count)
            .into_iter()
            .map(|i| offset + i * size)
            .map(|i| ProgramHeader::parse(
                &data[i..],
                layout,
                width
            ))
            .map(|r| r.map(Segment::new))
            .collect()
    }

    /// Get the size of the segment body
    pub fn body_size(&self) -> usize {
        self.header.body_size()
    }

    /// Get the offset of the segment in the data
    pub fn offset(&self) -> usize {
        self.header.offset()
    }

    /// Alias for [offset](Segment::offset)
    pub fn start(&self) -> usize {
        self.offset()
    }

    /// Get the ending byte offset of the segment body
    pub fn end(&self) -> usize {
        self.offset() + self.body_size()
    }

    /// Check if the segment contains the given section
    pub fn contains(&self, section: &Section) -> bool {
        section.start() >= self.start() &&
        section.end() <= self.end()
    }

    /// Check if the segment overlaps the given section
    pub fn overlaps(&self, section: &Section) -> bool {
        let a = max(self.start() as i64,section.start() as i64);
        let b = min(self.end() as i64,section.end() as i64);
        a - b <= 0
    }

    /// Get the sections for the segment
    pub fn sections<'a>(&self, binary: &'a Binary) -> Vec<&'a Section> {
        binary.sections(SectionType::Any)
            .into_iter()
            .filter(|s| self.contains(s) || ( self.overlaps(s) && s.is_kind(SectionType::Empty) ))
            .collect()
    }

    /// Get the mutable sections for the segment
    pub fn sections_mut<'a>(&self, binary: &'a mut Binary) -> Vec<&'a mut Section> {
        binary.sections_mut(SectionType::Any)
            .into_iter()
            .filter(|s| self.contains(s) || ( self.overlaps(s) && s.is_kind(SectionType::Empty) ))
            .collect()
    }

}
