//! A custom error type that aggregates internal errors
//!

use thiserror::Error as ThisError;

use num_enum::{TryFromPrimitiveError,TryFromPrimitive};
use enumflags2::{FromBitsError,BitFlag};

/// Type alias for 'Result' that uses our Error type
pub type Result<T> = std::result::Result<T, Error>;

/// Custom error type used everywhere in this crate
#[derive(ThisError, Debug)]
pub enum Error {

    /// A resource could not be found
    #[error("That resource or value could not be found")]
    NotFound,

    /// Binary data could not be parsed into fields
    #[error("Binary could not be parsed")]
    ParseError,

    /// Collection was accessed with an out-of-bounds index
    #[error("Slice or access is out of bounds")]
    OutOfBoundsError,

    /// Given data has missing or wrong values for the action
    #[error("Given data has the wrong shape for operation")]
    MalformedDataError,

    /// Filed to convert section to a table or array
    #[error("Given section is of the wrong type")]
    WrongSectionError,

    /// Could not parse complex type from primitive
    #[error("Could not convert from complex value")]
    FromComplexError,

    /// Could not parse complex type from primitive
    #[error("Could not convert from primitive value")]
    FromPrimitiveError(String),

    /// Failed to access shared data because Mutex is poisoned
    #[error("Mutex is poisoned and data is unavailable")]
    PoisonError(String),

    /// Bytes with no nul terminator could not be parsed as c-string
    #[error("Could not parse bytes into CStr representation")]
    FromBytesWithNulError(#[from] std::ffi::FromBytesWithNulError),

    /// String with no nul terminator could not be parsed as c-string
    #[error("Could not convert String to CString")]
    FromStringError(#[from] std::ffi::NulError),

    /// CString failed to convert to UTF-8 encoded String
    #[error("Could not convert CString to String")]
    IntoStringError(#[from] std::ffi::IntoStringError),

    /// This error will never actually be created
    #[error("This error will never actually be created")]
    InfallibleError(#[from] std::convert::Infallible),

    /// Failed to convert bytes to a value representation
    #[error("Failed while converting bytes to integer values")]
    ParseValueError(#[from] std::array::TryFromSliceError),

    /// Bytes could not be converted to UTF-8 encoded String
    #[error("Failed while converting bytes to str")]
    ParseUtf8Error(#[from] std::str::Utf8Error),

    /// Could not open a file for reading
    #[error("Could not open a file for reading")]
    IOError(#[from] std::io::Error),

    /// Could not convert integer to a different integer type
    #[error("Failed while converting integer to different integer")]
    IntConvertError(#[from] std::num::TryFromIntError),

}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(e: std::sync::PoisonError<T>) -> Self {
        Error::PoisonError(format!("std::sync::PoisonError: {}",e.to_string()))
    }
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