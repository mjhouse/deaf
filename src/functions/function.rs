use crate::errors::{Error,Result};
use crate::common::{STType,STBind,STVisibility,Layout,Width};
use crate::symbols::{Symbol,SymbolInfo};

#[derive(Default)]
pub struct Function {
    name: String,
    symbol: Symbol
}

impl Function {

    pub fn new() -> Function {
        Self::default()
    }

    /// Get the name of the function
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Set the name of the function
    /// 
    /// This function will override the 'st_name' field in the 
    /// symbol when the function is added to a binary and may
    /// insert a new value into the string table.
    pub fn set_name<T: Into<String>>(&mut self, value: T) {
        self.name = value.into();
    }

    /// Builder method to set the name of the function
    /// 
    /// This function will override the 'st_name' field in the 
    /// symbol when the function is added to a binary and may
    /// insert a new value into the string table.
    pub fn with_name<T: Into<String>>(mut self, name: T) -> Self {
        self.set_name(name);
        self
    }

    /// Get the function body
    pub fn body(&self) -> &[u8] {
        self.symbol.data()
    }

    /// Set the function body
    /// 
    /// Also updates the size of the symbol to reflect
    /// the size of the given body.
    pub fn set_body(&mut self, body: &[u8]) {
        self.symbol.set_data(body);
    }

    /// Extract the function body from section data
    /// 
    /// Also updates the size of the symbol to reflect
    /// the size of the extracted body.
    pub fn set_body_from(&mut self, data: &[u8], offset: usize) -> Result<()> {
        self.read_body(data, offset)?;
        Ok(())
    }

    /// Builder method to set the function body
    /// 
    /// Also updates the size of the symbol to reflect
    /// the size of the given body.
    pub fn with_body(mut self, body: &[u8]) -> Self {
        self.set_body(body);
        self
    }

    /// Builder method to extract body from section data 
    /// 
    /// Also updates the size of the symbol to reflect
    /// the size of the extracted body.
    pub fn with_body_from(mut self, data: &[u8], offset: usize) -> Result<Self> {
        self.set_body_from(data, offset)?;
        Ok(self)
    }

    /// Get the 'st_value' field of the function symbol
    pub fn address(&self) -> usize {
        self.symbol.value() as usize
    }

    /// Set the 'st_value' field of the function symbol
    pub fn set_address(&mut self, value: usize) {
        self.symbol.set_value(value as u64);
    }

    /// Builder method to set the 'st_value' field of the function symbol
    pub fn with_address(mut self, value: usize) -> Self {
        self.set_address(value);
        self
    }

    /// Get the 'st_size' field of the function symbol
    pub fn size(&self) -> usize {
        self.symbol.size() as usize
    }

    /// Set the 'st_size' field of the function symbol
    pub fn set_size(&mut self, value: usize) {
        self.symbol.set_size(value as u64);
    }

    /// Builder method to set the 'st_size' field of the function symbol
    pub fn with_size(mut self, value: usize) -> Self {
        self.set_size(value);
        self
    }

    /// Get the 'st_info' field of the function symbol
    pub fn info(&self) -> SymbolInfo {
        self.symbol.info()
    }

    /// Set the 'st_info' field of the function symbol
    pub fn set_info(&mut self, value: SymbolInfo) {
        self.symbol.set_info(value)
    }

    /// Builder method to set the 'st_info' field of the function symbol
    pub fn with_info(mut self, value: SymbolInfo) -> Self {
        self.set_info(value);
        self
    }

    /// Get the type of the function symbol
    pub fn kind(&self) -> STType {
        self.info().kind()
    }

    /// Set the kind of the function symbol
    pub fn set_kind(&mut self, kind: STType) {
        self.symbol.set_kind(kind);
    }

    /// Builder method to set the kind (on SymbolInfo) of the function symbol
    pub fn with_kind(mut self, value: STType) -> Self {
        self.set_kind(value);
        self
    }

    /// Get the binding of the function symbol
    pub fn bind(&self) -> STBind {
        self.symbol.bind()
    }

    /// Set the binding of the function symbol
    pub fn set_bind(&mut self, bind: STBind) {
        self.symbol.set_bind(bind)
    }

    /// Builder method to set the bind (on SymbolInfo) of the function symbol
    pub fn with_bind(mut self, value: STBind) -> Self {
        self.set_bind(value);
        self
    }

    /// Get the 'st_other' field as an enum
    pub fn visibility(&self) -> STVisibility {
        self.symbol.visibility()
    }

    /// Set the 'st_other' field as an enum
    pub fn set_visibility(&mut self, value: STVisibility) {
        self.symbol.set_visibility(value);
    }

