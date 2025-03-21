use std::path::Path;
use std::fs;

use crate::{Section, Segment};
use crate::symbols::Symbol;
use crate::functions::Function;
use crate::tables::{Table,TableView,StringItem, SymbolTable, SymbolTableMut, StringTable};
use crate::headers::FileHeader;
use crate::errors::{Error,Result};
use crate::common::{
    Layout,
    Width,
    SectionType,
    Updateable,
    Update,
    All
};

/// An ELF formatted binary file
pub struct Binary {
    header: FileHeader,
    sections: Vec<Section>,
    segments: Vec<Segment>,
}

impl Binary {

    fn empty() -> Self {
        Self { 
            header: FileHeader::new(), 
            sections: Vec::new(),
            segments: Vec::new()
        }
    }

    fn new(header: FileHeader, sections: Vec<Section>, segments: Vec<Segment>) -> Self {
        Self { header, sections, segments }
    }

    pub fn read(&mut self, data: &[u8]) -> Result<usize> {
        self.header = FileHeader::parse(&data)?;

        let count = self.header.shnum();
        let offset = self.header.shoff();
        let size = self.header.shentsize();
        let layout = self.header.layout();
        let width = self.header.width();

        self.sections = Section::read_all(
            &data,
            count,
            offset,
            size,
            layout,
            width
        )?;

        let count = self.header.phnum();
        let offset = self.header.phoff();
        let size = self.header.phentsize();

        self.segments = Segment::read_all(
            &data,
            count,
            offset,
            size,
            layout,
            width
        )?;

        self.process()?;
        Ok(self.size())
    }

    pub fn write(&self, data: &mut [u8]) -> Result<usize> {
        self.header.write(data)?;
        let offset = self.header.shoff();

        for (index,section) in self.sections.iter().enumerate() {
            section.write(
                data,
                offset,
                index,
            )?;
        }

        Ok(self.size())
    }

    pub fn load<T: AsRef<Path>>(path: T) -> Result<Self> {
        let data = fs::read(path.as_ref())?;
        let mut binary = Binary::empty();        
        binary.read(&data)?;
        Ok(binary)
    }

    pub fn save<T: AsRef<Path>>(&self, path: T) -> Result<usize> {
        let size = self.size();
        let mut data = vec![0;size];
        self.write(&mut data)?;
        fs::write(path, data)?;
        Ok(size)
    }

    pub fn process(&mut self) -> Result<()> {

        let str_section = self.section(self.header.shstrndx())?.clone();
        let str_table = StringTable::try_from(&str_section)?;

        for section in self.sections.iter_mut() {
            let offset = section.name_index();
            let name = str_table
                .at_offset(offset)
                .and_then(|e| e.string())
                .unwrap_or("".into());

            section.set_name(name);
        }

        Ok(())
    }

    pub fn size(&self) -> usize {
        self.header.size() +
        self.sections
            .iter()
            .fold(0,|a,s| a + s.size())
    }

    pub fn section(&self, index: usize) -> Result<&Section> {
        self.sections
            .get(index)
            .ok_or(Error::NotFound)
    }

    pub fn section_for_address(&self, address: usize) -> Result<&Section> {
        self.sections
            .iter()
            .find(|s| s.start() <= address && s.end() > address)
            .ok_or(Error::NotFound)
    }

    pub fn section_mut(&mut self, index: usize) -> Result<&mut Section> {
        self.sections
            .get_mut(index)
            .ok_or(Error::NotFound)
    }

    pub fn sections(&self, kind: SectionType) -> Vec<&Section> {
        self.sections
            .iter()
            .filter(|s| s.is_kind(kind))
            .collect()
    }

    pub fn sections_mut(&mut self, kind: SectionType) -> Vec<&mut Section> {
        self.sections
            .iter_mut()
            .filter(|s| s.is_kind(kind))
            .collect()
    }

    pub fn segments(&self) -> Vec<&Segment> {
        self.segments
            .iter()
            .collect()
    }

