use crate::errors::{Error,Result};
use crate::tables::common::ByteIter;
use crate::tables::table_item::TableItem;
use crate::headers::common::constants::{Width,Layout};

pub struct Table<T>
where
    T: TableItem
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
    T: TableItem
{

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

    pub fn read(&mut self, bytes: &[u8]) -> Result<usize> {
        let start = self.table_offset;
        let end = start + self.table_size;

        let size = self.item_size;

        // check that the entity size is > 0
        if size == 0 {
            return Err(Error::MalformedDataError);
        }

        // check that the table has data
        if self.table_size == 0 {
            return Err(Error::MalformedDataError);
        }

        // check that entities fit cleanly into section
        if self.table_size % size != 0 {
            return Err(Error::MalformedDataError);
        }

        // reserve a temporary buffer for items
        let mut items: Vec<T> = vec![];
        items.reserve(self.table_size / size);

        // build a delimiter for the item type
        let delim = T::delimiter(size);

        // iterate over entity-sized slices of the byte array
        for data in ByteIter::new(&bytes[start..end],delim) {
            // // parse a symbol from the byte range
            // let symbol = Symbol::parse(
            //     data,
            //     self.layout,
            //     self.width)?;

            // // add to vector of Symbol objects
            // values.push(symbol);
        }

        // don't update self until successful read
        self.items = items;
        Ok(self.items.len())
    }

    /// Write to buffer, returning the number of items written
    pub fn write(&self, bytes: &mut [u8]) -> Result<usize> {

        // check buffer is big enough
        if bytes.len() > self.size() {
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
            item.write(buffer)?;

            // update the offset for the next item
            offset += item.size();
        }

        // write to output buffer on success
        bytes.copy_from_slice(&items);

        Ok(self.len())
    }

    /// Get the number of items in the table
    fn len(&self) -> usize {
        self.items.len()
    }

    /// Get the calculated size of the table
    fn size(&self) -> usize {
        self.items
            .iter()
            .fold(0,|a,v| a + v.size())
    }

    /// Get an item at a given index
    fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    /// Set index to item, returning the index
    fn set(&mut self, index: usize, item: T) -> Result<usize> {
        if self.items.len() > index {
            self.items[index] = item;
            Ok(index)
        } else {
            Err(Error::OutOfBoundsError)
        }
    }

    /// Add item, returning the index
    fn add(&mut self, item: T) -> Result<usize> {
        self.items.push(item);
        Ok(self.len().saturating_sub(1))
    }

    /// Delete and return an item
    fn del(&mut self, index: usize) -> Option<T> {
        if self.items.len() > index {
            Some(self.items.remove(index))
        } else {
            None
        }
    }

}