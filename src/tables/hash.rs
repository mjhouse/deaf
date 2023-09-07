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

impl<'a> HashTableMut<'a> {

    fn new(section: &'a mut Section) -> Self {
        Self { section }
    }

}

impl<'a> TryFrom<&'a Section> for HashTable<'a> 
{
    type Error = Error;

    fn try_from(section: &'a Section) -> Result<Self> {
        match section.header().kind() {
            SHType::SHT_GNU_HASH | SHType::SHT_HASH => Ok(Self::new(section)),
            _ => Err(Error::WrongSectionError)
        }
    }
}

impl<'a> TryFrom<&'a mut Section> for HashTableMut<'a> 
{
    type Error = Error;

    fn try_from(section: &'a mut Section) -> Result<Self> {
        match section.header().kind() {
            SHType::SHT_GNU_HASH | SHType::SHT_HASH => Ok(Self::new(section)),
            _ => Err(Error::WrongSectionError)
        }
    }
}

impl<'a> From<HashTableMut<'a>> for HashTable<'a> 
{
    fn from(table: HashTableMut<'a>) -> Self {
        Self::new(table.section)
    }
}

impl<'a> From<HashTableMut<'a>> for &'a mut Section 
{
    fn from(table: HashTableMut<'a>) -> Self {
        table.section
    }
}

impl<'a> From<HashTable<'a>> for &'a Section 
{
    fn from(table: HashTable<'a>) -> Self {
        table.section
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::headers::FileHeader;
    use crate::utilities::read;

    use crate::utilities::tests::{
        LIBJPEG_DYNSYM as SYM_TEST,
        LIBVPF_SHSTRTAB as STR_TEST,
        LIBVPF_RELA_DYN as RELA_TEST,
        LIBQSCINTILLA_FINI_ARRAY as FINI_TEST,
        LIBQSCINTILLA_INIT_ARRAY as INIT_TEST, 
    };

    macro_rules! section {
        ( $path: expr, $index: expr ) => {
            read($path)
                .and_then(|d| FileHeader::parse(&d)
                .and_then(|h| Ok((d,h))))
                .and_then(|(d,h)|
                    Section::read_all(
                        &d,
                        h.shnum(),
                        h.shoff(),
                        h.shentsize(),
                        h.data(),
                        h.class()
                    )
                )
                .and_then(|s| s
                    .get($index)
                    .ok_or(Error::NotFound)
                    .cloned())
                .expect("Section not found")
        };
    }

    #[test]
    fn test_read_hash_table() {
        let section = section!("assets/libvpf/libvpf.so.4.1", 3);

        let table = HashTable::try_from(&section).unwrap();

        let nbuckets = table.nbuckets();
        dbg!(nbuckets);

    }
}