    pub fn segments_mut(&mut self) -> Vec<&mut Segment> {
        self.segments
            .iter_mut()
            .collect()
    }

    pub fn section_name(&self, offset: usize) -> Result<String> {
        self.section(self.header.shstrndx())
            .and_then(Table::<StringItem>::try_from)
            .and_then(|t| t
                .at_offset(offset)
                .and_then(|e| e.string()))
    }

    pub fn section_names(&self) -> Result<Vec<String>> {
        self.section(self.header.shstrndx())
            .and_then(StringTable::try_from)
            .and_then(|t| t
                .items())
            .and_then(|v| v
                .iter()
                .map(|e| e.string())
                .collect())
    }

    /// Get all string tables except the 'shstrtab'
    pub fn string_tables(&self) -> Vec<StringTable> {
        let k = self.header.shstrndx();
        self.sections
            .iter()
            .enumerate()
            .filter(|(i,_)| *i != k)
            .map(|(_,t)| t)
            .flat_map(StringTable::try_from)
            .collect()
    }

    pub fn symbol_tables(&self) -> Vec<SymbolTable> {
        self.sections
            .iter()
            .flat_map(SymbolTable::try_from)
            .collect()
    }

    pub fn symbol_tables_mut(&mut self) -> Vec<SymbolTableMut> {
        self.sections
            .iter_mut()
            .flat_map(SymbolTableMut::try_from)
            .collect()
    }

    pub fn symbol_name(&self, offset: usize) -> Result<String> {
        self.string_tables()
            .iter()
            .find_map(|t| t
                .at_offset(offset)
                .and_then(|s| s.string())
                .ok())
            .ok_or(Error::NotFound)
    }

    pub fn symbols(&self) -> Result<Vec<Symbol>> {

        let mut symbols = self
            .symbol_tables()
            .iter()
            .flat_map(SymbolTable::items)
            .flatten()
            .collect::<Vec<Symbol>>();

        for symbol in symbols.iter_mut() {

            if (symbol.is_object()   ||
                symbol.is_function() ||
                symbol.is_section()) &&
                !symbol.is_weak()    &&
                symbol.has_section()
            {
                let section = symbol.section(self)?;

                // get an offset into the section
                let offset = symbol
                    .value()
                    .saturating_sub(section
                        .address() as u64);

                // get a slice of bytes the size of the symbol
                let data = section
                    .slice(
                        offset as usize,
                        symbol.size() as usize)?;

                // add them to the symbol
                symbol.set_data(data);

            }
        }

        Ok(symbols)
    }

    pub fn functions(&self) -> Result<Vec<Function>> {

        let mut functions = self
            .symbols()?
            .into_iter()
            .filter_map(|s| Function::try_from(s).ok())
            .collect::<Vec<Function>>();

        for function in functions.iter_mut() {

            // get the function name
            let index = function.symbol().name();
            let name = self.symbol_name(index)?;

            // set the function name
            function.set_name(name)

        }

        Ok(functions)
    }

    pub fn data(&self, address: usize, size: usize) -> Vec<u8> {
        self.section_for_address(address)
            .and_then(|s| s.slice(s.offset().saturating_sub(address), size))
            .and_then(|d| Ok(d.to_vec()))
            .unwrap_or(Vec::new())
    }

    /// Get the number of section headers in the file
    pub fn shnum(&self) -> usize {
        self.header.shnum()
    }

    /// Get the offset of the section header table
    pub fn shoff(&self) -> usize {
        self.header.shoff()
    }

    /// Get the size of section headers
    pub fn shentsize(&self) -> usize {
        self.header.shentsize()
    }

    /// Get the number of program headers in the file
    pub fn phnum(&self) -> usize {
        self.header.phnum()
    }

    /// Get the offset of the program header table
    pub fn phoff(&self) -> usize {
        self.header.phoff()
    }

    /// Get the size of program headers
    pub fn phentsize(&self) -> usize {
        self.header.phentsize()
    }

    pub fn shstrndx(&self) -> usize {
        self.header.shstrndx()
    }

