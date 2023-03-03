use crate::headers::common::constants::*;
use crate::errors::{Error, Result};
use num_enum::{TryFromPrimitive,FromPrimitive};
use enumflags2::BitFlags;

/// Parse an object from bytes
///
/// # Arguments
///
/// * `b` - A slice of bytes to parse
/// * `l` - The endianness of the bytes
///
/// # Examples
///
/// ```
/// # use deaf::common::bytes::FromBytes;
/// # use deaf::headers::common::constants::*;
///
/// // layout doesn't matter for strings
/// let layout = Layout::Little;
/// let bytes = &[ b'E', b'L', b'F' ];
///
/// let string = String::from_bytes(
///     bytes,
///     layout
/// ).unwrap();
///
/// assert_eq!(string,"ELF");
/// ```
pub trait FromBytes {
    fn from_bytes(b: &[u8], l: Layout) -> Result<Self>
    where
        Self: Sized;
}

/// Convert an object into bytes
///
/// # Arguments
///
/// * `b` - A slice of bytes to write to
/// * `l` - The endianness to use during write
///
/// # Examples
///
/// ```
/// # use deaf::common::bytes::IntoBytes;
/// # use deaf::headers::common::constants::*;
///
/// // layout doesn't matter for strings
/// let layout = Layout::Little;
/// let expected = &[ b'E', b'L', b'F' ];
/// let found = &mut [ 0x00, 0x00, 0x00 ];
///
/// let string = String::from("ELF");
///
/// string.to_bytes(
///     found,
///     layout
/// ).unwrap();
///
/// assert_eq!(found,expected);
/// ```
pub trait IntoBytes {
    fn to_bytes(&self, b: &mut [u8], l: Layout) -> Result<()>;
}

/// Convert a value into another value
///
/// # Examples
///
/// ```
/// # use deaf::common::bytes::Convert;
/// # use deaf::headers::common::constants::*;
/// # use deaf::errors::Result;
///
/// let expected = Width::X64;
/// let found: Result<Width> = 2_u8.convert();
///
/// assert_eq!(found.unwrap(),expected);
/// ```
pub trait Convert<A> {
    fn convert(self) -> Result<A> where A: Sized;
}

/// Blanket implementation for NOP conversions to self
impl<A> Convert<A> for A {
    fn convert(self) -> Result<Self> { Ok(self) }
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

impl FromBytes for [u8;7] {
    fn from_bytes(b: &[u8], _: Layout) -> Result<Self> {
        Ok(b.try_into()?)
    }
}

impl IntoBytes for [u8;7] {
    fn to_bytes(&self, b: &mut [u8], _: Layout) -> Result<()> {
        Ok(b.copy_from_slice(self))
    }
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

impl FromBytes for i32 {
    fn from_bytes(b: &[u8], l: Layout) -> Result<Self> {
        Ok(from!(Self, l, b))
    }
}

impl IntoBytes for i32 {
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

impl FromBytes for i64 {
    fn from_bytes(b: &[u8], l: Layout) -> Result<Self> {
        Ok(from!(Self, l, b))
    }
}

impl IntoBytes for i64 {
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

impl Convert<u8> for u16 {
    fn convert(self) -> Result<u8> { Ok(self.try_into()?) }
}

impl Convert<u32> for u64 {
    fn convert(self) -> Result<u32> { Ok(self.try_into()?) }
}

impl Convert<u16> for usize {
    fn convert(self) -> Result<u16> { Ok(self.try_into()?) }
}

impl Convert<u32> for usize {
    fn convert(self) -> Result<u32> { Ok(self.try_into()?) }
}

impl Convert<u64> for usize {
    fn convert(self) -> Result<u64> { Ok(self.try_into()?) }
}

impl Convert<u8> for Width {
    fn convert(self) -> Result<u8> { Ok(self.try_into()?) }
}

impl Convert<u8> for Layout {
    fn convert(self) -> Result<u8> { Ok(self.try_into()?) }
}

impl Convert<u32> for PHType {
    fn convert(self) -> Result<u32> { Ok(self.try_into()?) }
}

impl Convert<u32> for SHType {
    fn convert(self) -> Result<u32> { Ok(self.try_into()?) }
}

impl Convert<u32> for BitFlags<SHFlags> {
    fn convert(self) -> Result<u32> { Ok(self.bits().try_into()?) }
}

impl Convert<u64> for BitFlags<SHFlags> {
    fn convert(self) -> Result<u64> { Ok(self.bits().try_into()?) }
}

impl Convert<u16> for u8 {
    fn convert(self) -> Result<u16> { Ok(self.try_into()?) }
}

impl Convert<u64> for u32 {
    fn convert(self) -> Result<u64> { Ok(self.try_into()?) }
}

impl Convert<usize> for u16 {
    fn convert(self) -> Result<usize> { Ok(self.try_into()?) }
}

impl Convert<usize> for u32 {
    fn convert(self) -> Result<usize> { Ok(self.try_into()?) }
}

impl Convert<usize> for u64 {
    fn convert(self) -> Result<usize> { Ok(self.try_into()?) }
}

impl Convert<i64> for i32 {
    fn convert(self) -> Result<i64> { Ok(self.try_into()?) }
}

impl Convert<i32> for i64 {
    fn convert(self) -> Result<i32> { Ok(self.try_into()?) }
}

impl Convert<Width> for u8 {
    fn convert(self) -> Result<Width> { 
        Ok(Width::try_from_primitive(self)?)
    }
}

impl Convert<Layout> for u8 {
    fn convert(self) -> Result<Layout> { 
        Ok(Layout::try_from_primitive(self)?)
    }
}

impl Convert<PHType> for u32 {
    fn convert(self) -> Result<PHType> { 
        Ok(PHType::try_from_primitive(self)?)
    }
}

impl Convert<SHType> for u32 {
    fn convert(self) -> Result<SHType> { 
        Ok(SHType::try_from_primitive(self)?)
    }
}

impl Convert<BitFlags<SHFlags>> for u64 {
    fn convert(self) -> Result<BitFlags<SHFlags>> { 
        Ok(BitFlags::from_bits(self)?)
    }
}

impl Convert<BitFlags<SHFlags>> for u32 {
    fn convert(self) -> Result<BitFlags<SHFlags>> { 
        Ok(BitFlags::from_bits(self.try_into()?)?)
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
    fn test_to_bytes_i64_little_endian_fitting() {
        let v: i64 = 0x0807060504030201;
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
