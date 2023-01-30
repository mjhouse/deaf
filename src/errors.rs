use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T,Error>;

#[derive(ThisError, Debug)]
pub enum Error {

    #[error("Binary could not be parsed")]
    ParseError,

    #[error("Binary is not an ELF file")]
    FileTypeError,

    #[error("Failed while converting bytes to larger values")]
    ParseValueError(#[from] std::array::TryFromSliceError),

    #[error(transparent)]
    IOError(#[from] std::io::Error),

}