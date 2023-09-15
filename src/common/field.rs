use std::marker::PhantomData;

use crate::common::{FromBytes, IntoBytes, Convert, Layout, Width, Ranges};
use crate::errors::{Error, Result};

/// A single field in a section, table item etc.
///
/// Internally maintains ranges for various widths (32- or 64-bit)
/// and layout (little- or big-endian) and allows bytes to be read 
/// as values or values to be written to a byte buffer.
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
    ranges: Ranges,
    layout: Layout,
}

impl<T32, T64, Out> Field<T32, T64, Out>
where
    T32: FromBytes + IntoBytes + Convert<Out>,
    T64: FromBytes + IntoBytes + Convert<Out>,
    Out: Convert<T32> + Convert<T64> + std::fmt::Debug,
{
    /// Create a new field from given ranges
    pub const fn new(ranges: Ranges) -> Self {
        Self {
            a: PhantomData {},
            b: PhantomData {},
            c: PhantomData {},
            ranges,
            layout: Layout::Little,
        }
    }

    /// Create a new field with an empty range
    pub const fn empty() -> Self {
        Self {
            a: PhantomData {},
            b: PhantomData {},
            c: PhantomData {},
            ranges: Ranges::empty(),
            layout: Layout::Little,
        }
    }

    /// Builder method to set the width of the field
    pub const fn with_width(mut self, width: Width) -> Self {
        self.ranges.width = width;
        self
    }

    /// Builder method to set the layout of the field
    pub const fn with_layout(mut self, layout: Layout) -> Self {
        self.layout = layout;
        self
    }

    /// Get the width of the field
    pub fn width(&self) -> Width {
        self.ranges.width
    }

    /// Set the width of the field
    pub fn set_width(&mut self, width: Width) {
        self.ranges.width = width;
    }

    /// Get the layout of the field
    pub fn layout(&self) -> Layout {
        self.layout
    }

    /// Set the layout of the field
    pub fn set_layout(&mut self, layout: Layout) {
        self.layout = layout;
    }

    /// Set an offset in bytes to read at
    pub fn set_offset(&mut self, offset: usize) {
        let range = self.ranges.at_mut();
        range.start += offset;
        range.end   += offset;
    }

    /// Set an index based on field size to read at
    pub fn set_index(&mut self, index: usize) {
        let size  = self.size();
        let range = self.ranges.at_mut();
        range.start += index * size;
        range.end   += index * size;
    }

    /// Get a constrained slice of bytes using the appropriate range
    ///
    /// If the slice is too short, this method will fail, otherwise
    /// it will just reduce the slice to fit the data.
    fn slice<'a>(&self, bytes: &'a [u8]) -> Result<&'a [u8]> {
        let range = self.ranges.get();
        if range.end > bytes.len() {
            Err(Error::OutOfBoundsError)
        } else {
            Ok(&bytes[range])
        }
    }

    /// Get a constrained slice of mutable bytes using the appropriate range
    ///
    /// If the slice is too short, this method will fail, otherwise
    /// it will just reduce the slice to fit the data.
    fn slice_mut<'a>(&self, bytes: &'a mut [u8]) -> Result<&'a mut [u8]> {
        let range = self.ranges.get();
        if range.end > bytes.len() {
            Err(Error::OutOfBoundsError)
        } else {
            Ok(&mut bytes[range])
        }
    }

    /// Read the buffer as the output value with a 32-bit width
    fn get_x32(&self, bytes: &[u8]) -> Result<Out> {
        let bytes = self.slice(bytes)?;
        let layout = self.layout();
        T32::from_bytes(bytes, layout)
            .and_then(|v| v.convert())
    }

    /// Read the buffer as the output value with a 64-bit width
    fn get_x64(&self, bytes: &[u8]) -> Result<Out> {
        let bytes = self.slice(bytes)?;
        let layout = self.layout();
        T64::from_bytes(bytes, layout)
            .and_then(|v| v.convert())
    }

    /// Write the output value to the buffer with a 32-bit width
    fn set_x32(&self, bytes: &mut [u8], v: Out) -> Result<()> {
        let bytes = self.slice_mut(bytes)?;
        let layout = self.layout();
        <Out as Convert<T32>>::convert(v)?.to_bytes(bytes,layout)
    }

    /// Write the output value to the buffer with a 64-bit width
    fn set_x64(&self, bytes: &mut [u8], v: Out) -> Result<()> {
        let bytes = self.slice_mut(bytes)?;
        let layout = self.layout();
        <Out as Convert<T64>>::convert(v)?.to_bytes(bytes,layout)
    }

    /// Read the buffer and convert into the output value
    pub fn get(&self, bytes: &[u8]) -> Result<Out> {
        Ok(match self.width() {
            Width::X32 => self.get_x32(bytes)?,
            Width::X64 => self.get_x64(bytes)?,
        })
    }

    /// Convert output value and write to the buffer
    pub fn set(&self, bytes: &mut [u8], value: Out) -> Result<()> {
        Ok(match self.width() {
            Width::X32 => self.set_x32(bytes,value)?,
            Width::X64 => self.set_x64(bytes,value)?,
        })
    }

    /// The expected size in bytes of the output/input buffers
    pub fn size(&self) -> usize {
        self.ranges.size()
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
    fn test_simple_set_failure() {
        let bytes = &mut [ 0x0A, 0x09, 0x08, 0x07, 0x06 ];
        let field: Field<u64> = Field::new(RANGES_U64);

        let result = field.set(bytes,0x0908070605040302);
        assert!(result.is_err());
    }

    #[test]
    fn test_simple_get_failure() {
        let bytes = &mut [ 0x0A, 0x09, 0x08, 0x07, 0x06 ];
        let field: Field<u64> = Field::new(RANGES_U64);

        let result = field.get(bytes);
        assert!(result.is_err());
    }

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

