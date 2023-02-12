use thiserror::Error as ThisError;

use num_enum::{TryFromPrimitiveError,TryFromPrimitive};
use enumflags2::{FromBitsError,BitFlag};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Binary could not be parsed")]
    ParseError,

    #[error("Binary is not an ELF file")]
    FileTypeError,

    #[error("Slice or access is out of bounds")]
    OutOfBoundsError,

    #[error("Given section is of the wrong type")]
    WrongSectionError,

    #[error("Could not convert from primitive value")]
    FromPrimitiveError(String),

    #[error("This error will never actually be created")]
    InfallibleError(#[from] std::convert::Infallible),

    #[error("Failed while converting bytes to integer values")]
    ParseValueError(#[from] std::array::TryFromSliceError),

    #[error("Failed while converting bytes to str")]
    ParseUtf8Error(#[from] std::str::Utf8Error),

    #[error("Failed while converting integer to different integer")]
    IntConvertError(#[from] std::num::TryFromIntError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}

impl<T> From<TryFromPrimitiveError<T>> for Error
where 
    T: TryFromPrimitive
{
    fn from(e: TryFromPrimitiveError<T>) -> Self {
        Error::FromPrimitiveError(format!("TryFromPrimitiveError: {}",e.to_string()))
    }
}

impl<T> From<FromBitsError<T>> for Error
where
    T: BitFlag,
    T::Numeric: core::fmt::LowerHex
{
    fn from(e: FromBitsError<T>) -> Self {
        Error::FromPrimitiveError(format!("FromBitsError: {:x}",e.invalid_bits()))
    }
}