use crate::{symbols::Symbol, errors::Error, common::STType};

pub struct Function {
    name: String,
    body: Vec<u8>,
    symbol: Symbol
}

impl Function {

    fn new(symbol: Symbol) -> Function {
        Self { 
            name: String::new(),
            body: Vec::new(),
            symbol 
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self
    }

    pub fn address(&self) -> u64 {
        self.symbol.value()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn body(&self) -> &[u8] {
        self.body.as_slice()
    }

    pub fn body_mut(&mut self) -> &mut [u8] {
        self.body.as_mut_slice()
    }

    pub fn size(&self) -> usize {
        self.symbol.size() as usize
    }

}

impl TryFrom<Symbol> for Function {
    type Error = Error;

    fn try_from(symbol: Symbol) -> Result<Self, Self::Error> {
        match symbol.kind() {
            STType::STT_FUNC => Ok(Self::new(symbol)),
            _ => Err(Error::WrongTypeError)
        }
    }
}