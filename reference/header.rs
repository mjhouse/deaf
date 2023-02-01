use crate::constants::*;
use std::ops::Range;
use crate::errors::{Error,Result};
use std::marker::PhantomData;

struct Ranges {
    a: Architecture,
    any: Range<usize>,
    x32: Range<usize>,
    x64: Range<usize>,
}

impl Ranges {
    pub fn new() -> Self {
        Self::simple(0..0)
    }
    pub fn simple(r: Range<usize>) -> Self {
        Self { 
            a: Architecture::Any, 
            any: r,
            x32: 0..0, 
            x64: 0..0
        }
    }
    pub fn arch(mut self, a: Architecture) -> Self {
        self.a = a; self
    }
    pub fn any(mut self, r: Range<usize>) -> Self {
        self.any = r; self
    }
    pub fn x32(mut self, r: Range<usize>) -> Self {
        self.x32 = r; self
    }
    pub fn x64(mut self, r: Range<usize>) -> Self {
        self.x64 = r; self
    }
    pub fn get(&self) -> Range<usize> {
        use Architecture::*;
        match self.a {
            Any => self.any.clone(),
            X32 => self.x32.clone(),
            X64 => self.x64.clone(),
        }
    }
    pub fn set(&mut self, a: Architecture) {
        self.a = a;
    }
}

/*
    let ranges = Ranges::new()
        .x32(0x02..0x04)
        .x64(0x02..0x08);
    
    ranges.set(Architecture::x32);
    let x32_range = ranges.get();

*/

enum Architecture {
    Any,
    X32,
    X64,
}

#[derive(Clone,Copy)]
enum Endianness {
    Any,
    Big,
    Little,
}


trait AsBytes {
    fn to_bytes(&self, b: &mut [u8], e: Endianness);
    fn from_bytes(b: &[u8], e: Endianness) -> Result<Self> where Self: Sized;
}

impl AsBytes for u8 {
    fn to_bytes(&self, b: &mut [u8], e: Endianness) {
        match e {
            Endianness::Big => b.copy_from_slice(&self.to_be_bytes()),
            _ => b.copy_from_slice(&self.to_le_bytes())
        };
    }
    fn from_bytes(b: &[u8], e: Endianness) -> Result<Self> {
        Ok(match e {
            Endianness::Big => Self::from_be_bytes(b.try_into()?),
            _ => Self::from_le_bytes(b.try_into()?)
        })
    }
}

impl AsBytes for u16 {
    fn to_bytes(&self, b: &mut [u8], e: Endianness) {
        match e {
            Endianness::Big => b.copy_from_slice(&self.to_be_bytes()),
            _ => b.copy_from_slice(&self.to_le_bytes())
        };
    }
    fn from_bytes(b: &[u8], e: Endianness) -> Result<Self> {
        Ok(match e {
            Endianness::Big => Self::from_be_bytes(b.try_into()?),
            _ => Self::from_le_bytes(b.try_into()?)
        })
    }
}

impl AsBytes for u32 {
    fn to_bytes(&self, b: &mut [u8], e: Endianness) {
        match e {
            Endianness::Big => b.copy_from_slice(&self.to_be_bytes()),
            _ => b.copy_from_slice(&self.to_le_bytes())
        };
    }
    fn from_bytes(b: &[u8], e: Endianness) -> Result<Self> {
        Ok(match e {
            Endianness::Big => Self::from_be_bytes(b.try_into()?),
            _ => Self::from_le_bytes(b.try_into()?)
        })
    }
}

impl AsBytes for u64 {
    fn to_bytes(&self, b: &mut [u8], e: Endianness) {
        match e {
            Endianness::Big => b.copy_from_slice(&self.to_be_bytes()),
            _ => b.copy_from_slice(&self.to_le_bytes())
        };
    }
    fn from_bytes(b: &[u8], e: Endianness) -> Result<Self> {
        Ok(match e {
            Endianness::Big => Self::from_be_bytes(b.try_into()?),
            _ => Self::from_le_bytes(b.try_into()?)
        })
    }
}