    /// Builder method to set the 'st_other' field as an enum
    pub fn with_visibility(mut self, value: STVisibility) -> Self {
        self.set_visibility(value);
        self
    }

    /// Get the section index of the function symbol 
    /// 
    /// This gets the 'st_shndx' field of the internal symbol.
    pub fn section(&self) -> usize {
        self.symbol.shndx() as usize
    }

    /// Set the section index of the function symbol 
    /// 
    /// This sets the 'st_shndx' field of the internal symbol.
    pub fn set_section(&mut self, value: usize) {
        self.symbol.set_shndx(value as u16);
    }

    /// Builder method to set the sextion index of the function symbol
    /// 
    /// This sets the 'st_shndx' field of the internal symbol.
    pub fn with_section(mut self, value: usize) -> Self {
        self.set_section(value);
        self
    }

    /// Get the current layout of the function symbol
    pub fn layout(&self) -> Layout {
        self.symbol.layout()
    }

    /// Set the current layout of the function symbol
    pub fn set_layout(&mut self, layout: Layout){
        self.symbol.set_layout(layout)
    }

    /// Builder method to set the layout of the function symbol
    pub fn with_layout(mut self, value: Layout) -> Self {
        self.set_layout(value);
        self
    }

    /// Get the current width of the function symbol
    pub fn width(&self) -> Width {
        self.symbol.width()
    }

    /// Set the current width of the function symbol
    pub fn set_width(&mut self, width: Width){
        self.symbol.set_width(width)
    }

    /// Builder method to set the width of the function symbol
    pub fn with_width(mut self, value: Width) -> Self {
        self.set_width(value);
        self
    }

    /// Get the symbol that represents this function
    /// 
    /// Fields on the symbol will not reflect some changes
    /// until the function is added to the binary. The 
    /// function name, for example, may be set as a string, 
    /// but will not update the 'st_name' field of the symbol 
    /// until the string is added to the string table.
    pub fn symbol(&self) -> &Symbol {
        &self.symbol
    }

    /// Get the mutable symbol that represents this function
    /// 
    /// Changes made directly to the symbol may be overridden
    /// when the function is added to the binary. The 'st_name'
    /// field of the symbol, for example, will be overwritten
    /// when the string 'name' is added to the string table.
    pub fn symbol_mut(&mut self) -> &mut Symbol {
        &mut self.symbol
    }

    /// Set the symbol of the function
    /// 
    /// This method will overwrite most existing changes made 
    /// to the function and some fields on the given symbol 
    /// will be overwritten when the function is added to the 
    /// binary.
    pub fn set_symbol(&mut self, value: Symbol) {
        self.symbol = value;
    }

    /// Builder method to set the symbol of the function
    /// 
    /// This method will overwrite most existing changes made 
    /// to the function and some fields on the given symbol 
    /// will be overwritten when the function is added to the 
    /// binary.
    pub fn with_symbol(mut self, value: Symbol) -> Self {
        self.set_symbol(value);
        self
    }

    /// Get the offset starting position of the function body
    pub fn start(&self, offset: usize) -> usize {
        self.address().saturating_sub(offset)
    }

    /// Get the offset ending position of the function body
    pub fn end(&self, offset: usize) -> usize {
        self.start(offset).saturating_add(self.size())
    }

    /// Read the symbol entry from the beginning of the given data
    pub fn read_symbol(&mut self, data: &[u8]) -> Result<()> {
        self.symbol.read(data)
    }

    /// Read the function body from the given data
    /// 
    /// Offset is subtracted from the function address to position
    /// the function body inside of a section.
    pub fn read_body(&mut self, data: &[u8], offset: usize) -> Result<()> {
        let start = self.start(offset);
        let end = self.end(offset);
        if start <= end && end <= data.len() {
            self.set_body(&data[start..end]);
            Ok(())
        }
        else {
            Err(Error::OutOfBoundsError)
        }
    }

    /// Write the symbol entry to the beginning of the given data
    fn write_symbol(&self, data: &mut [u8]) -> Result<()> {
        assert!(self.symbol.is_function());
        self.symbol.write(data)
    }

    /// Write the function body to the given data
    /// 
    /// Offset is subtracted from the function address to position
    /// the function body inside of a section.
    pub fn write_body(&self, data: &mut [u8], offset: usize) -> Result<()> {
        let start = self.start(offset);
        let end = self.end(offset);
        if start <= end && end <= data.len() {
            let body = self.body();
            data[start..end].copy_from_slice(body);
            Ok(())
        }
        else {
            Err(Error::OutOfBoundsError)
        }
    }

    /// Finish and validate the new function
    pub fn build(self) -> Result<Self> {

        // ensure that the symbol is valid
        self.symbol.validate()?;

        // ensure that the symbol is a function
        if !self.symbol.is_function() {
            return Err(Error::WrongTypeError);
        }

        Ok(self)
    }

}

