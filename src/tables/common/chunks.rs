
pub enum ByteDelimiter {
    Value(u8),
    Length(usize),
}

pub struct ByteIterator<'a> {
    data: &'a [u8],
    delim: ByteDelimiter,
    limit: usize,
    index: usize,
}

impl<'a> ByteIterator<'a> {

    // if you want to limit the max length, slice the data before
    // iteration
    pub fn new(data: &'a [u8], delim: ByteDelimiter) -> Self {
        Self { 
            data: data,
            delim: delim,
            limit: data.len(),
            index: 0,
        }
    }

    // pub fn new(data: &'a [u8]) -> Self {
    //     Self { 
    //         data: data,
    //         delim: ByteDelimiter::Length(0),
    //         limit: data.len(),
    //         index: 0,
    //         max: data.len(),
    //         finished: false,
    //     }
    // }

    // pub fn until_value<T: Into<u8>>(mut self, value: T) -> Self {
    //     self.delim = ByteDelimiter::Value(value.into());
    //     self
    // }

    // pub fn until_length(mut self, length: usize) -> Self {
    //     self.delim = ByteDelimiter::Length(length);
    //     self
    // }

    // pub fn with_limit(mut self, limit: usize) -> Self {
    //     self.limit = limit;
    //     self
    // }

    // pub fn finish(mut self) -> Self {
    //     let mut finished = true;

    //     if self.limit == 0 {
    //         println!("ByteIterator used with a limit of 0");
    //         finished = false;
    //     }

    //     if self.max == 0 {
    //         println!("ByteIterator given 0-length byte slice");
    //         finished = false;
    //     }

    //     if let ByteDelimiter::Length(v) = self.delim {
    //         if v == 0 {
    //             println!("ByteIterator given entity length of 0");
    //             finished = false;
    //         }
    //     }

    //     self.finished = finished;
    //     self
    // }

}

impl<'a> Iterator for ByteIterator<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;

        // early return if we're finished
        if self.index > self.limit {
            return result;
        }

        let data = &self.data[self.index..];

        match self.delim {
            ByteDelimiter::Value(v) => {
                if let Some(n) = data.iter().position(|&k| k == v) {
                    result = Some(&data[..n]);
                    self.index += n + 1;
                }
            },
            ByteDelimiter::Length(v) => {
                if data.len() >= v {
                    result = Some(&data[..v]);
                    self.index += v;
                }
            }
        }

        result
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const NULL_TERM_STR: &[u8] = &[
      b'T',b'h',b'i',b's',b' ',b'i',b's',b' ',b'a',b' ',b't',b'e',b's',b't',b'\0',
      b'A',b'n',b'o',b't',b'h',b'e',b'r',b' ',b't',b'e',b's',b't',b'\0',
    ];

    const FIXED_LEN_DATA: &[u8] = &[
      b'T',b'h',b'i',b's',b' ',b'i',b's',b' ',b'a',b' ',b't',b'e',b's',b't',
      b'T',b'h',b'i',b's',b' ',b'i',b's',b' ',b'a',b' ',b't',b'e',b's',b't',
    ];

    #[test]
    fn test_iterate_by_null_terminators() {
        let mut iter = ByteIterator::new(NULL_TERM_STR,ByteDelimiter::Value(b'\0'));
        let mut count = 0;
        for chunk in iter {
            dbg!(std::str::from_utf8(chunk));
        }
    }

    #[test]
    fn test_iterate_by_fixed_length() {
        let mut iter = ByteIterator::new(FIXED_LEN_DATA,ByteDelimiter::Length(14));
        let mut count = 0;

        for chunk in iter {
            dbg!(std::str::from_utf8(chunk));
        }
    }
}