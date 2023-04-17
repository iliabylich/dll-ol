use crate::{context::Context, test::TestName};

struct Failure {
    dlib_path: String,
    test_name: TestName,
    message: String,
}

#[derive(Default)]
pub(crate) struct Reporter {
    failures: Vec<Failure>,
}

const GREEN: &str = "\x1b[1;32m";
const RED: &str = "\x1b[0;31m";
const RESET_COLOR: &str = "\x1b[0m";

impl Reporter {
    pub(crate) fn suite_started(&mut self) {
        eprintln!("\nStarting...");
    }

    pub(crate) fn suite_finished(&mut self) {
        eprintln!("\nFinished.\n");

        if self.failures.is_empty() {
            eprintln!("All tests passed");
        } else {
            eprintln!("{} tests failed:\n", self.failures.len());
            for failure in &mut self.failures {
                eprintln!(
                    "  {} (in {})",
                    failure.test_name.pretty(),
                    failure.dlib_path
                );
                eprintln!("    {}\n", failure.message);
            }
        }
    }

    pub(crate) fn test_group_started(&mut self) {
        let test_group = Context::current_test_group().unwrap();

        eprintln!(
            "\nRunning {} tests from {}",
            test_group.tests_count(),
            test_group.name()
        );
    }

    pub(crate) fn test_group_finished(&mut self) {}

    pub(crate) fn test_started(&mut self) {
        let test = Context::current_test().unwrap();

        eprint!("test {} ... ", test.name.pretty());
    }

    pub(crate) fn test_passed(&mut self) {
        let _test = Context::current_test().unwrap();
        eprintln!("{}ok{}", GREEN, RESET_COLOR);
    }

    pub(crate) fn test_failed(&mut self, message: String) {
        let test = Context::current_test().unwrap();
        eprintln!("{}FAILED{}", RED, RESET_COLOR);

        self.failures.push(Failure {
            dlib_path: test.dlib_path.clone(),
            test_name: test.name.clone(),
            message,
        })
    }
}
