use crate::common::ranges::ADDRESS;
use crate::errors::{Error,Result};
use crate::common::{ByteIter,SHType,Layout,Width,Item};
use crate::Section;

type Value = Item<u32,u64>;

/// A Section represented as an immutable HashTable
pub struct HashTable<'a> {
    section: &'a Section
}

/// A Section represented as a mutable HashTable
pub struct HashTableMut<'a> {
    section: &'a mut Section
}

impl<'a> HashTable<'a> {

    fn new(section: &'a Section) -> Self {
        Self { section }
    }

    fn read(&self, offset: usize) -> Result<u64> {
        Value::new(ADDRESS)
            .with_layout(self.layout())
            .with_width(self.width())
            .parse(&self.section.data()[offset..])
            .map(|v| v.get())
    }

    fn nbuckets(&self) -> u64 {
        self.read(0)
            .expect("Failed to read nbuckets")
    }

    fn nchain(&self) -> u64 {
        self.read(0)
            .expect("Failed to read nchain")
    }

    fn layout(&self) -> Layout {
        self.section.layout()
    }

    fn width(&self) -> Width {
        self.section.width()
    }

}