//! Utility functions for both testing and execution
//!

#[cfg(test)]
pub mod tests {  
    use std::fs::File;
    use std::io::Read;
    use std::io::BufReader;

    pub struct TestSection {
        pub bytes: &'static [u8],
        pub name: &'static str,
        pub address: usize,
        pub offset: usize,
        pub size: usize,
        pub length: usize,
        pub entsize: usize,
    }

    pub const LIBJPEG_DYNSYM: TestSection = TestSection {
        bytes: include!("../assets/libjpeg/dump/section_dynsym.in"),
        name: ".dynsym",
        address: 1864,
        offset: 1864,
        size: 4656,
        length: 194,
        entsize: 24,
    };

    pub const LIBQSCINTILLA_FINI_ARRAY: TestSection = TestSection {
        bytes: include!("../assets/libqscintilla2/dump/section_fini_array.in"),
        name: ".fini_array",
        address: 3811960,
        offset: 3807864,
        size: 8,
        length: 1,
        entsize: 8,
    };

    pub const LIBQSCINTILLA_INIT_ARRAY: TestSection = TestSection {
        bytes: include!("../assets/libqscintilla2/dump/section_init_array.in"),
        name: ".init_array",
        address: 3811048,
        offset: 3806952,
        size: 912,
        length: 114,
        entsize: 8,
    };

    pub const LIBVPF_DYNSYM: TestSection = TestSection {
        bytes: include!("../assets/libvpf/dump/section_dynsym.in"),
        name: ".dynsym",
        address: 2744,
        offset: 2744,
        size: 7056,
        length: 294,
        entsize: 24,
    };

    pub const LIBVPF_INIT_ARRAY: TestSection = TestSection {
        bytes: include!("../assets/libvpf/dump/section_init_array.in"),
        name: ".init_array",
        address: 289584,
        offset: 285488,
        size: 8,
        length: 1,
        entsize: 8,
    };

    pub const LIBVPF_RELA_DYN: TestSection = TestSection {
        bytes: include!("../assets/libvpf/dump/section_rela_dyn.in"),
        name: ".rela.dyn",
        address: 14656,
        offset: 14656,
        size: 1224,
        length: 51,
        entsize: 24,
    };

    pub const LIBVPF_RELA_PLT: TestSection = TestSection {
        bytes: include!("../assets/libvpf/dump/section_rela_plt.in"),
        name: ".rela.plt",
        address: 15880,
        offset: 15880,
        size: 1344,
        length: 56,
        entsize: 24,
    };

    pub const LIBVPF_SHSTRTAB: TestSection = TestSection {
        bytes: include!("../assets/libvpf/dump/section_shstrtab.in"),
        name: ".shstrtab",
        address: 0,
        offset: 287172,
        size: 263,
        length: 26,
        entsize: 0,
    };

    pub fn read(path: &str) -> Vec<u8> {
        File::open(path)
            .map(BufReader::new)
            .and_then(|b| b
                .bytes()
                .collect())
            .expect("Could not read file")
    }

}