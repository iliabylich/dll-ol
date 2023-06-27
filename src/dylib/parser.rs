use goblin::{
    elf::Elf,
    mach::{Mach, MachO, MultiArch},
    pe::PE,
    Object,
};

pub(crate) struct Parser {
    data: Vec<u8>,
}

impl Parser {
    pub(crate) fn new(dlib_path: &str) -> Self {
        let data = std::fs::read(&dlib_path).unwrap();
        Self { data }
    }

    fn parse_all(&self) -> Vec<String> {
        match Object::parse(&self.data).unwrap() {
            Object::Elf(elf) => Self::parse_elf(elf),
            Object::PE(pe) => Self::parse_pe(pe),
            Object::Mach(Mach::Binary(mach)) => Self::parse_mach_o_binary(mach),
            Object::Mach(Mach::Fat(mach)) => Self::parse_mach_fat(mach),
            Object::Archive(archive) => todo!("archive: {:#?}", &archive),
            Object::Unknown(magic) => panic!("Unknown magic: {:#x}", magic),
        }
    }

    fn parse_elf(elf: Elf) -> Vec<String> {
        elf.strtab
            .to_vec()
            .unwrap()
            .into_iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    }

    fn parse_pe(pe: PE) -> Vec<String> {
        pe.exports
            .into_iter()
            .map(|export| export.name.unwrap_or_default().to_string())
            .collect::<Vec<_>>()
    }

    fn parse_mach_o_binary(mach: MachO) -> Vec<String> {
        mach.exports()
            .unwrap()
            .into_iter()
            .map(|e| e.name)
            .collect::<Vec<_>>()
    }

    fn parse_mach_fat(mach: MultiArch) -> Vec<String> {
        todo!("parse_mach_fat: {:#?}", mach)
    }

    pub(crate) fn parse_test_symbols(self) -> Vec<String> {
        let mut symbols = self.parse_all();
        symbols.retain(|sym| sym.contains("__ol_test_"));
        for symbol in symbols.iter_mut() {
            if symbol.starts_with("___ol_test_") {
                *symbol = symbol.strip_prefix('_').unwrap().to_string();
            }
        }
        symbols
    }
}

#[cfg(test)]
fn parse_fixture(path: &str) -> Vec<String> {
    let mut symbols = Parser::new(path).parse_test_symbols();
    symbols.sort_unstable();
    symbols
}

#[test]
fn test_elf() {
    let symbols = parse_fixture(crate::fixtures::ELF_PATH);
    assert_eq!(
        symbols,
        vec!["__ol_test_crash", "__ol_test_fail", "__ol_test_pass"]
    );
}

#[test]
fn test_mach_o() {
    let symbols = parse_fixture(crate::fixtures::MACH_O_PATH);
    assert_eq!(
        symbols,
        vec!["__ol_test_crash", "__ol_test_fail", "__ol_test_pass"]
    );
}

#[test]
fn test_pe() {
    let symbols = parse_fixture(crate::fixtures::PE_PATH);
    assert_eq!(
        symbols,
        vec!["__ol_test_crash", "__ol_test_fail", "__ol_test_pass"]
    );
}
