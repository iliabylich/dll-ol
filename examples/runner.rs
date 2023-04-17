use dll_ol::{trigger_assertions_inclusion, TestSuite, Tests};

pub fn main() {
    trigger_assertions_inclusion();

    let paths = std::env::args().into_iter().skip(1).collect::<Vec<_>>();
    let file_group = TestSuite::new(&paths);

    for test in file_group.tests() {
        let name = test.name.clone();

        println!(
            "---- Running test {} (from {}, addr = {:?})",
            test.name, test.dlib_path, test.f
        );

        (test.f)();

        println!("---- Test {} finished", name);
    }
}
