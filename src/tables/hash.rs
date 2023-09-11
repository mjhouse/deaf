use crate::common::ranges::{ADDRESS, NBUCKETS, NCHAIN, SYMOFFSET, BLOOMSIZE, BLOOMSHIFT, VALUE4};
use crate::errors::{Error,Result};
use crate::common::{ItemArray,ByteIter,SHType,Layout,Width,Item,FromBytes};
use crate::Section;

use super::{SymbolTable, StringTable, TableView};

type Value = Item<u32>;

/// A Section represented as an immutable HashTable
pub struct HashTable<'a> {
    bucket_count: Value,
    symoffset: Value,
    bloom_count: Value,
    bloom_shift: Value,
    bloom_array: ItemArray<u32,u64>,
    bucket_array: ItemArray<u32>,
    chain_array: ItemArray<u32>,
    section: &'a Section
}

/// A Section represented as a mutable HashTable
pub struct HashTableMut<'a> {
    section: &'a mut Section
}

impl<'a> HashTable<'a> {

    fn new(section: &'a Section) -> Self {
        let w = section.width();
        let l = section.layout();
        Self { 
            bucket_count: Value::make(NBUCKETS,w,l),
            symoffset: Value::make(SYMOFFSET,w,l),
            bloom_count: Value::make(BLOOMSIZE,w,l),
            bloom_shift: Value::make(BLOOMSHIFT,w,l),
            bloom_array: ItemArray::make(ADDRESS,w,l)
                .with_offset(16)
                .with_offset(0), // dynamic offset
            bucket_array: ItemArray::make(VALUE4,w,l)
                .with_offset(16)
                .with_offset(0), // dynamic offset
            chain_array: ItemArray::make(VALUE4,w,l)
                .with_offset(16)
                .with_offset(0), // dynamic offset
            section
        }
    }

    fn hash(&self, name: &str) -> u32 {
        let mut h: u32 = 0;
        let mut g: u32;
    
        for c in name.as_bytes() {
            h = (h << 4) + *c as u32;
            g = h & 0xf000_0000;
    
            if g != 0 {
                h ^= g >> 24;
                h &= !g;
            }
        }
    
        h
    }

    fn gnu_hash(&self, name: &str) -> u32 {
        let mut h: u32 = 5381;

        for c in name.as_bytes() {
            h = (h << 5)
                .wrapping_add(h)
                .wrapping_add(*c as u32);
        }
    
        h
    }

    fn bucket_count(&self) -> Result<usize> {
        self.bucket_count
            .clone()
            .read(&self.section.data())
            .and_then(|v| v
                .try_into()
                .map_err(|_| Error::ConversionError))
    }

    fn bloom_count(&self) -> Result<usize> {
        self.bloom_count
            .clone()
            .read(&self.section.data())
            .and_then(|v| v
                .try_into()
                .map_err(|_| Error::ConversionError))
    }

    fn bucket_size(&self) -> Result<usize> {
        self.bucket_count()
            .map(|v| self
                .buckets()
                .length(v))
    }

    fn bloom_size(&self) -> Result<usize> {
        self.bloom_count()
            .map(|v| self
                .blooms()
                .length(v))
    }

    fn symoffset(&self) -> Result<usize> {
        self.symoffset
            .clone()
            .read(&self.section.data())
            .and_then(|v| v
                .try_into()
                .map_err(|_| Error::ConversionError))
    }

    fn bloom_shift(&self) -> Result<usize> {
        self.bloom_shift
            .clone()
            .read(&self.section.data())
            .and_then(|v| v
                .try_into()
                .map_err(|_| Error::ConversionError))
    }

    fn blooms(&self) -> ItemArray<u32,u64> {
        self.bloom_array.clone()
    }

    fn buckets(&self) -> ItemArray<u32> {
        self.bucket_array.clone()
    }

    fn chains(&self) -> ItemArray<u32> {
        self.chain_array.clone()
    }

    fn bloom(&self, index: usize) -> Result<u64> {
        self.blooms()
            .read(&self.section.data(),index)
            .and_then(|v| v
                .try_into()
                .map_err(|_| Error::ConversionError))
    }

    fn bucket(&self, index: usize) -> Result<u32> {
        self.bloom_size()
            .and_then(|v| self
                .buckets()
                .with_last_offset(v)
                .read(&self.section.data(),index))
    }

    fn chain(&self, index: usize) -> Result<u32> {
        self.bloom_size()
            .and_then(|a| self
                .bucket_size()
                .map(|b| (a,b)))
            .and_then(|(a,b)| self
                .buckets()
                .with_last_offset(a + b)
                .read(&self.section.data(),index))
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

        let bucket_size = table.bucket_size();
        dbg!(bucket_size);

        let symoffset = table.symoffset();
        dbg!(symoffset);

        let bloom_size = table.bloom_size();
        dbg!(bloom_size);

        let bloom_shift = table.bloom_shift();
        dbg!(bloom_shift);

        dbg!(table.section.entity_size());

    }

    #[test]
    fn test_validate_hash_function() {
        let section = section!("assets/libvpf/libvpf.so.4.1", 3);

        let table = HashTable::try_from(&section).unwrap();

        // matches: https://flapenguin.me/elf-dt-hash
        assert_eq!(table.hash("printf"),0x077905a6);
        assert_eq!(table.hash("exit"),0x0006cf04);
    }

    #[test]
    fn test_validate_gnu_hash_function() {
        let section = section!("assets/libvpf/libvpf.so.4.1", 3);

        let table = HashTable::try_from(&section).unwrap();

        // https://flapenguin.me/elf-dt-gnu-hash
        assert_eq!(table.gnu_hash("printf"),0x156b2bb8);
        assert_eq!(table.gnu_hash("exit"),0x7c967e3f);
    }

    #[test]
    fn test_find_function() {
        // let path = "assets/libjpeg/libjpeg.so.9";
        // let binary = Binary::load(path).unwrap();

        // let strings = &binary.sections[36];
        // let symbols = &binary.sections[35];

        // let dynstr = StringTable::try_from(strings).unwrap();
        // let dynsym = SymbolTable::try_from(symbols).unwrap();

        // let strings = section!("assets/libjpeg/libjpeg.so.9", 36);
        // let symbols = section!("assets/libjpeg/libjpeg.so.9", 3);
        // let section = section!("assets/libjpeg/libjpeg.so.9", 2);

        // let string_table = StringTable::try_from(&strings).unwrap();
        // let symbol_table = SymbolTable::try_from(&symbols).unwrap();
        // let table = HashTable::try_from(&section).unwrap();

        // let nchain = table.nchain().unwrap();
        // dbg!(nchain);
        // dbg!(symbol_table.len());
        // dbg!(table.width());

        // table.find(&string_table,&symbol_table,"call_gmon_start");
    }
}