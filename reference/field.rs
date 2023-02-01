use std::ops::Range;
use std::marker::PhantomData;

use crate::constants::FieldValue;
use crate::errors::{Error,Result};

pub struct Field<T, const S: usize, const E: usize>
where 
    T: FieldValue
{
    range: Range<usize>,
    phantom: PhantomData<T>
}

macro_rules! apply {
    ( $s: ident, $b: ident, $e: block ) => {
        if $s.check($b) {
            Ok($e)
        } else {
            Err(Error::ParseError)
        }
    }
}

impl<T, const S: usize, const E: usize> Field<T, S, E>
where 
    T: FieldValue
{
    pub fn new() -> Self {
        Self { 
            range: Range { start: S, end: E }, 
            phantom: PhantomData
        }
    }

    pub fn slice<'a>(&self, bytes: &'a [u8]) -> &'a [u8] {
        &bytes[self.range.clone()]
    }

    pub fn slice_mut<'a>(&self, bytes: &'a mut [u8]) -> &'a mut [u8] {
        &mut bytes[self.range.clone()]
    }

    // check that the provided range is inside the slice
    pub fn check(&self, bytes: &[u8]) -> bool {
        bytes.len() >= self.range.end
    }

    // reads from the binary and returns an instance of `T`
    pub fn read(&self, bytes: &[u8]) -> Result<T> {
        apply!(self, bytes, {
            T::from_slice(self.slice(bytes))
        })
    }

    // writes to the binary and returns bytes changed
    pub fn write(&self, bytes: &mut [u8], value: T) -> Result<()> {
        apply!(self, bytes, {
            self.slice_mut(bytes)
                .copy_from_slice(value
                    .into_slice());
        })
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constants::*;

    #[test]
    fn test_read_from_binary_value() {
        let bytes = [ 0x7F, b'E', b'L', b'F' ];
        let field = FileMagicField::new();
        let value = field.read(&bytes).unwrap();
        assert_eq!(value,FileMagic::Valid);
    }

    #[test]
    fn test_read_from_binary_none() {
        let bytes = [ 0x1, 0x2, 0x3, 0x4 ];
        let field = FileMagicField::new();
        let value = field.read(&bytes).unwrap();
        assert!(value.is_unknown());
    }

    #[test]
    fn test_read_from_binary_out_of_range() {
        let bytes = [ 0x1, 0x2, 0x3 ]; // too short
        let field = FileMagicField::new();
        let value = field.read(&bytes);
        assert!(value.is_err());
    }

    #[test]
    fn test_write_to_binary_value() {
        let mut bytes = [ 0x1, 0x2, 0x3, 0x4, 0x0 ];
        let field = FileClassField::new();
        let result = field.write(&mut bytes, FileClass::X32Bit);
        assert!(result.is_ok());
        assert_eq!(bytes, [ 0x1, 0x2, 0x3, 0x4, 0x1 ]);
    }

    #[test]
    fn test_write_to_binary_none() {
        let mut bytes = [ 0x1, 0x2, 0x3, 0x4, 0x0 ];
        let field = FileClassField::new();
        let result = field.write(&mut bytes, FileClass::X64Bit);
        assert!(result.is_ok());
        assert_eq!(bytes, [ 0x1, 0x2, 0x3, 0x4, 0x2 ]);
    }

    #[test]
    fn test_write_to_binary_out_of_range() {
        let mut bytes = [ 0x1, 0x2, 0x3, 0x4 ];
        let field = FileClassField::new();
        let result = field.write(&mut bytes, FileClass::X64Bit);
        assert!(result.is_err());
        assert_eq!(bytes, [ 0x1, 0x2, 0x3, 0x4 ]);
    }
}