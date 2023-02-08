use crate::headers::common::constants::*;
use crate::errors::{Error, Result};
use num_enum::{TryFromPrimitive,FromPrimitive};
use enumflags2::BitFlags;

pub trait FromBytes {
    fn from_bytes(b: &[u8], l: Layout) -> Result<Self>
    where
        Self: Sized;
}

pub trait IntoBytes {
    fn to_bytes(&self, b: &mut [u8], l: Layout) -> Result<()>;
}

// convert A into Self
pub trait AsOutput<A> {
    fn as_output(v: A) -> Result<Self> where Self: Sized;
}

// convert self into A
pub trait AsInput<A> {
    fn as_input(self) -> Result<A> where A: Sized;
}

macro_rules! from {
    ( $m: ident, $l: ident, $b: ident ) => {
        match $l {
            Layout::Big => $m::from_be_bytes($b.try_into()?),
            _ => $m::from_le_bytes($b.try_into()?),
        }
    };
}

macro_rules! into {
    ( $m: ident, $l: ident, $b: ident ) => {
        match $l {
            Layout::Big => $b.copy_from_slice(&$m.to_be_bytes()),
            _ => $b.copy_from_slice(&$m.to_le_bytes()),
        }
    };
}

impl FromBytes for u8 {
    fn from_bytes(b: &[u8], l: Layout) -> Result<Self> {
        Ok(from!(Self, l, b))
    }
}

impl IntoBytes for u8 {
    fn to_bytes(&self, b: &mut [u8], l: Layout) -> Result<()> {
        Ok(into!(self, l, b))
    }
}

impl FromBytes for u16 {
    fn from_bytes(b: &[u8], l: Layout) -> Result<Self> {
        Ok(from!(Self, l, b))
    }
}

impl IntoBytes for u16 {
    fn to_bytes(&self, b: &mut [u8], l: Layout) -> Result<()> {
        Ok(into!(self, l, b))
    }
}

impl FromBytes for u32 {
    fn from_bytes(b: &[u8], l: Layout) -> Result<Self> {
        Ok(from!(Self, l, b))
    }
}

impl IntoBytes for u32 {
    fn to_bytes(&self, b: &mut [u8], l: Layout) -> Result<()> {
        Ok(into!(self, l, b))
    }
}

impl FromBytes for u64 {
    fn from_bytes(b: &[u8], l: Layout) -> Result<Self> {
        Ok(from!(Self, l, b))
    }
}

impl IntoBytes for u64 {
    fn to_bytes(&self, b: &mut [u8], l: Layout) -> Result<()> {
        Ok(into!(self, l, b))
    }
}

impl FromBytes for String {
    fn from_bytes(b: &[u8], _: Layout) -> Result<Self> {
        Ok(std::str::from_utf8(b)?.into())
    }
}

impl IntoBytes for String {
    fn to_bytes(&self, b: &mut [u8], _: Layout) -> Result<()> {
        Ok(b.copy_from_slice(&self.as_bytes()))
    }
}

impl AsInput<String> for String {
    fn as_input(self) -> Result<Self> { Ok(self) }
}

impl AsInput<u8> for u8 {
    fn as_input(self) -> Result<Self> { Ok(self) }
}

impl AsInput<u16> for u16 {
    fn as_input(self) -> Result<Self> { Ok(self) }
}

impl AsInput<u8> for u16 {
    fn as_input(self) -> Result<u8> { Ok(self.try_into()?) }
}

impl AsInput<u32> for u32 {
    fn as_input(self) -> Result<Self> { Ok(self) }
}

impl AsInput<u32> for u64 {
    fn as_input(self) -> Result<u32> { Ok(self.try_into()?) }
}

impl AsInput<u64> for u64 {
    fn as_input(self) -> Result<Self> { Ok(self) }
}

impl AsInput<u16> for usize {
    fn as_input(self) -> Result<u16> { Ok(self.try_into()?) }
}

impl AsInput<u32> for usize {
    fn as_input(self) -> Result<u32> { Ok(self.try_into()?) }
}

impl AsInput<u64> for usize {
    fn as_input(self) -> Result<u64> { Ok(self.try_into()?) }
}

impl AsInput<u8> for Width {
    fn as_input(self) -> Result<u8> { Ok(self.into()) }
}

impl AsInput<u8> for Layout {
    fn as_input(self) -> Result<u8> { Ok(self.into()) }
}

impl AsInput<u32> for PHType {
    fn as_input(self) -> Result<u32> { Ok(self.into()) }
}

impl AsInput<u32> for SHType {
    fn as_input(self) -> Result<u32> { Ok(self.into()) }
}

impl AsInput<u32> for BitFlags<SHFlags> {
    fn as_input(self) -> Result<u32> { Ok(self.bits().try_into()?) }
}

impl AsInput<u64> for BitFlags<SHFlags> {
    fn as_input(self) -> Result<u64> { Ok(self.bits()) }
}

impl AsOutput<String> for String {
    fn as_output(a: Self) -> Result<Self> { Ok(a) }
}

impl AsOutput<u8> for u8 {
    fn as_output(a: Self) -> Result<Self> { Ok(a) }
}

impl AsOutput<u8> for u16 {
    fn as_output(a: u8) -> Result<Self> { Ok(a.try_into()?) }
}

impl AsOutput<u16> for u16 {
    fn as_output(a: Self) -> Result<Self> { Ok(a) }
}

impl AsOutput<u32> for u32 {
    fn as_output(a: Self) -> Result<Self> { Ok(a) }
}

impl AsOutput<u32> for u64 {
    fn as_output(a: u32) -> Result<Self> { Ok(a.try_into()?) }
}

