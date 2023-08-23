use crate::errors::{Error,Result};
use crate::common::{ByteIter,Width,Layout};
use crate::tables::TableItem;

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
    table_offset: usize,
    table_size: usize,
    item_size: usize,
    layout: Layout,
    width: Width,
    items: Vec<T>
}

impl<T> Table<T>
where
    T: TableItem + Default
{
    pub fn empty() -> Self {
        Self {
            table_offset: 0,
            table_size: 0,
            item_size: 0,
            layout: Layout::Little,
            width: Width::X64,
            items: vec![]
        }
    }

    /// Create a new table from section information taken from
    /// a section header.
    pub fn new(table_offset: usize, table_size: usize, item_size: usize, layout: Layout, width: Width) -> Self {
        Self {
            table_offset,
            table_size,
            item_size,
            layout,
            width,
            items: vec![]
        }
    }

    /// Read from buffer, returning the table
    pub fn parse(mut self, data: &[u8]) -> Result<Self> {
        self.read(data)?;
        Ok(self)
    }

    /// Read from buffer, returning the number of items read
    pub fn read(&mut self, bytes: &[u8]) -> Result<usize> {
        let start = self.table_offset;
        let end = start + self.table_size;

        let size = self.item_size;

        // reserve a temporary buffer for items
        let mut items: Vec<T> = vec![];

        // if a size is given, reserve space upfront
        if size > 0 {
            items.reserve(self.table_size / size);
        }

        // build a delimiter for the item type
        let delim = T::delimiter(size);

        // iterate over entity-sized slices of the byte array
        for data in ByteIter::new(&bytes[start..end],delim) {
            let mut item = T::default();

            // set expected layout and width from table
            item.set_layout(self.layout);
            item.set_width(self.width);

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

        // check buffer is big enough
        if bytes.len() < self.size() {
            return Err(Error::OutOfBoundsError);
        }

        let mut offset = 0;

        // reserve a temporary buffer for items
        let mut items: Vec<u8> = vec![];
        items.resize(self.size(),0);

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

    /// Set index to item, returning the index
    pub fn set(&mut self, index: usize, item: T) -> Result<usize> {
        if self.items.len() > index {
            self.items[index] = item;
            Ok(index)
        } else {
            Err(Error::OutOfBoundsError)
        }
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

}