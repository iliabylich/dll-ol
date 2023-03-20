use dll_ol::find_test_symbols;

pub fn main() {
    let paths = std::env::args().into_iter().skip(1).collect::<Vec<_>>();

    for path in paths {
        println!("Reading fele {:?}", path);
        if let Ok(data) = std::fs::read(path) {
            println!("{:?}", find_test_symbols(&data))
        } else {
            println!("Failed to read...")
        }
    }
}
