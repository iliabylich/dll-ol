mod assertions;
mod backtrace;
mod dylib;
mod logger;
mod suite;
mod test;
mod test_file;

#[cfg(test)]
mod fixtures;

fn dlib_paths() -> Vec<String> {
    std::env::args().skip(1).collect::<Vec<_>>()
}

pub fn main() {
    use crate::{logger::Logger, suite::Suite};

    let _ = assertions::trigger_inclusion();

    let mut logger = Logger::new();

    Suite::new(dlib_paths()).run(&mut logger);
}

#[test]
fn test_everything() {
    use crate::{
        backtrace::Backtrace,
        logger::Logger,
        suite::{Suite, SuiteResult},
        test::TestResult,
        test_file::TestFileResult,
    };

    let _ = assertions::trigger_inclusion();

    let mut logger = Logger::new();

    let dlib_path = String::from(fixtures::FOR_CURRENT_PLATFORM);
    Suite::new(vec![dlib_path]).run(&mut logger);

    let log = logger.finish();
    assert_eq!(log.len(), 10);
    use logger::LogEvent;
    dbg!(&log);

    assert_eq!(log[0], LogEvent::SuiteStarted);
    assert_eq!(
        log[1],
        LogEvent::FileStarted {
            dlib_path: String::from(fixtures::FOR_CURRENT_PLATFORM),
            total_count: 3,
        }
    );
    assert_eq!(
        log[2],
        LogEvent::TestStarted {
            test_name: String::from("crash"),
        }
    );
    assert_eq!(log[3], LogEvent::TestSkipped);
    assert_eq!(
        log[4],
        LogEvent::TestStarted {
            test_name: String::from("fail")
        }
    );
    assert_eq!(log[5], LogEvent::TestFailed);
    assert_eq!(
        log[6],
        LogEvent::TestStarted {
            test_name: String::from("pass")
        }
    );
    assert_eq!(log[7], LogEvent::TestPassed);
    assert_eq!(log[8], LogEvent::FileFinished);
    assert_eq!(
        log[9],
        LogEvent::SuiteFinished {
            result: SuiteResult {
                file_results: vec![TestFileResult {
                    dlib_path: String::from(fixtures::FOR_CURRENT_PLATFORM),
                    test_results: vec![
                        TestResult::Skip,
                        TestResult::Failed {
                            name: String::from("fail"),
                            message: String::from("Expected 1 but got 2"),
                            backtrace: Backtrace::new()
                        },
                        TestResult::Passed
                    ]
                }]
            }
        }
    );
}
