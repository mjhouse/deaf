use std::marker::PhantomData;

use crate::errors::{Error,Result};
use crate::common::{ByteIter};
use crate::tables::TableItem;
use crate::headers::SectionHeader;
use crate::Section;

pub struct Table<'a,T>
where
    T: TableItem + Default
{
    item: PhantomData<T>,
    section: &'a Section
}

pub struct TableMut<'a,T>
where
    T: TableItem + Default
{
    item: PhantomData<T>,
    section: &'a mut Section
}

impl<'a,T> Table<'a,T>
where
    T: TableItem + Default
{

    /// Create a new table from the given section
    fn new(section: &'a Section) -> Self {
        Self {
            item: PhantomData {},
            section: section,
        }
    }

    /// Get the offset of an item from the index
    fn item_offset(&self, index: usize) -> usize {
        self.item_size() * index
    }

    /// Get the size of the table item
    fn item_size(&self) -> usize {
        self.section.entity_size()
    }

    /// Get the number of items in the table
    fn item_count(&self) -> usize {
        self.section.entity_count()
    }

    /// Read from a fitted slice, returning the item
    fn get(&self, index: usize) -> Result<T> {
        let data = self.section.data();
        let offset = self.item_offset(index);
        let mut item = T::default();

        // set expected layout and width from table
        item.set_layout(self.section.layout());
        item.set_width(self.section.width());

        item.read(&data[offset..])?;
        Ok(item)
    }

    /// Get all items from the table
    fn items(&self) -> Result<Vec<T>> {
        (0..self.item_count())
            .into_iter()
            .map(|i| self.get(i))
            .collect()
    }
}

impl<'a,T> TableMut<'a,T>
where
    T: TableItem + Default
{

    /// Create a new mutable table for the given section
    fn new(section: &'a mut Section) -> Self {
        Self {
            item: PhantomData {},
            section: section,
        }
    }

    /// Get the offset of an item from the index
    fn item_offset(&self, index: usize) -> usize {
        self.item_size() * index
    }

    /// Get the total size of the table
    fn table_size(&self) -> usize {
        self.section.body_size()
    }

    /// Get the size of the table item
    fn item_size(&self) -> usize {
        self.section.entity_size()
    }

    /// Get the number of items in the table
    fn item_count(&self) -> usize {
        self.section.entity_count()
    }

    /// Reserve bytes at the end of the buffer
    fn reserve_end(&mut self, size: usize) {
        let limit = self.table_size();

        // reserve additional space
        self.section
            .data_mut()
            .reserve(size);

        // update the section size
        self.section
            .set_body_size(limit + size);
    }

    /// Reserve bytes at the start of the buffer
    fn reserve_start(&mut self, size: usize) {
        // reserve additional space
        self.reserve_end(size);

        // rotate that space to the front
        self.section
            .data_mut()
            .rotate_right(size);
    }

    /// Read from a fitted slice, returning the item
    fn get(&self, index: usize) -> Result<T> {
        let data = self.section.data();
        let offset = self.item_offset(index);
        let mut item = T::default();

        // set expected layout and width from table
        item.set_layout(self.section.layout());
        item.set_width(self.section.width());

        item.read(&data[offset..])?;
        Ok(item)
    }

    /// Write to a fitted slice, returning the number of bytes written
    fn set(&mut self, index: usize, item: T) -> Result<usize> {
        let size  = self.item_size();
        let start = self.item_offset(index);
        let end   = start + size;
        let data  = self.section.data_mut();

        // get a constrained, mutable slice of bytes
        let buffer = &mut data[start..end];

        // write the item to the byte slice
        item.write(buffer)?;
        Ok(size)
    }

    /// Append an item to the table
    fn append(&mut self, item: T) -> Result<usize> {
        let size   = self.item_size();
        let offset = self.table_size();

        // reserve additional space
        self.reserve_end(size);

        // write to that position
        self.set(offset,item)
    }

    /// Prepend an item to the table
    fn prepend(&mut self, item: T) -> Result<usize> {
        let size   = self.item_size();
        let offset = 0;

        // reserve additional space
        self.reserve_start(size);

        // write to that position
        self.set(offset,item)
    }

    /// Remove an item from the table by index
    fn remove(&mut self, index: usize) -> Result<T> {
        let size  = self.item_size();
        let start = self.item_offset(index);
        
        // get the item from the buffer
        let item = self.get(index)?;

        // remove the data from the buffer
        self.section
            .data_mut()
            .drain(start..start + size);

        Ok(item)
    }

    /// Get all items from the table
    fn items(&self) -> Result<Vec<T>> {
        (0..self.item_count())
            .into_iter()
            .map(|i| self.get(i))
            .collect()
    }
}

impl<'a,T> From<TableMut<'a,T>> for Table<'a,T>
where
    T: TableItem + Default
{
    fn from(table: TableMut<'a,T>) -> Self {
        Self::new(table.section)
    }
}