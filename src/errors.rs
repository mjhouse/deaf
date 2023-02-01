use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Binary could not be parsed")]
    ParseError,

    #[error("Binary is not an ELF file")]
    FileTypeError,

    #[error("Failed while converting bytes to integer values")]
    ParseValueError(#[from] std::array::TryFromSliceError),

    #[error("Failed while converting bytes to str")]
    ParseUtf8Error(#[from] std::str::Utf8Error),

    #[error("Failed while converting integer to different integer")]
    IntConvertError(#[from] std::num::TryFromIntError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