impl AsBytes for String {
    fn to_bytes(&self, b: &mut [u8], e: Endianness) {
        b.copy_from_slice(&self.as_bytes());
    }
    fn from_bytes(b: &[u8], e: Endianness) -> Result<Self> {
        Ok(std::str::from_utf8(b)?.into())
    }
}

impl AsBytes for Endianness {
    fn to_bytes(&self, b: &mut [u8], _: Endianness) {
        b.copy_from_slice(&[ 0x01 ]);
    }
    fn from_bytes(b: &[u8], e: Endianness) -> Result<Self> {
        if b.len() > 0 {
            match b[0] {
                0x01 => Ok(Endianness::Little),
                0x02 => Ok(Endianness::Big),
                _ => Err(Error::ParseError)
            }
        }
        else {
            Err(Error::ParseError)
        }
    }
}

struct Field<A,B>
where
    A: AsBytes + Into<B>,
    B: Into<A>
{
    pa: PhantomData<A>,
    pb: PhantomData<B>,
    pub e: Endianness,
    pub r: Ranges,
}

impl<A,B> Field<A,B>
where
    A: AsBytes + Into<B>,
    B: Into<A>
{
    
    pub fn new(e: Endianness, r: Ranges) -> Self {
        Self { pa: PhantomData {}, pb: PhantomData {}, e, r }
    }

    pub fn simple(r: Range<usize>) -> Self {
        Self::new(Endianness::Any,Ranges::simple(r))
    }

    pub fn get(&self, b: &[u8]) -> Result<B> {
        Ok(A::from_bytes(&b[self.r.get()],self.e)?.into())
    }

    pub fn set(&self, b: &mut [u8], v: B) {
        v.into().to_bytes(&mut b[self.r.get()],self.e);
    }

    pub fn set_arch(&mut self, a: Architecture) {
        self.r.set(a);
    }

}


#[derive(Default,Debug)]
pub struct Header {
    elf_hsize: usize,
    elf_hmagic: [u8;4],
    ei_class: u8,
    ei_data: u8,
    ei_version: u8,
    ei_osabi: u8,
    ei_abiversion: u8,
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

fn as_slice(b: &[u8], r: Range<usize>) -> &[u8] {
    &b[r]
}

fn as_u8(b: &[u8], r: Range<usize>, be: bool) -> Result<u8> {
    Ok(if be {
        u8::from_be_bytes(b[r].try_into()?)
    }
    else {
        u8::from_le_bytes(b[r].try_into()?)
    })
}

fn as_u16(b: &[u8], r: Range<usize>, be: bool) -> Result<u16> {
    Ok(if be {
        u16::from_be_bytes(b[r].try_into()?)
    }
    else {
        u16::from_le_bytes(b[r].try_into()?)
    })
}

fn as_u32(b: &[u8], r: Range<usize>, be: bool) -> Result<u32> {
    Ok(if be {
        u32::from_be_bytes(b[r].try_into()?)
    }
    else {
        u32::from_le_bytes(b[r].try_into()?)
    })
}

fn as_u64(b: &[u8], r: Range<usize>, be: bool) -> Result<u64> {
    Ok(if be {
        u64::from_be_bytes(b[r].try_into()?)
    }
    else {
        u64::from_le_bytes(b[r].try_into()?)
    })
}

impl Header {

    pub fn new(bytes: &mut [u8]) -> Result<Self> {
        let mut h = Self::default();
        h.parse_ei(bytes)?;
        h.parse_e(bytes)?;
        Ok(h)
    }

