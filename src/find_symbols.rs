use goblin::{mach::Mach, Object};

fn find_all_symbols(data: &[u8]) -> Option<Vec<String>> {
    match Object::parse(&data).ok()? {
        Object::Elf(elf) => {
            let syms = elf
                .strtab
                .to_vec()
                .ok()?
                .into_iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>();
            Some(syms)
        }
        Object::PE(pe) => {
            let syms = pe
                .exports
                .into_iter()
                .map(|export| export.name.unwrap_or_default().to_string())
                .collect::<Vec<_>>();
            Some(syms)
        }
        Object::Mach(Mach::Binary(mach)) => {
            let syms = mach
                .exports()
                .ok()?
                .into_iter()
                .map(|e| e.name)
                .collect::<Vec<_>>();
            Some(syms)
        }
        Object::Mach(Mach::Fat(mach)) => {
            println!("fat mach: {:#?}", &mach);
            None
        }
        Object::Archive(archive) => {
            println!("archive: {:#?}", &archive);
            None
        }
        Object::Unknown(magic) => {
            println!("unknown magic: {:#x}", magic);
            None
        }
    }
}

pub fn find_test_symbols(data: &[u8]) -> Option<Vec<String>> {
    let mut symbols = find_all_symbols(data)?;
    symbols.retain(|sym| sym.contains("__test_"));
    Some(symbols)
}

#[cfg(test)]
mod tests {
    use super::find_test_symbols;

    fn read_fixture(path: &str) -> Vec<String> {
        let data = std::fs::read(path).unwrap();
        let mut symbols = find_test_symbols(&data).unwrap();
        symbols.sort_unstable();
        symbols
    }

    #[test]
    fn test_elf() {
        let symbols = read_fixture("fixtures/elf.so");
        assert_eq!(symbols, vec!["__test_fail", "__test_pass"]);
    }

    #[test]
    fn test_mach_o() {
        let symbols = read_fixture("fixtures/mach-o-binary.dylib");
        assert_eq!(symbols, vec!["___test_fail", "___test_pass"]);
    }

    #[test]
    fn test_pe() {
        let symbols = read_fixture("fixtures/pe.dll");
        assert_eq!(symbols, vec!["__test_fail", "__test_pass"]);
    }
}
