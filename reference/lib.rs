use std::fs::File;
use std::io::Read;

mod constants;
mod errors;
mod parse;
mod field;

mod header;

use errors::{Error,Result};

pub struct Binary {
    buffer: Vec<u8>
}

fn is_elf(bytes: &[u8]) -> Result<bool> {
    match parse::range(bytes, 0..4)? {
        // check magic number (0x7F + ELF)
        &[ 0x7F, b'E', b'L', b'F' ] => Ok(true),
        _ => Ok(false)
    }
}

impl Binary {

    pub fn new(path: &str) -> Result<Self> {
        // open the binary file and create buffer
        let mut f = File::open(path)?;
        let mut b = Vec::new();
        
        // read bytes from the file into the buffer
        f.read_to_end(&mut b)?;

        // if the file is an elf file, return self
        match is_elf(&b) {
            Ok(true) => Ok(Self { buffer: b }),
            _        => Err(Error::FileTypeError)
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // open an actual ELF file
        let b1 = Binary::new("assets/libvpf.so.4.1");
        assert!(b1.is_ok());

        // fails to open an ELF file that's not there
        let b2 = Binary::new("assets/NONEXISTENT.so");
        assert!(b2.is_err());

        // fails to open a file that isn't an ELF file
        let b3 = Binary::new("assets/test.txt");
        assert!(b3.is_err());
    }
}
