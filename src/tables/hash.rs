use crate::common::ranges::{ADDRESS, NBUCKETS, SYMOFFSET, BLOOMSIZE, BLOOMSHIFT, VALUE4};
use crate::errors::{Error,Result};
use crate::common::{ItemArray,SHType,Layout,Width,Item,Convert};
use crate::tables::{SymbolTable,StringTable,TableView};
use crate::Section;



/// A Section represented as an immutable HashTable
pub struct HashTable<'a> {
    bucket_count: Item<u32>,
    symoffset: Item<u32>,
    bloom_count: Item<u32>,
    bloom_shift: Item<u32>,
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
            bucket_count: Item::make(NBUCKETS,w,l),
            symoffset: Item::make(SYMOFFSET,w,l),
            bloom_count: Item::make(BLOOMSIZE,w,l),
            bloom_shift: Item::make(BLOOMSHIFT,w,l),
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

    fn bucket_count<T>(&self) -> Result<T>
    where 
        u32: Convert<T>
    {
        self.bucket_count
            .clone()
            .read(&self.section.data())
            .and_then(|v| v
                .convert())
    }

    fn bloom_count<T>(&self) -> Result<T>
    where 
        u32: Convert<T>
    {
        self.bloom_count
            .clone()
            .read(&self.section.data())
            .and_then(|v| v
                .convert())
    }

    fn bucket_size(&self) -> Result<usize> {
        self.bucket_count()
            .map(|v| self
                .buckets()
                .length(v))
    }

    fn bloom_size<T>(&self) -> Result<T>
    where 
        usize: Convert<T>
    {
        self.bloom_count()
            .map(|v| self
                .blooms()
                .length(v))
            .and_then(|v| v
                .convert())
    }

    fn symoffset<T>(&self) -> Result<T>
    where 
        u32: Convert<T>
    {
        self.symoffset
            .clone()
            .read(&self.section.data())
            .and_then(|v| v
                .convert())
    }

    fn bloom_shift<T>(&self) -> Result<T>
    where 
        u32: Convert<T>
    {
        self.bloom_shift
            .clone()
            .read(&self.section.data())
            .and_then(|v| v
                .convert())
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

    fn bloom<T,V>(&self, index: V) -> Result<T> 
    where 
        u64: Convert<T>,
        V: Convert<usize>
    {
        self.blooms()
            .read(
                self.data(),
                index.convert()?)
            .and_then(|v| v
                .convert())
    }

    fn bucket<T,V>(&self, index: V) -> Result<T>
    where 
        u32: Convert<T>,
        V: Convert<usize>
    {
        self.bloom_size()
            .and_then(|v| self
                .buckets()
                .with_last_offset(v)
                .read(
                    self.data(),
                    index.convert()?))
            .and_then(|v| v
                .convert())
    }

    fn chain<T,V>(&self, index: V) -> Result<T>
    where 
        u32: Convert<T>,
        V: Convert<usize>
    {
        self.bloom_size()
            .and_then(|a: usize| self
                .bucket_size()
                .map(|b| a + b))
            .and_then(|v| self
                .chains()
                .with_last_offset(v)
                .read(
                    self.data(),
                    index.convert()?))
            .and_then(|v| v
                .convert())
    }

    fn in_bloom_filter(&self, name: &str) -> Result<bool> {
        // https://blogs.oracle.com/solaris/post/gnu-hash-elf-sections

        let c = match self.width() {
            Width::X32 => 32,
            Width::X64 => 64
        };

        let size:  u32 = self.bloom_size()?;
        let shift: u32 = self.bloom_shift()?;

        let h1: u32 = self.gnu_hash(name);
        let h2: u32 = h1 >> shift;

        let n1 = (h1 / c) % size;
        let n2 = (h2 / c) % size;

        let b1 = h1 % c;
        let b2 = h2 % c;

        let m: u64 = (1 << b1) | (1 << b2);
        let v: u64 = self.bloom(n1)?;

        Ok(v & m == m)
    }

    fn find(&self, symbols: &SymbolTable, strings: &StringTable, name: &str) -> Result<usize> {
        // 64: parse_get_number

        if let Ok(false) = self.in_bloom_filter(name) {
            return Err(Error::NotFound);
        };

        let nbuckets: u32 = self.bucket_count()?;
        let symoffset: u32 = self.symoffset()?;
        
        let namehash = self.gnu_hash(name);
        let mut index = namehash % nbuckets; 

        if index < symoffset {
            dbg!("index is less than symoffset");
            return Err(Error::NotFound);
        }

        let mut count = 0;

        loop {
            let hash: u32 = self.chain(index - symoffset)?;
            dbg!(index);
            dbg!(symbols.len());
            dbg!(hash);
            dbg!(name);

            let item = symbols
                .at(index as usize)
                .map(|i| i.name())
                .unwrap_or(0);

            let found = strings
                .at_offset(item as usize)
                .map(|v| v.string_lossy())
                .unwrap_or(String::new());

            dbg!(&found);

            if (namehash | 1) == (hash | 1) && found == name {
                dbg!(found);
                break;
            }

            if hash & 1 == 1 {
                dbg!(count);
                break;
            }

            index += 1;
            count += 1;
        }

        Ok(0)
    }

    fn layout(&self) -> Layout {
        self.section.layout()
    }

    fn width(&self) -> Width {
        self.section.width()
    }

    fn data(&self) -> &[u8] {
        self.section.data()
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
    use crate::tables::{StringTable,SymbolTable,TableView};

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
        let path = "assets/libvpf/libvpf.so.4.1";

        let hash_section   = section!("assets/libvpf/libvpf.so.4.1", 3);
        let symbol_section = section!("assets/libvpf/libvpf.so.4.1", 4);
        let string_section = section!("assets/libvpf/libvpf.so.4.1", 5);

        let hash = HashTable::try_from(&hash_section).unwrap();
        let symbols = SymbolTable::try_from(&symbol_section).unwrap();
        let strings = StringTable::try_from(&string_section).unwrap();

        let bucket_size = hash.bucket_size();
        // dbg!(bucket_size);

        let symoffset = hash.symoffset::<u32>();
        dbg!(symoffset);

        let bloom_size = hash.bloom_size::<u32>();
        // dbg!(bloom_size);

        let bloom_shift = hash.bloom_shift::<u32>();
        // dbg!(bloom_shift);

        hash.find(&symbols,&strings,"parse_get_number");
        
        // for (i,item) in symbols.items().unwrap().into_iter().enumerate() {
        //     let result = strings
        //         .at_offset(item.name() as usize)
        //         .map(|v| v.string_lossy());

        //     if let Ok(name) = result {
        //         let check = hash
        //             .in_bloom_filter(&name)
        //             .unwrap_or(false);
        //         println!("{} {}: {:?}",
        //             i,
        //             check,
        //             name);
        //     }
        // }

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
    fn test_in_bloom_filter() {
        let section = section!("assets/libvpf/libvpf.so.4.1", 3);

        let table = HashTable::try_from(&section).unwrap();

        let check = table.in_bloom_filter("NAME");
        assert!(check.is_ok());

        let value = check.unwrap();
        assert!(value);
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