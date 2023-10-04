use crate::{symbols::Symbol, errors::Error, common::STType};

pub struct Function {
    name: String,
    body: Vec<u8>,
    symbol: Symbol
}

impl Function {

    pub fn new(symbol: Symbol) -> Function {
        Self { 
            name: String::new(),
            body: Vec::new(),
            symbol 
        }
    }

    pub(crate) fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub(crate) fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self
    }

    pub fn address(&self) -> usize {
        self.symbol.value() as usize
    }

    pub fn set_address(&mut self, value: usize) {
        self.symbol.set_value(value as u64);
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn set_name(&mut self, value: &str) {
        self.name = value.into();
    }

    pub fn body(&self) -> &[u8] {
        self.body.as_slice()
    }

    pub fn set_body(&mut self, body: &[u8]) {
        self.body = body.into();
    }

    pub fn body_mut(&mut self) -> &mut [u8] {
        self.body.as_mut_slice()
    }

    pub fn size(&self) -> usize {
        self.body.len()
    }

    pub fn symbol(&self) -> &Symbol {
        &self.symbol
    }

    pub fn symbol_mut(&mut self) -> &mut Symbol {
        &mut self.symbol
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