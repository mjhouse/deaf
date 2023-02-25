use crate::errors::{Error, Result};
use crate::headers::common::constants::{Width,Layout,SHType,sizes};
use crate::headers::section::header::{
    SectionHeader,
    SectionHeaderValues
};
use crate::tables::relocation::Relocation;
use crate::tables::common::ByteIter;
use crate::tables::common::Table;

pub struct RelocationTable {
    offset: usize,
    layout: Layout,
    width: Width,
    entity_size: usize,
    section_size: usize,
    values: Vec<Relocation>
}

impl RelocationTable {

    pub fn new(offset: usize, size: usize, layout: Layout, width: Width, entity_size: usize) -> Self {
        Self {
            offset: offset,
            layout: layout,
            width: width,
            entity_size: entity_size,
            section_size: size,
            values: vec![],
        }
    }

    // reads from an offset to offset + section_size
    pub fn read(&mut self, bytes: &[u8]) -> Result<usize> {
        let start = self.offset;
        let end = self.offset + self.section_size;

        let size = self.entity_size;
        let layout = self.layout;
        let width = self.width;

        // check that the entity size is > 0
        if size == 0 {
            return Err(Error::MalformedDataError);
        }

        // check that the section has data
        if self.section_size == 0 {
            return Err(Error::MalformedDataError);
        }

        // check that entities fit cleanly into section
        if self.section_size % size != 0 {
            return Err(Error::MalformedDataError);
        }

        // reserve a temporary buffer for relocations
        let mut values = vec![];
        values.reserve(self.section_size / size);

        for data in ByteIter::length(&bytes[start..end],size) {
            // parse a relocation from the byte range
            let relocation = Relocation::parse(data,layout,width)?;

            // add to vector of Relocation objects
            values.push(relocation);
        }

        // don't update self until successful read
        self.values = values;
        Ok(self.values.len())
    }

    // writes from the beginning of the given byte slice
    pub fn write(&self, bytes: &mut [u8]) -> Result<usize> {

        // check buffer is big enough
        if bytes.len() > self.size() {
            return Err(Error::OutOfBoundsError);
        }

        let size = self.entity_size;
        let layout = self.layout;
        let width = self.width;

        // iterate all contained relocations
        for (i,relocation) in self.values.iter().enumerate() {
            
            // calculate relocation position in the output buffer
            let reloc_start = i * size;
            let reloc_end = reloc_start + size;

            // get a constrained, mutable slice of bytes to write to
            let buffer = &mut bytes[reloc_start..reloc_end];

            // write the relocation to the byte slice
            relocation.write(buffer)?;
        }

        Ok(self.values.len())
    }

}

impl Table<Relocation> for RelocationTable {

    fn len(&self) -> usize {
        self.values.len()
    }

    fn size(&self) -> usize {
        self.len() * self.entity_size
    }

    fn get(&self, index: usize) -> Option<Relocation> {
        self.values.get(index).cloned()
    }

    fn set(&mut self, index: usize, item: Relocation) -> Result<usize> {
        self.values[index] = item;
        Ok(index)
    }

    fn add(&mut self, item: Relocation) -> Result<usize> {
        self.values.push(item);
        Ok(self.len().saturating_sub(1))
    }

    fn del(&mut self, index: usize) -> Option<Relocation> {
        if self.values.len() > index {
            Some(self.values.remove(index))
        } else {
            None
        }
    }

}

impl TryFrom<SectionHeader> for RelocationTable {
    type Error = Error;

    fn try_from(header: SectionHeader) -> Result<Self> {
        match header.values.sh_type {
            SHType::SHT_RELA | SHType::SHT_REL => Ok(Self::new(
                header.offset(),
                header.size(),
                header.layout(),
                header.width(),
                header.entsize()
            )),
            _ => Err(Error::WrongSectionError)
        }
    }
}