impl TryFrom<Symbol> for Function {
    type Error = Error;

    fn try_from(symbol: Symbol) -> Result<Self> {
        match symbol.kind() {
            STType::STT_FUNC => Self::new()
                .with_symbol(symbol)
                .build(),
            _ => Err(Error::WrongTypeError)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::common::{Layout, Width, STBind};

    use super::*;

    macro_rules! setup_32 {
        () => {{
            // 32-bit, little-endian hex representation of a symbol
            // with attached assembly function body
            let data: [u8;44] = [
                0x00, 0x00, 0x00, 0x00,                         // name:  0
                0x10, 0x00, 0x00, 0x00,                         // value: 16
                0x1c, 0x00, 0x00, 0x00,                         // size:  28
                0x12,                                           // info:  STB_GLOBAL + STT_FUNC
                0x01,                                           // other: 1
                0x01, 0x00,                                     // shndx: 1

                0xf3, 0x0f, 0x1e, 0xfa,          	        // endbr64 
                0x48, 0x83, 0xec, 0x48,          	        // sub    $0x48,%rsp
                0x8b, 0x17,                   	            // mov    (%rdi),%edx
                0x48, 0x8b, 0x4e, 0x10,          	        // mov    0x10(%rsi),%rcx
                0x64, 0x48, 0x8b, 0x04, 0x25, 0x28, 0x00, 	// mov    %fs:0x28,%rax
                0x75, 0x05,                  	            // jne    5864 <Icmpval@@Base+0x64>
                0x48, 0x83, 0xc4, 0x48,          	        // add    $0x48,%rsp
                0xc3                   	                    // ret
            ];

            data
        }};
    }

    macro_rules! setup_64 {
        () => {{
            // 64-bit, little-endian hex representation of a symbol
            // with attached assembly
            let data: [u8;52] = [
                0x01, 0x00, 0x00, 0x00,                         // name:  1
                0x12,                                           // info:  STB_GLOBAL + STT_FUNC
                0x01,                                           // other: 1
                0x01, 0x00,                                     // shndx: 1
                0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // value: 24 (address, for function)
                0x1c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // size:  28 (body size, for function)

                0xf3, 0x0f, 0x1e, 0xfa,          	        // endbr64 
                0x48, 0x83, 0xec, 0x48,          	        // sub    $0x48,%rsp
                0x8b, 0x17,                   	            // mov    (%rdi),%edx
                0x48, 0x8b, 0x4e, 0x10,          	        // mov    0x10(%rsi),%rcx
                0x64, 0x48, 0x8b, 0x04, 0x25, 0x28, 0x00, 	// mov    %fs:0x28,%rax
                0x75, 0x05,                  	            // jne    5864 <Icmpval@@Base+0x64>
                0x48, 0x83, 0xc4, 0x48,          	        // add    $0x48,%rsp
                0xc3                   	                    // ret
            ];

            data
        }};
    }

    #[test]
    fn test_create_function_64() {
        let data = setup_64!();

        let mut buffer: [u8;52] = [0;52];

        let function = Function::new()
            .with_symbol(Symbol::new()        
                .with_layout(Layout::Little)
                .with_width(Width::X64)
                .with_visibility(STVisibility::STV_INTERNAL)
                .with_bind(STBind::STB_GLOBAL)
                .with_kind(STType::STT_FUNC)
                .with_name(1)
                .with_shndx(1)
                .build()
                .unwrap())
            .with_name("Test")
            .with_size(28)
            .with_address(24)
            .with_body_from(&data, 0)
            .expect("could not read body from data")
            .build()
            .expect("could not build new function");

        let result = function.write_symbol(&mut buffer);
        assert!(result.is_ok());

        let result = function.write_body(&mut buffer, 0);
        assert!(result.is_ok());

        assert_eq!(buffer,data);
    }

    #[test]
    fn test_create_function_32() {
        let data = setup_32!();

        let mut buffer: [u8;44] = [0;44];

        let mut function = Function::new()
            .with_name("Test")
            .with_size(28)
            .with_section(1)
            .with_address(16)
            .with_width(Width::X32)
            .with_layout(Layout::Little)
            .with_bind(STBind::STB_GLOBAL)
            .with_kind(STType::STT_FUNC)
            .with_visibility(STVisibility::STV_INTERNAL)
            .build()
            .unwrap();

        let result = function.read_body(&data,0);
        assert!(result.is_ok());

        let result = function.write_symbol(&mut buffer);
        assert!(result.is_ok());

        let result = function.write_body(&mut buffer, 0);
        assert!(result.is_ok());

        assert_eq!(buffer,data);
    }

}