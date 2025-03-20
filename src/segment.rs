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
        let s1 = self.start();
        let e1 = self.end();
        let s2 = section.start();
        let e2 = section.end();

        (s2 >= s1 && s2 <= e1) && 
        (e2 >= s1 && e2 <= e1)
    }

    /// Check if the segment overlaps the given section
    pub fn overlaps(&self, section: &Section) -> bool {
        let s1 = self.start();
        let e1 = self.end();
        let s2 = section.start();
        let e2 = section.end();

        (s2 >= s1 && s2 <= e1) || 
        (e2 >= s1 && e2 <= e1)
    }

    /// Check if the segment includes the given section
    /// 
    /// A section is included if it is either completely contained
    /// by the segment OR is partially overlaps the segment and
    /// is flagged as empty (NOBITS).
    pub fn includes(&self, section: &Section) -> bool {
        self.contains(section) || ( 
            self.overlaps(section) && 
            section.is_kind(SectionType::Empty) 
        )
    }

    /// Get the sections for the segment
    pub fn sections<'a>(&self, binary: &'a Binary) -> Vec<&'a Section> {
        binary.sections(SectionType::Any)
            .into_iter()
            .filter(|s| self.includes(s))
            .collect()
    }

    /// Get the mutable sections for the segment
    pub fn sections_mut<'a>(&self, binary: &'a mut Binary) -> Vec<&'a mut Section> {
        binary.sections_mut(SectionType::Any)
            .into_iter()
            .filter(|s| self.includes(s))
            .collect()
    }

}
