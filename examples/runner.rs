use dll_ol::TestRunner;

pub fn main() {
    let paths = std::env::args().into_iter().skip(1).collect::<Vec<_>>();

    for path in paths {
        TestRunner::new(&path).unwrap();
    }
}