impl AsOutput<u64> for u64 {
    fn as_output(a: Self) -> Result<Self> { Ok(a) }
}

impl AsOutput<u16> for usize {
    fn as_output(a: u16) -> Result<Self> { Ok(a.try_into()?) }
}

impl AsOutput<u32> for usize {
    fn as_output(a: u32) -> Result<Self> { Ok(a.try_into()?) }
}

impl AsOutput<u64> for usize {
    fn as_output(a: u64) -> Result<Self> { Ok(a.try_into()?) }
}

impl AsOutput<u8> for Width {
    fn as_output(a: u8) -> Result<Self> {
        Ok(Self::try_from_primitive(a)?)
    }
}

impl AsOutput<u8> for Layout {
    fn as_output(a: u8) -> Result<Self> {
        Ok(Self::try_from_primitive(a)?)
    }
}

impl AsOutput<u32> for PHType {
    fn as_output(a: u32) -> Result<Self> {
        Ok(Self::try_from_primitive(a)?)
    }
}

impl AsOutput<u32> for SHType {
    fn as_output(a: u32) -> Result<Self> {
        Ok(Self::from_primitive(a))
    }
}

impl AsOutput<u64> for BitFlags<SHFlags> {
    fn as_output(a: u64) -> Result<BitFlags<SHFlags>> {
        Ok(BitFlags::from_bits(a)?)
    }
}

impl AsOutput<u32> for BitFlags<SHFlags> {
    fn as_output(a: u32) -> Result<BitFlags<SHFlags>> {
        Ok(BitFlags::from_bits(a.into())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // layout doesn't matter for u8
    #[test]
    fn test_from_bytes_u8() {
        let d = &[0x02];
        let v: u8 = u8::from_bytes(d, Layout::Little).unwrap();
        assert_eq!(v, 0x02);
    }

    // layout doesn't matter for u8
    #[test]
    fn test_to_bytes_u8() {
        let v: u8 = 0x02;
        let mut b = [0; 1];
        v.to_bytes(&mut b, Layout::Little).unwrap();
        assert_eq!(&b, &[0x02]);
    }

    #[test]
    fn test_from_bytes_u16_little_endian() {
        let d = &[0x0A, 0x0B];
        let v: u16 = u16::from_bytes(d, Layout::Little).unwrap();
        assert_eq!(v, 0x0B0A);
    }

    #[test]
    fn test_to_bytes_u16_little_endian() {
        let v: u16 = 0x0B0A;
        let mut b = [0; 2];
        v.to_bytes(&mut b, Layout::Little).unwrap();
        assert_eq!(&b, &[0x0A, 0x0B]);
    }

    #[test]
    fn test_from_bytes_u16_big_endian() {
        let d = &[0x0A, 0x0B];
        let v: u16 = u16::from_bytes(d, Layout::Big).unwrap();
        assert_eq!(v, 0x0A0B);
    }

    #[test]
    fn test_to_bytes_u16_big_endian() {
        let v: u16 = 0x0A0B;
        let mut b = [0; 2];
        v.to_bytes(&mut b, Layout::Big).unwrap();
        assert_eq!(&b, &[0x0A, 0x0B]);
    }

    #[test]
    fn test_from_bytes_u32_little_endian() {
        let d = &[0x0A, 0x0B, 0x0C, 0x0D];
        let v: u32 = u32::from_bytes(d, Layout::Little).unwrap();
        assert_eq!(v, 0x0D0C0B0A);
    }

    #[test]
    fn test_to_bytes_u32_little_endian() {
        let v: u32 = 0x0D0C0B0A;
        let mut b = [0; 4];
        v.to_bytes(&mut b, Layout::Little).unwrap();
        assert_eq!(&b, &[0x0A, 0x0B, 0x0C, 0x0D]);
    }

    #[test]
    fn test_from_bytes_u32_big_endian() {
        let d = &[0x0A, 0x0B, 0x0C, 0x0D];
        let v: u32 = u32::from_bytes(d, Layout::Big).unwrap();
        assert_eq!(v, 0x0A0B0C0D);
    }

    #[test]
    fn test_to_bytes_u32_big_endian() {
        let v: u32 = 0x0A0B0C0D;
        let mut b = [0; 4];
        v.to_bytes(&mut b, Layout::Big).unwrap();
        assert_eq!(&b, &[0x0A, 0x0B, 0x0C, 0x0D]);
    }

    #[test]
    fn test_from_bytes_u64_little_endian() {
        let d = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let v: u64 = u64::from_bytes(d, Layout::Little).unwrap();
        assert_eq!(v, 0x0807060504030201);
    }

    #[test]
    fn test_to_bytes_u64_little_endian() {
        let v: u64 = 0x0807060504030201;
        let mut b = [0; 8];
        v.to_bytes(&mut b, Layout::Little).unwrap();
        assert_eq!(&b, &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
    }

    #[test]
    fn test_from_bytes_u64_big_endian() {
        let d = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        let v: u64 = u64::from_bytes(d, Layout::Big).unwrap();
        assert_eq!(v, 0x0102030405060708);
    }

    #[test]
    fn test_to_bytes_u64_big_endian() {
        let v: u64 = 0x0102030405060708;
        let mut b = [0; 8];
        v.to_bytes(&mut b, Layout::Big).unwrap();
        assert_eq!(&b, &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
    }

    // layout doesn't matter for string
    #[test]
    fn test_from_bytes_string() {
        let d = &[b'E', b'L', b'F'];
        let v: String = String::from_bytes(d, Layout::Little).unwrap();
        assert_eq!(v, "ELF".to_string());
    }
}
