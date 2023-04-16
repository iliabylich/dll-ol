use dll_ol::{trigger_assertions_inclusion, MultiFile, TestSuite};

pub fn main() {
    trigger_assertions_inclusion();

    let paths = std::env::args().into_iter().skip(1).collect::<Vec<_>>();
    let file_group = MultiFile::new(&paths).unwrap();

    file_group.each_test(|test| {
        println!(
            "Running test {} (from {}, addr = {:?})",
            test.name, test.dlib_path, test.f
        );
    });
}
