use dll_ol::Runner;

pub fn main() {
    let paths = std::env::args().into_iter().skip(1).collect::<Vec<_>>();

    for path in paths {
        Runner::new(&path).unwrap();
    }
}
