use crate::{symbols::Symbol, errors::Error, common::STType};

pub struct Function {
    name: String,
    symbol: Symbol
}

impl Function {

    fn new(symbol: Symbol) -> Function {
        Self { 
            name: String::new(),
            symbol 
        }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    pub fn address(&self) -> u64 {
        self.symbol.value()
    }

    pub fn name(&self) -> &str {
        self.name.as_ref()
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