use crate::errors::{Error,Result};
use crate::common::{ByteIter};
use crate::tables::TableItem;
use crate::headers::SectionHeader;
use crate::Section;

/// A section interpreted as a table
///
/// Each Table instance is considered to be 
/// a series of records of type `T`. Tables can
/// be parsed from a SectionHeader and a byte
/// buffer containing the body of the section.
pub struct Table<T>
where
    T: TableItem + Default
{
    pub(crate) header: SectionHeader,
    pub(crate) items: Vec<T>
}

impl<T> Table<T>
where
    T: TableItem + Default
{

    pub fn new(header: &SectionHeader) -> Self {
        Self {
            header: header.clone(),
            items: vec![]
        }
    }

    /// Read from buffer, returning the table
    pub fn parse(mut self, data: &[u8]) -> Result<Self> {
        self.read(data)?;
        Ok(self)
    }

    /// Read from fitted slice, returning the number of items read
    pub fn read_slice(&mut self, bytes: &[u8]) -> Result<usize> {
        let end = self.header.body_size();
        let size = self.header.entsize();
        let limit = bytes.len();

        if end > limit {
            return Err(Error::OutOfBoundsError);
        }

        // reserve a temporary buffer for items
        let mut items: Vec<T> = vec![];

        // if a size is given, reserve space upfront
        if size > 0 {
            items.reserve(self.header.body_size() / size);
        }

        // build a delimiter for the item type
        let delim = T::delimiter(size);

        // iterate over entity-sized slices of the byte array
        for data in ByteIter::new(&bytes[..end],delim) {
            let mut item = T::default();

            // set expected layout and width from table
            item.set_layout(self.header.layout());
            item.set_width(self.header.width());

            // parse the table item and add to collection
            item.read(data)?;
            items.push(item);
        }

        // don't update self until successful read
        self.items = items;
        Ok(self.items.len())
    }

    /// Read from buffer, returning the number of items read
    pub fn read(&mut self, bytes: &[u8]) -> Result<usize> {
        let start = self.header.offset();
        let end = start + self.header.body_size();

        let size = self.header.entsize();
        let limit = bytes.len();

        if end > limit {
            return Err(Error::OutOfBoundsError);
        }

        // reserve a temporary buffer for items
        let mut items: Vec<T> = vec![];

        // if a size is given, reserve space upfront
        if size > 0 {
            items.reserve(self.header.body_size() / size);
        }

        // build a delimiter for the item type
        let delim = T::delimiter(size);

        // iterate over entity-sized slices of the byte array
        for data in ByteIter::new(&bytes[start..end],delim) {
            let mut item = T::default();

            // set expected layout and width from table
            item.set_layout(self.header.layout());
            item.set_width(self.header.width());

            // parse the table item and add to collection
            item.read(data)?;
            items.push(item);
        }

        // don't update self until successful read
        self.items = items;
        Ok(self.items.len())
    }

    /// Write to buffer, returning the number of items written
    pub fn write(&self, bytes: &mut [u8]) -> Result<usize> {
        let size = self.size();
        let mut offset = 0;

        // check buffer is big enough
        if bytes.len() < size {
            return Err(Error::OutOfBoundsError);
        }

        // reserve a temporary buffer for items
        let mut items = vec![0u8;size];

        // iterate all contained items
        for item in self.items.iter() {
            
            // calculate position in the output buffer
            let start = offset;
            let end = start + item.size();

            // get a constrained, mutable slice of bytes
            let buffer = &mut items[start..end];

            // write the item to the byte slice
            item.write(buffer).unwrap();

            // update the offset for the next item
            offset += item.size();
        }

        // write to output buffer on success
        bytes.copy_from_slice(&items);

        Ok(self.len())
    }

    /// Get the number of items in the table
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Get the calculated size of the table in bytes
    pub fn size(&self) -> usize {
        self.items
            .iter()
            .fold(0,|a,v| a + v.size())
    }

    /// Get an item at a given index
    pub fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    pub fn all(&self) -> &Vec<T> {
        &self.items
    }

    /// Set index to item, returning the index
    pub fn set(&mut self, index: usize, item: T) -> Result<usize> {
        if self.items.len() > index {
            self.items[index] = item;
            Ok(index)
        } else {
            Err(Error::OutOfBoundsError)
        }
    }

    /// Insert an item at particular index, returning the
    /// new size of the table.
    pub fn insert(&mut self, index: usize, item: T) -> Result<usize> {
        self.items.insert(index,item);
        Ok(self.len().saturating_sub(1))
    }

    /// Add item, returning the index
    pub fn add(&mut self, item: T) -> Result<usize> {
        self.items.push(item);
        Ok(self.len().saturating_sub(1))
    }

    /// Delete and return an item
    pub fn del(&mut self, index: usize) -> Option<T> {
        if self.items.len() > index {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

    pub fn header(&self) -> SectionHeader {
        self.header.clone()
    }

    pub fn data(&self) -> Result<Vec<u8>> {
        let mut buffer = vec![0;self.size()];
        self.write(buffer.as_mut_slice())?;
        Ok(buffer)
    }

}

impl<T> TryFrom<&Table<T>> for Section
where
    T: TableItem + Default
{
    type Error = Error;

    fn try_from(table: &Table<T>) -> Result<Self> {
        let header = table.header();
        let data = table.data()?;
        let mut section = Section::new(header);
        section.set_data(data);
        Ok(section)
    }
}

impl<T> TryFrom<Table<T>> for Section
where
    T: TableItem + Default
{
    type Error = Error;

    fn try_from(table: Table<T>) -> Result<Self> {
        Self::try_from(&table)
    }
}