    fn parse_ei(&mut self, bytes: &mut [u8]) -> Result<()> {
        let f_elf_hmagic: Field<String,String> = Field::simple(0x01..0x04);
        let hmagic = f_elf_hmagic.get(bytes);
        dbg!(hmagic);

        f_elf_hmagic.set(bytes,"BAD".into());

        let hmagic2 = f_elf_hmagic.get(bytes);
        dbg!(hmagic2);

        self.elf_hmagic = bytes[0x00..0x04].try_into()?;

        if self.elf_hmagic != [ 0x7F, b'E', b'L', b'F' ] {
            return Err(Error::ParseError);
        }

        let f_ei_class: Field<u8,u8> = Field::simple(0x04..0x05);

        // f_ei_class.set(bytes,1);

        // f_ei_data = as_u8(bytes, 0x05..0x06, false)?;
        // f_ei_version = as_u8(bytes, 0x06..0x07, false)?;
        // f_ei_osabi = as_u8(bytes, 0x07..0x08, false)?;
        // f_ei_abiversion = as_u8(bytes, 0x08..0x09, false)?;

        self.ei_class = f_ei_class.get(bytes)?;
        self.ei_data = as_u8(bytes, 0x05..0x06, false)?;
        self.ei_version = as_u8(bytes, 0x06..0x07, false)?;
        self.ei_osabi = as_u8(bytes, 0x07..0x08, false)?;
        self.ei_abiversion = as_u8(bytes, 0x08..0x09, false)?;

        Ok(())
    }

    fn parse_e(&mut self, bytes: &[u8]) -> Result<()> {
        let big_endian = self.ei_data == 2;
        let bit_size_32 = self.ei_class == 1;

        self.e_type = as_u16(bytes, 0x10..0x12, big_endian)?;
        self.e_machine = as_u16(bytes, 0x12..0x14, big_endian)?;
        self.e_version = as_u32(bytes, 0x14..0x18, big_endian)?;


        // let f_e_entry: Field<u64> = Field::new(Endianness::Little,Ranges::new()
        //     .x32(0x18..0x1C)
        //     .x64(0x18..0x20));

        if bit_size_32 {
            self.e_entry = as_u32(bytes, 0x18..0x1C, big_endian)? as u64;
        }
        else {
            self.e_entry = as_u64(bytes, 0x18..0x20, big_endian)?;
        }

        if bit_size_32 {
            self.e_phoff = as_u32(bytes, 0x1C..0x20, big_endian)? as u64;
        }
        else {
            self.e_phoff = as_u64(bytes, 0x20..0x28, big_endian)?;
        }

        if bit_size_32 {
            self.e_shoff = as_u32(bytes, 0x20..0x24, big_endian)? as u64;
        }
        else {
            self.e_shoff = as_u64(bytes, 0x28..0x30, big_endian)?;
        }

        if bit_size_32 {
            self.e_flags = as_u32(bytes, 0x24..0x28, big_endian)?;
        }
        else {
            self.e_flags = as_u32(bytes, 0x30..0x34, big_endian)?;
        }

        if bit_size_32 {
            self.e_ehsize = as_u16(bytes, 0x28..0x2A, big_endian)?;
        }
        else {
            self.e_ehsize = as_u16(bytes, 0x34..0x36, big_endian)?;
        }

        if bit_size_32 {
            self.e_phentsize = as_u16(bytes, 0x2A..0x2C, big_endian)?;
        }
        else {
            self.e_phentsize = as_u16(bytes, 0x36..0x38, big_endian)?;
        }

        if bit_size_32 {
            self.e_phnum = as_u16(bytes, 0x2C..0x2E, big_endian)?;
        }
        else {
            self.e_phnum = as_u16(bytes, 0x38..0x3A, big_endian)?;
        }

        if bit_size_32 {
            self.e_shentsize = as_u16(bytes, 0x2E..0x30, big_endian)?;
        }
        else {
            self.e_shentsize = as_u16(bytes, 0x3A..0x3C, big_endian)?;
        }

        if bit_size_32 {
            self.e_shnum = as_u16(bytes, 0x30..0x32, big_endian)?;
        }
        else {
            self.e_shnum = as_u16(bytes, 0x3C..0x3E, big_endian)?;
        }

        if bit_size_32 {
            self.e_shstrndx = as_u16(bytes, 0x32..0x34, big_endian)?;
        }
        else {
            self.e_shstrndx = as_u16(bytes, 0x3E..0x40, big_endian)?;
        }

        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_extract_header_from_shared_library() {
        let mut f = File::open("assets/libvpf.so.4.1").unwrap();
        let mut b = Vec::new();
        f.read_to_end(&mut b).unwrap();
        let header = Header::new(&mut b);
        dbg!(header);
    }

}