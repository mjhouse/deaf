use crate::errors::Result;
use crate::common::{
    Width,
    Layout,
    Item,
    ranges::*, STType, STBind
};
use crate::symbols::SymbolInfo;
use crate::tables::TableItem;

/// A Symbol item found in symbol tables
#[derive(Clone,Debug)]
pub struct Symbol {
    st_name: Item<u32,u32,usize>,
    st_value: Item<u32,u64>,
    st_size: Item<u32,u64>,
    st_info: Item<u8,u8,SymbolInfo>,
    st_other: Item<u8>,
    st_shndx: Item<u16,u16>,
}

impl Symbol {

    /// Create a new symbol
    pub fn new() -> Self {
        Self {
            st_name: Item::new(ST_NAME), 
            st_value: Item::new(ST_VALUE),
            st_size: Item::new(ST_SIZE), 
            st_info: Item::new(ST_INFO),    
            st_other: Item::new(ST_OTHER),     
            st_shndx: Item::new(ST_SHNDX),
        }
    }

    /// Get the 'st_name' field (name *index*) of the symbol
    pub fn name(&self) -> usize {
        self.st_name.get()
    }

    /// Set the 'st_name' field (name *index*) of the symbol
    pub fn set_name(&mut self, value: usize) {
        self.st_name.set(value);
    }

    /// Get the 'st_value' field of the symbol
    pub fn value(&self) -> u64 {
        self.st_value.get()
    }

    /// Set the 'st_value' field of the symbol
    pub fn set_value(&mut self, value: u64) {
        self.st_value.set(value);
    }

    /// Get the 'st_size' field of the symbol
    pub fn size(&self) -> u64 {
        self.st_size.get()
    }

    /// Set the 'st_size' field of the symbol
    pub fn set_size(&mut self, value: u64) {
        self.st_size.set(value);
    }

    /// Get the calculated size of the symbol
    pub fn item_size(&self) -> usize {
        TableItem::size(self)
    }

    /// Get the 'st_info' field of the symbol
    pub fn info(&self) -> SymbolInfo {
        self.st_info.get()
    }

    /// Set the 'st_info' field of the symbol
    pub fn set_info(&mut self, value: SymbolInfo) {
        self.st_info.set(value);
    }

    /// Get the type of the symbol
    pub fn kind(&self) -> STType {
        self.info().kind()
    }

    /// Set the kind of the symbol
    pub fn set_kind(&mut self, kind: STType) {
        self.set_info(self.info().with_kind(kind))
    }

    /// Get the binding of the symbol
    pub fn bind(&self) -> STBind {
        self.info().bind()
    }

    /// Set the binding of the symbol
    pub fn set_bind(&mut self, bind: STBind) {
        self.set_info(self.info().with_bind(bind))
    }

    /// Get the 'st_other' field of the symbol
    pub fn other(&self) -> u8 {
        self.st_other.get()
    }

    /// Set the 'st_other' field of the symbol
    pub fn set_other(&mut self, value: u8) {
        self.st_other.set(value);
    }

    /// Get the 'st_shndx' field of the symbol
    pub fn shndx(&self) -> u16 {
        self.st_shndx.get()
    }

    /// Set the 'st_shndx' field of the symbol
    pub fn set_shndx(&mut self, value: u16) {
        self.st_shndx.set(value);
    }

    /// Get the current layout of the symbol
    pub fn layout(&self) -> Layout {
        self.st_name.layout()
    }

    /// Set the current layout of the symbol
    pub fn set_layout(&mut self, layout: Layout){
        TableItem::set_layout(self, layout)
    }

    /// Get the current width of the symbol
    pub fn width(&self) -> Width {
        self.st_name.width()
    }

    /// Set the current width of the symbol
    pub fn set_width(&mut self, width: Width){
        TableItem::set_width(self, width)
    }

    /// Read all fields from a data buffer
    pub fn read(&mut self, data: &[u8]) -> Result<()> {
        TableItem::read(self, data)
    }

    /// Write all fields to a mutable data buffer
    pub fn write(&self, data: &mut [u8]) -> Result<()> {
        TableItem::write(self,data)
    }

}

impl TableItem for Symbol {

    fn set_layout(&mut self, layout: Layout){
        self.st_name.set_layout(layout);
        self.st_value.set_layout(layout);
        self.st_size.set_layout(layout);
        self.st_info.set_layout(layout);
        self.st_other.set_layout(layout);
        self.st_shndx.set_layout(layout);
    }

    fn set_width(&mut self, width: Width){
        self.st_name.set_width(width);
        self.st_value.set_width(width);
        self.st_size.set_width(width);
        self.st_info.set_width(width);
        self.st_other.set_width(width);
        self.st_shndx.set_width(width);
    }

    fn read(&mut self, b: &[u8]) -> Result<()> {
        self.st_name.read(b)?;
        self.st_value.read(b)?;
        self.st_size.read(b)?;
        self.st_info.read(b)?;
        self.st_other.read(b)?;
        self.st_shndx.read(b)?;
        Ok(())
    }

    fn write(&self, b: &mut [u8]) -> Result<()> {
        self.st_name.write(b)?;
        self.st_value.write(b)?;
        self.st_size.write(b)?;
        self.st_info.write(b)?;
        self.st_other.write(b)?;
        self.st_shndx.write(b)?;
        Ok(())
    }

    fn size(&self) -> usize {
        self.st_name.size() +
        self.st_value.size() +
        self.st_size.size() +
        self.st_info.size() +
        self.st_other.size() +
        self.st_shndx.size()
    }

}

impl Default for Symbol {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! setup {
        () => {{
            let mut symbol = Symbol::new();

            symbol.set_layout(Layout::Little);
            symbol.set_width(Width::X64);
    
            let data: [u8;24] = [
                0x01, 0x00, 0x00, 0x00,                         // name:  1
                0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // value: 1
                0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // size:  24
                0x21,                                           // info:  STB_WEAK + STT_OBJECT
                0x01,                                           // other: 1
                0x01, 0x00,                                     // shndx: 1
            ];

            (symbol,data)
        }};
    }

    #[test]
    fn test_symbol_read_write_kind() {
        let (mut symbol, mut data) = setup!();

        let result = symbol.read(&data);
        assert!(result.is_ok());
        assert_eq!(symbol.kind(),STType::STT_OBJECT);
        
        symbol.set_kind(STType::STT_FUNC);
        assert_eq!(symbol.kind(),STType::STT_FUNC);

        let result = symbol.write(&mut data);
        assert!(result.is_ok());

        let result = symbol.read(&data);
        assert!(result.is_ok());
        assert_eq!(symbol.kind(),STType::STT_FUNC);
    }

    #[test]
    fn test_symbol_read_write_bind() {
        let (mut symbol, mut data) = setup!();

        let result = symbol.read(&data);
        assert!(result.is_ok());
        assert_eq!(symbol.bind(),STBind::STB_LOCAL);
        
        symbol.set_bind(STBind::STB_GLOBAL);
        assert_eq!(symbol.bind(),STBind::STB_GLOBAL);

        let result = symbol.write(&mut data);
        assert!(result.is_ok());

        let result = symbol.read(&data);
        assert!(result.is_ok());
        assert_eq!(symbol.bind(),STBind::STB_GLOBAL);
    }
}