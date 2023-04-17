use dll_ol::{trigger_assertions_inclusion, TestSuite};

pub fn main() {
    trigger_assertions_inclusion();

    let paths = std::env::args().into_iter().skip(1).collect::<Vec<_>>();
    let test_suite = TestSuite::new(&paths);

    test_suite.run();
}
