use std::marker::PhantomData;
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::headers::common::bytes::{FromBytes, IntoBytes, Convert};
use crate::headers::common::constants::{Layout, Width};
use crate::errors::{Error, Result};
use crate::headers::common::ranges::Ranges;

#[derive(Debug,Clone)]
pub struct Field<T32 = u8, T64 = T32, Out = T64>
where
    T32: FromBytes + IntoBytes + Convert<Out>,
    T64: FromBytes + IntoBytes + Convert<Out>,
    Out: Convert<T32> + Convert<T64> + std::fmt::Debug,
{
    a: PhantomData<T32>,
    b: PhantomData<T64>,
    c: PhantomData<Out>,
    pub ranges: Ranges,
    pub layout: Layout,
}

impl<T32, T64, Out> Field<T32, T64, Out>
where
    T32: FromBytes + IntoBytes + Convert<Out>,
    T64: FromBytes + IntoBytes + Convert<Out>,
    Out: Convert<T32> + Convert<T64> + std::fmt::Debug,
{
    pub const fn new(ranges: Ranges) -> Self {
        Self {
            a: PhantomData {},
            b: PhantomData {},
            c: PhantomData {},
            ranges,
            layout: Layout::Little,
        }
    }

    pub fn get_x32(&self, b: &[u8]) -> Result<Out> {
        let bytes = &b[self.ranges.get()];
        let layout = self.layout.clone();
        T32::from_bytes(bytes, layout)
            .and_then(|v| v.convert())
    }

    pub fn get_x64(&self, b: &[u8]) -> Result<Out> {
        let bytes = &b[self.ranges.get()];
        let layout = self.layout.clone();
        T64::from_bytes(bytes, layout)
            .and_then(|v| v.convert())
    }

    pub fn set_x32(&self, b: &mut [u8], v: Out) -> Result<()> {
        let bytes = &mut b[self.ranges.get()];
        let layout = self.layout.clone();
        <Out as Convert<T32>>::convert(v)?.to_bytes(bytes,layout)
    }

    pub fn set_x64(&self, b: &mut [u8], v: Out) -> Result<()> {
        let bytes = &mut b[self.ranges.get()];
        let layout = self.layout.clone();
        <Out as Convert<T64>>::convert(v)?.to_bytes(bytes,layout)
    }

    pub fn get(&self, b: &[u8]) -> Result<Out> {
        Ok(match self.ranges.width {
            Width::X32 => self.get_x32(b)?,
            Width::X64 => self.get_x64(b)?,
        })
    }

    pub fn set(&self, b: &mut [u8], v: Out) -> Result<()> {
        Ok(match self.ranges.width {
            Width::X32 => self.set_x32(b,v)?,
            Width::X64 => self.set_x64(b,v)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub const RANGES_STR: Ranges = Ranges::new(0x01..0x04, 0x01..0x04);
    pub const RANGES_U8:  Ranges = Ranges::new(0x01..0x02, 0x01..0x02);
    pub const RANGES_U16: Ranges = Ranges::new(0x01..0x03, 0x01..0x03);
    pub const RANGES_U32: Ranges = Ranges::new(0x01..0x05, 0x01..0x05);
    pub const RANGES_U64: Ranges = Ranges::new(0x01..0x09, 0x01..0x09);

    pub const RANGES_COMPLEX_U8_U16: Ranges = Ranges::new(0x01..0x02, 0x01..0x03);

    #[test]
    fn test_simple_field_u8_get() {
        let bytes = &[ 0x01, 0x02, 0x03, 0x04, 0x05, 0x06 ];
        let field: Field<u8> = Field::new(RANGES_U8);

        let result = field.get(bytes);
        assert!(result.is_ok());

        let value = result.unwrap();
        assert_eq!(value, 0x02);
    }

    #[test]
    fn test_simple_field_u8_set() {
        let bytes = &mut [ 0x01, 0x02, 0x03, 0x04, 0x05, 0x06 ];
        let field: Field<u8> = Field::new(RANGES_U8);

        let result = field.set(bytes,0xFF);
        assert!(result.is_ok());
        assert_eq!(bytes[1], 0xFF);
    }

    #[test]
    fn test_simple_field_u16_get() {
        let bytes = &[ 0x01, 0x02, 0x03, 0x04, 0x05, 0x06 ];
        let field: Field<u16> = Field::new(RANGES_U16);

        let result = field.get(bytes);
        assert!(result.is_ok());

        let value = result.unwrap();
        assert_eq!(value, 0x0302);
    }

    #[test]
    fn test_simple_field_u16_set() {
        let bytes = &mut [ 0x06, 0x05, 0x04, 0x03, 0x02, 0x01 ];
        let field: Field<u16> = Field::new(RANGES_U16);

        let result = field.set(bytes,0x0302);
        assert!(result.is_ok());

        assert_eq!(bytes[1], 0x02);
        assert_eq!(bytes[2], 0x03);
    }

    #[test]
    fn test_simple_field_u32_get() {
        let bytes = &[ 0x01, 0x02, 0x03, 0x04, 0x05, 0x06 ];
        let field: Field<u32> = Field::new(RANGES_U32);

        let result = field.get(bytes);
        assert!(result.is_ok());

        let value = result.unwrap();
        assert_eq!(value, 0x05040302);
    }

    #[test]
    fn test_simple_field_u32_set() {
        let bytes = &mut [ 0x06, 0x05, 0x04, 0x03, 0x02, 0x01 ];
        let field: Field<u32> = Field::new(RANGES_U32);

        let result = field.set(bytes,0x05040302);
        assert!(result.is_ok());

        assert_eq!(bytes[1..5], [ 0x02, 0x03, 0x04, 0x05 ]);
    }

    #[test]
    fn test_simple_field_u64_get() {
        let bytes = &[ 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A ];
        let field: Field<u64> = Field::new(RANGES_U64);

        let result = field.get(bytes);
        assert!(result.is_ok());

        let value = result.unwrap();
        assert_eq!(value, 0x0908070605040302);
    }

    #[test]
    fn test_simple_field_u64_set() {
        let bytes = &mut [ 0x0A, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01 ];
        let field: Field<u64> = Field::new(RANGES_U64);

        let result = field.set(bytes,0x0908070605040302);
        assert!(result.is_ok());

        assert_eq!(bytes[1..9], [ 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09 ]);
    }

    #[test]
    fn test_simple_field_string_get() {
        let bytes = &[ 0x00, b'E', b'L', b'F', 0x06 ];
        let field: Field<String> = Field::new(RANGES_STR);

        let result = field.get(bytes);
        assert!(result.is_ok());

        let value = result.unwrap();
        assert_eq!(value, "ELF".to_string());
    }

    #[test]
    fn test_simple_field_string_set() {
        let bytes = &mut [ 0x00, b'E', b'L', b'F', 0x06 ];
        let field: Field<String> = Field::new(RANGES_STR);

        let result = field.set(bytes,"BAD".to_string());
        assert!(result.is_ok());

        let value = field.get(bytes).unwrap();
        assert_eq!(value, "BAD".to_string());
    }

    #[test]
    fn test_complex_field_u8_u16_get() {
        let bytes = &[ 0x01, 0x02, 0x03, 0x04, 0x05, 0x06 ];

        let mut field: Field<u8,u16> = Field::new(
            RANGES_COMPLEX_U8_U16);

        // check at 32-bit width
        field.ranges.width = Width::X32;
        let value = field.get(bytes).unwrap();
        assert_eq!(value, 0x02);

        // check at 64-bit width
        field.ranges.width = Width::X64;
        let value = field.get(bytes).unwrap();
        assert_eq!(value, 0x0302);
    }

    #[test]
    fn test_complex_field_u8_u16_set() {
        let bytes = &mut [ 0x01, 0x02, 0x03, 0x04, 0x05, 0x06 ];

        let mut field: Field<u8,u16> = Field::new(
            RANGES_COMPLEX_U8_U16);

        // check at 32-bit width
        field.ranges.width = Width::X32;
        assert!(field.set(bytes,0x09).is_ok());
        assert_eq!(bytes[1..3], [ 0x09, 0x03 ]);

        // check at 64-bit width
        field.ranges.width = Width::X64;
        assert!(field.set(bytes,0x0903).is_ok());
        assert_eq!(bytes[1..3], [ 0x03, 0x09 ]);
    }

    #[test]
    fn test_complex_field_width_get() {
        let mut bytes = [ 0x01, 0x02, 0x03, 0x04, 0x05, 0x06 ];

        let field: Field<u8,u8, Width> = Field::new(
            RANGES_U8);

        // check at 64-bit width
        let value = field.get(&bytes).unwrap();
        assert_eq!(value, Width::X64);

        bytes[1] = 0x01;

        // check at 32-bit width
        let value = field.get(&bytes).unwrap();
        assert_eq!(value, Width::X32);
    }

    #[test]
    fn test_complex_field_width_set() {
        let mut bytes = [ 0x01, 0x00, 0x03, 0x04, 0x05, 0x06 ];

        let field: Field<u8,u8, Width> = Field::new(
            RANGES_U8);

        // check at 64-bit width
        assert!(field.set(&mut bytes, Width::X64).is_ok());
        assert_eq!(bytes[1],0x02);

        // check at 64-bit width
        assert!(field.set(&mut bytes, Width::X32).is_ok());
        assert_eq!(bytes[1],0x01);
    }

}

