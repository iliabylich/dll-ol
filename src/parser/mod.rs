use goblin::{
    elf::Elf,
    mach::{Mach, MachO, MultiArch},
    pe::PE,
    Object,
};

mod error;
use error::ParseError;

pub(crate) struct Parser<'a> {
    data: &'a [u8],
}

impl<'a> Parser<'a> {
    pub(crate) fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    fn parse_all(&self) -> Result<Vec<String>, ParseError> {
        match Object::parse(&self.data)? {
            Object::Elf(elf) => Self::parse_elf(elf),
            Object::PE(pe) => Self::parse_pe(pe),
            Object::Mach(Mach::Binary(mach)) => Self::parse_mach_o_binary(mach),
            Object::Mach(Mach::Fat(mach)) => Self::parse_mach_fat(mach),
            Object::Archive(archive) => todo!("archive: {:#?}", &archive),
            Object::Unknown(magic) => Err(ParseError::UnsupportedFormat(magic)),
        }
    }

    fn parse_elf(elf: Elf) -> Result<Vec<String>, ParseError> {
        let syms = elf
            .strtab
            .to_vec()?
            .into_iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();
        Ok(syms)
    }

    fn parse_pe(pe: PE) -> Result<Vec<String>, ParseError> {
        let syms = pe
            .exports
            .into_iter()
            .map(|export| export.name.unwrap_or_default().to_string())
            .collect::<Vec<_>>();
        Ok(syms)
    }

    fn parse_mach_o_binary(mach: MachO) -> Result<Vec<String>, ParseError> {
        let syms = mach
            .exports()?
            .into_iter()
            .map(|e| e.name)
            .collect::<Vec<_>>();
        Ok(syms)
    }

    fn parse_mach_fat(mach: MultiArch) -> Result<Vec<String>, ParseError> {
        todo!("parse_mach_fat: {:#?}", mach)
    }

    pub(crate) fn parse_test_symbols(&self) -> Result<Vec<String>, ParseError> {
        let mut symbols = self.parse_all()?;
        symbols.retain(|sym| sym.contains("__ol_test_"));
        for symbol in symbols.iter_mut() {
            while symbol.starts_with("___ol_test_") {
                *symbol = symbol.strip_prefix("_").unwrap().to_string();
            }
        }
        Ok(symbols)
    }
}

#[cfg(test)]
fn parse_fixture(path: &str) -> Vec<String> {
    let data = std::fs::read(path).unwrap();
    let mut symbols = Parser::new(&data).parse_test_symbols().unwrap();
    symbols.sort_unstable();
    symbols
}

#[test]
fn test_elf() {
    let symbols = parse_fixture(crate::testing::fixture::ELF_PATH);
    assert_eq!(
        symbols,
        vec!["__ol_test_crash", "__ol_test_fail", "__ol_test_pass"]
    );
}

#[test]
fn test_mach_o() {
    let symbols = parse_fixture(crate::testing::fixture::MACH_O_PATH);
    assert_eq!(
        symbols,
        vec!["__ol_test_crash", "__ol_test_fail", "__ol_test_pass"]
    );
}

#[test]
fn test_pe() {
    let symbols = parse_fixture(crate::testing::fixture::PE_PATH);
    assert_eq!(
        symbols,
        vec!["__ol_test_crash", "__ol_test_fail", "__ol_test_pass"]
    );
}
