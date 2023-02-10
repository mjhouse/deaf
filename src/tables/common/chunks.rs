
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

    fn take_slice(&self) -> Option<&'a [u8]> {
        if self.index <= self.limit {
            Some(&self.data[self.index..])
        }
        else {
            None
        }
    }

    fn find_value(&mut self, value: u8) -> Option<usize> {
        self.take_slice()
            .and_then(|d| d
                .iter()
                .position(|&i| i == value)
                .map(|i| i + 1))
    }

    fn find_length(&mut self, value: usize) -> Option<usize> {
        self.take_slice()
            .and_then(|d| if d.len() >= value {
                Some(value)
            } else {
                None
            })
    }

    fn take_value(&mut self, value: u8) -> Option<&'a [u8]> {
        self.find_value(value)
            .and_then(|i| self
                .take_slice()
                .map(|d| &d[..i]))
    }

    fn take_length(&mut self, length: usize) -> Option<&'a [u8]> {
        self.find_length(length)
            .and_then(|i| self
                .take_slice()
                .map(|d| &d[..i]))
    }

}

impl<'a> Iterator for ByteIterator<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.delim {
            ByteDelimiter::Value(v) => self.take_value(v),  // self.index += end + 1;
            ByteDelimiter::Length(v) => self.take_length(v),// self.index += length;
        };

        if let Some(d) = result {
            self.index += d.len();
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
    fn test_byte_iter_take_slice() {
        let mut iter = ByteIterator::new(NULL_TERM_STR,ByteDelimiter::Value(b'\0'));
        assert!(iter.take_slice().is_some());

        // returns whole slcie because the iter index defaults to 0
        let result = iter.take_slice().unwrap();
        assert_eq!(result,NULL_TERM_STR);
    }

    #[test]
    fn test_byte_iter_find_value() {
        let mut iter = ByteIterator::new(NULL_TERM_STR,ByteDelimiter::Value(b'\0'));
        let result = iter.find_value(b'\0');
        assert!(result.is_some());
        assert_eq!(result.unwrap(),15);
    }

    #[test]
    fn test_byte_iter_find_length() {
        let mut iter = ByteIterator::new(NULL_TERM_STR,ByteDelimiter::Length(14));
        let result = iter.find_length(14);
        assert!(result.is_some());
        assert_eq!(result.unwrap(),14);
    }

    #[test]
    fn test_byte_iter_take_value() {
        let mut iter = ByteIterator::new(NULL_TERM_STR,ByteDelimiter::Value(b'\0'));
        let result = iter.take_value(b'\0');

        assert!(result.is_some());
        
        let data = result.unwrap();
        let string = std::str::from_utf8(data).unwrap();
        
        assert_eq!(data.len(),15);
        assert_eq!(string,"This is a test\0");
    }

    #[test]
    fn test_byte_iter_take_length() {
        let mut iter = ByteIterator::new(NULL_TERM_STR,ByteDelimiter::Length(14));
        let result = iter.take_length(14);

        assert!(result.is_some());
        
        let data = result.unwrap();
        let string = std::str::from_utf8(data).unwrap();
        
        assert_eq!(data.len(),14);
        assert_eq!(string,"This is a test");
    }

    #[test]
    fn test_byte_iter_by_value() {
        let mut iter = ByteIterator::new(NULL_TERM_STR,ByteDelimiter::Value(b'\0'));
        let strings = iter
            .filter_map(|d| std::str::from_utf8(d).ok())
            .map(|s| s.into())
            .collect::<Vec<String>>();


        assert_eq!(strings.len(),2);
        assert_eq!(strings[0],"This is a test\0");
        assert_eq!(strings[1],"Another test\0");
    }

    #[test]
    fn test_byte_iter_by_length() {
        let mut iter = ByteIterator::new(FIXED_LEN_DATA,ByteDelimiter::Length(14));
        let strings = iter
            .filter_map(|d| std::str::from_utf8(d).ok())
            .map(|s| s.into())
            .collect::<Vec<String>>();

        assert_eq!(strings.len(),2);
        assert_eq!(strings[0],"This is a test");
        assert_eq!(strings[1],"This is a test");
    }

}