use crate::{
    context::Context,
    formatter::Formatter,
    test::Test,
    test_suite::{TestGroup, TestSuite},
};

pub struct StdoutFormatter;

const GREEN: &str = "\x1b[1;32m";
const RED: &str = "\x1b[0;31m";
const RESET_COLOR: &str = "\x1b[0m";

impl Formatter for StdoutFormatter {
    fn suite_started(&self, _suite: &TestSuite) {
        println!("\nStarting...");
    }

    fn suite_finished(&self, _suite: &TestSuite) {
        println!("\nFinished.\n");
        let failures = Context::failures();

        if failures.is_empty() {
            println!("All tests passed");
        } else {
            println!("{} tests failed:\n", failures.len());
            for failure in failures {
                println!("{} (in {})", failure.test_name.pretty(), failure.dlib_path);
                println!("{}\n", failure.message);
                println!("    Backtrace:");
                for (idx, frame) in failure.user_backtrace().iter().enumerate() {
                    println!("{:>4}: {}", idx, frame.symbol_name());
                    if let Some(file_line_col) = frame.file_line_col() {
                        println!("             {}", file_line_col);
                    }
                }
            }
        }
    }

    fn group_started(&self, group: &TestGroup) {
        println!(
            "\nRunning {} tests from {}",
            group.tests_count(),
            group.name()
        );
    }

    fn group_finished(&self, _group: &TestGroup) {}

    fn test_started(&self, test: &Test) {
        print!("test {} ... ", test.name.pretty());
    }

    fn test_passed(&self, _test: &Test) {
        println!("{}ok{}", GREEN, RESET_COLOR);
    }

    fn test_failed(&self, _test: &Test, _message: String) {
        println!("{}FAILED{}", RED, RESET_COLOR);
    }
}