    /// Get the layout of the file (little or big endian)
    pub fn layout(&self) -> Layout {
        self.header.data()
    }

    /// Get the addressing width of the file (32, 64 etc)
    pub fn width(&self) -> Width {
        self.header.class()
    }

}

impl Updateable for Binary {
    fn update(&mut self) {
        self.header.update();
        self.sections.update();
        Update::<All>::clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_string_table() {
        let binary = Binary::load("assets/libjpeg/libjpeg.so.9").unwrap();

        let names = binary
            .sections(SectionType::Strings)
            .iter()
            .map(|s| s.name_index())
            .map(|i| binary.section_name(i))
            .collect::<Result<Vec<String>>>()
            .unwrap();

        assert_eq!(names[0].as_str(),".dynstr");
        assert_eq!(names[1].as_str(),".shstrtab");
        assert_eq!(names[2].as_str(),".strtab");
    }

    #[test]
    fn test_get_symbol_tables() {
        let path = "assets/libvpf/libvpf.so.4.1";
        let binary = Binary::load(path).unwrap();

        let tables = binary.symbol_tables();
        assert_eq!(tables.len(),1);

        let index = tables[0].name_index();
        assert_eq!(index,59);

        let result = binary.section_name(index);
        assert!(result.is_ok());

        let name = result.unwrap();
        assert_eq!(name, ".dynsym".to_string());
    }

    #[test]
    fn test_get_symbols() {
        let path = "assets/libvpf/libvpf.so.4.1";
        let binary = Binary::load(path).unwrap();

        let result = binary.symbols();
        assert!(result.is_ok());

        let symbols = result.unwrap();
        assert_eq!(symbols.len(),294);

        let index = symbols[1].name();
        let result = binary.symbol_name(index);
        assert!(result.is_ok());

        let name = result.unwrap();
        assert_eq!(name, "__ctype_toupper_loc".to_string());

        let index = symbols[78].name();
        let result = binary.symbol_name(index);
        assert!(result.is_ok());

        let name = result.unwrap();
        assert_eq!(name, "NOPROJ".to_string());
    }

    #[test]
    fn test_get_functions() {
        let path = "assets/libvpf/libvpf.so.4.1";
        let binary = Binary::load(path).unwrap();

        let result = binary.functions();

        assert!(result.is_ok());

        let functions = result.unwrap();
        assert_eq!(functions.len(),280);

        let function1 = &functions[80];
        let function2 = &functions[171];
        let function3 = &functions[238];

        assert_eq!(function1.address(),0x15df0);
        assert_eq!(function2.address(),0x06250);
        assert_eq!(function3.address(),0x256a0);

        assert_eq!(function1.name(),"table_in_list".to_string());
        assert_eq!(function2.name(),"swap_two".to_string());
        assert_eq!(function3.name(),"leftjust".to_string());
    }

    #[test]
    fn test_get_sections_for_address() {
        let path = "assets/libvpf/libvpf.so.4.1";
        let binary = Binary::load(path).unwrap();

        let result = binary.section_for_address(0x2d5);
        assert!(result.is_ok());

        let section = result.unwrap();
        assert_eq!(section.name(),".note.gnu.build-id");

        let result = binary.section_for_address(0x2f0);
        assert!(result.is_ok());

        let section = result.unwrap();
        assert_eq!(section.name(),".gnu.hash");

        let result = binary.section_for_address(0x5740);
        assert!(result.is_ok());

        let section = result.unwrap();
        assert_eq!(section.name(),".text");

        let result = binary.section_for_address(0x33f40);
        assert!(result.is_ok());

        let section = result.unwrap();
        assert_eq!(section.name(),".fini");

        let result = binary.section_for_address(0x33f39);
        assert!(result.is_ok());

        let section = result.unwrap();
        assert_eq!(section.name(),".text");
    }

    #[test]
    fn test_get_segments() {

        let path = "assets/libvpf/libvpf.so.4.1";
        let binary = Binary::load(path).unwrap();
        
        let segments = binary.segments();

        fn assert(binary: &Binary, segments: &Vec<&Segment>, index: usize, offset: usize, names: Vec<&str>) {
            // get the segment by index
            let segment = segments.iter().nth(index).unwrap();

            // check that the offset of the segment is correct
            assert_eq!(segment.offset(),offset);
    
            // get all included section names
            let sections = segment
                .sections(&binary)
                .into_iter()
                .map(Section::name)
                .collect::<Vec<String>>();
            
            // make sure the segment includes the correct number of sections
            assert_eq!(sections.len(),names.len());
    
            // make sure that each expected section is included by name
            for name in names.into_iter() {
                assert!(sections.contains(&name.to_string()),"{}",name);
            }

        }

        assert(
            &binary, &segments, 0, 0x00000, vec![
                "",
                ".note.gnu.property",
                ".note.gnu.build-id",
                ".gnu.hash",
                ".dynsym",
                ".dynstr",
                ".gnu.version",
                ".gnu.version_r",
                ".rela.dyn",
                ".rela.plt",
            ]
        );

        assert(
            &binary, &segments, 1, 0x05000, vec![
                ".init",
                ".plt",
                ".plt.got",
                ".plt.sec",
                ".text",
                ".fini",
            ]
        );

        assert(
            &binary, &segments, 2, 0x34000, vec![
                ".rodata",
                ".eh_frame_hdr",
                ".eh_frame",
            ]
        );

        assert(
            &binary, &segments, 3, 0x45b30, vec![
                ".init_array",
                ".fini_array",
                ".data.rel.ro",
                ".dynamic",
                ".got",
                ".data",
                ".bss",
            ]
        );

        assert(&binary, &segments, 4, 0x45bb0, vec![".dynamic"]);
        assert(&binary, &segments, 5, 0x002a8, vec![".note.gnu.property"]);
        assert(&binary, &segments, 6, 0x002c8, vec![".note.gnu.build-id"]);
        assert(&binary, &segments, 7, 0x002a8, vec![".note.gnu.property"]);
        assert(&binary, &segments, 8, 0x36bd0, vec![".eh_frame_hdr"]);
        assert(&binary, &segments, 9, 0x00000, vec![""]);

        assert(
            &binary, &segments, 10, 0x45b30, vec![
                ".init_array",
                ".fini_array",
                ".data.rel.ro",
                ".dynamic",
                ".got",
            ]
        );

    }


    // #[test]
    // fn test_display_sections() {
    //     let path = "assets/libvpf/libvpf.so.4.1";
    //     let binary = Binary::load(path).unwrap();

    //     for (i,section) in binary.sections.iter().enumerate() {
    //         let kind = section.header().kind();
    //         let index = section.name_index();
    //         let name = binary.section_name(index).unwrap();

    //         println!("{}: {} (kind={:?})",i,name,kind);
    //     }
    // }

    // #[test]
    // fn test_display_string_table() {
    //     let path = "assets/libvpf/libvpf.so.4.1";
    //     let binary = Binary::load(path).unwrap();

    //     let sections = &binary.sections[27];

    //     let dynstr = StringTable::try_from(sections).unwrap();
        
    //     for (i,item) in dynstr.items().unwrap().into_iter().enumerate() {
    //         println!("{}: {}",i,item.string_lossy());
    //     }
    // }

    // #[test]
    // fn test_display_symbol_table() {
    //     let path = "assets/libvpf/libvpf.so.4.1";
    //     let binary = Binary::load(path).unwrap();

    //     let strings = &binary.sections[5];
    //     let symbols = &binary.sections[4];

    //     let dynstr = StringTable::try_from(strings).unwrap();
    //     let dynsym = SymbolTable::try_from(symbols).unwrap();
        
    //     for (i,item) in dynsym.items().unwrap().into_iter().enumerate() {
    //         let name = dynstr
    //             .at_offset(item.name() as usize)
    //             .map(|v| v.string_lossy());
    //         println!("{}: {:?}",i,name);
    //     }
    // }

}