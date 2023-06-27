use crate::suite::SuiteResult;

#[derive(Debug, Clone)]
pub(crate) struct Logger {
    events: Vec<LogEvent>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LogEvent {
    SuiteStarted,
    SuiteFinished {
        result: SuiteResult,
    },
    FileStarted {
        dlib_path: String,
        total_count: usize,
    },
    FileFinished,
    TestStarted {
        test_name: String,
    },
    TestPassed,
    TestFailed,
    TestSkipped,
}

impl Logger {
    const GREEN: &str = "\x1b[1;32m";
    const RED: &str = "\x1b[0;31m";
    const YELLOW: &str = "\x1b[0;33m";
    const RESET_COLOR: &str = "\x1b[0m";

    pub(crate) fn new() -> Self {
        Self { events: vec![] }
    }

    pub(crate) fn push(&mut self, event: LogEvent) {
        self.events.push(event.clone());
        Logger::print(event);
    }

    #[cfg(test)]
    pub(crate) fn finish(self) -> Vec<LogEvent> {
        self.events
    }

    fn print(event: LogEvent) {
        match event {
            LogEvent::SuiteStarted => {
                println!("\nStarting...");
            }
            LogEvent::SuiteFinished { result } => {
                println!("\nFinished.\n");

                let failures = result.into_failures();

                if failures.is_empty() {
                    println!("All tests passed");
                } else {
                    println!("{} tests failed:\n", failures.len());
                    for failure in failures {
                        println!("{} (in {})", failure.test_name, failure.dlib_path);
                        println!("{}\n", failure.message);
                        println!("    Backtrace:");
                        for (idx, frame) in failure.backtrace.symbols().iter().enumerate() {
                            println!("{:>4}: {}", idx, frame.symbol_name());
                            if let Some(file_line_col) = frame.file_line_col() {
                                println!("             {}", file_line_col);
                            }
                        }
                    }
                }
            }
            LogEvent::FileStarted {
                dlib_path,
                total_count,
            } => {
                println!("\nRunning {} tests from {}", total_count, dlib_path);
            }
            LogEvent::FileFinished => {}
            LogEvent::TestStarted { test_name } => {
                print!("test {} ... ", test_name);
            }
            LogEvent::TestPassed => {
                println!("{}ok{}", Self::GREEN, Self::RESET_COLOR);
            }
            LogEvent::TestFailed => {
                println!("{}FAILED{}", Self::RED, Self::RESET_COLOR);
            }
            LogEvent::TestSkipped => {
                println!("{}FAILED{}", Self::YELLOW, Self::RESET_COLOR);
            }
        }
    }
}
