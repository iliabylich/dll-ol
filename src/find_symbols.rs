use goblin::Object;

fn find_all_symbols(data: &[u8]) -> Option<Vec<&str>> {
    match Object::parse(&data).ok()? {
        Object::Elf(elf) => elf.strtab.to_vec().ok(),
        Object::PE(pe) => {
            println!("pe: {:#?}", &pe);
            None
        }
        Object::Mach(mach) => {
            println!("mach: {:#?}", &mach);
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

pub fn find_test_symbols(data: &[u8]) -> Option<Vec<&str>> {
    let mut symbols = find_all_symbols(data)?;
    symbols.retain(|sym| sym.starts_with("__test_"));
    Some(symbols)
}

#[test]
fn test_elf() {
    let data = std::fs::read("fixtures/elf.so").unwrap();
    let mut symbols = find_test_symbols(&data).unwrap();
    symbols.sort_unstable();
    assert_eq!(symbols, vec!["__test_fail", "__test_pass"]);
}
