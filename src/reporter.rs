use crate::{
    context::Context,
    test::{TestName, CURRENT as CURRENT_TEST},
};
use std::cell::RefCell;

struct Failure {
    dlib_path: String,
    test_name: TestName,
    message: String,
}

#[derive(Default)]
pub(crate) struct Reporter {
    failures: Vec<Failure>,
}

thread_local! {
    pub(crate) static INSTANCE: RefCell<Reporter> = RefCell::new(Reporter::default());
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

        INSTANCE.with(|reporter| {
            let reporter = reporter.borrow();
            let failures = reporter.failures.as_slice();

            if failures.is_empty() {
                eprintln!("All tests passed");
            } else {
                eprintln!("{} tests failed:\n", failures.len());
                for failure in failures {
                    eprintln!(
                        "  {} (in {})",
                        failure.test_name.pretty(),
                        failure.dlib_path
                    );
                    eprintln!("    {}\n", failure.message);
                }
            }
        })
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
        CURRENT_TEST.with(|current_test| {
            if let Some(test) = current_test.borrow_mut().as_mut() {
                if test.state.set_passed() {
                    eprintln!("{}ok{}", GREEN, RESET_COLOR);
                }
            }
        });
    }

    pub(crate) fn test_failed(&mut self, message: String) {
        let mut dlib_path = String::new();
        let mut test_name = TestName::default();

        CURRENT_TEST.with(|current_test| {
            if let Some(test) = current_test.borrow_mut().as_mut() {
                dlib_path = test.dlib_path.clone();
                test_name = test.name.clone();

                if test.state.set_failed() {
                    eprintln!("{}FAILED{}", RED, RESET_COLOR);
                }
            }
        });

        INSTANCE.with(|reporter| {
            let failure = Failure {
                dlib_path,
                test_name,
                message,
            };
            reporter.borrow_mut().failures.push(failure)
        })
    }
}
