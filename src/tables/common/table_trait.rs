use crate::errors::Result;

pub trait Table<T> {

    /// the number of elements in the table
    fn len(&self) -> usize;

    /// the size in bytes of the table
    fn size(&self) -> usize;

    /// get the item at `index` or return None
    fn get(&self, index: usize) -> Option<T>;

    /// set an index position to `item` returning the 
    /// index on success
    fn set(&mut self, index: usize, item: T) -> Result<usize>;

    /// add the item at the end of the table returning 
    /// the index on success
    fn add(&mut self, item: T) -> Result<usize>;

    /// Delete an element from the table, returning the 
    /// element on success or None
    fn del(&mut self, index: usize) -> Option<T>;

}