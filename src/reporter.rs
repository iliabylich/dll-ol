use backtrace::{Backtrace, BacktraceSymbol};

use crate::{context::Context, test::TestName};

struct Failure {
    dlib_path: String,
    test_name: TestName,
    message: String,
    backtrace: Backtrace,
}

impl Failure {
    fn user_backtrace(&self) -> Vec<BacktraceSymbol> {
        self.backtrace
            .frames()
            .iter()
            .flat_map(|f| f.symbols())
            .filter(|sym| {
                if let Some(filename) = sym.filename() {
                    let ext = filename
                        .extension()
                        .and_then(|s| s.to_str())
                        .unwrap_or_default();
                    if ext == "rs" {
                        // Rust, ignore
                        false
                    } else {
                        // Non-Rust file with debug symbols
                        true
                    }
                } else {
                    // Non-Rust file without debug symbols
                    true
                }
            })
            .cloned()
            .collect::<Vec<_>>()
    }
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
                eprintln!("    {}", failure.message);
                eprintln!("    Backtrace:");
                for frame in failure.user_backtrace() {
                    eprintln!("      {:?}", frame);
                }
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

    pub(crate) fn test_failed(&mut self, message: String, backtrace: Backtrace) {
        let test = Context::current_test().unwrap();
        eprintln!("{}FAILED{}", RED, RESET_COLOR);

        self.failures.push(Failure {
            dlib_path: test.dlib_path.clone(),
            test_name: test.name.clone(),
            message,
            backtrace,
        })
    }
}
