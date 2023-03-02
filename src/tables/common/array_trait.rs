use crate::errors::Result;
use std::ops::{Index,IndexMut};

pub trait Array<T> {

    /// The number of elements in the array
    fn len(&self) -> usize;

    /// The size in bytes of the array
    fn size(&self) -> usize;

    /// Get the item at `index` or None
    fn get(&self, index: usize) -> Option<&T>;

    /// Get the mutable item at `index` or None
    fn get_mut(&mut self, index: usize) -> Option<&mut T>;

    /// Insert an item at the specified index
    fn insert(&mut self, index: usize, item: T);

    /// Add the item at the end of the array
    fn push(&mut self, item: T);

    /// Remove an element from the array
    fn remove(&mut self, index: usize) -> T;

}

impl<T> Index<usize> for dyn Array<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("Out of bounds")
    }
}

impl<T> IndexMut<usize> for dyn Array<T> {

    fn index_mut(&mut self, index: usize) -> &mut T {
        self.get_mut(index).expect("Out of bounds")
    }
}