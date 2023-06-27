use crate::{
    backtrace::Backtrace,
    logger::{LogEvent, Logger},
    test::TestResult,
    test_file::{TestFile, TestFileResult},
};

pub(crate) struct Suite {
    dlib_paths: Vec<String>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct SuiteResult {
    pub(crate) file_results: Vec<TestFileResult>,
}

impl Suite {
    pub(crate) fn new(dlib_paths: Vec<String>) -> Self {
        Self { dlib_paths }
    }

    pub(crate) fn run(self, logger: &mut Logger) {
        logger.push(LogEvent::SuiteStarted);

        let file_results = self
            .dlib_paths
            .into_iter()
            .map(|dlib_path| TestFile::new(dlib_path))
            .map(|test_file| test_file.run(logger))
            .collect::<Vec<_>>();

        logger.push(LogEvent::SuiteFinished {
            result: SuiteResult { file_results },
        })
    }
}

pub(crate) struct Failure {
    pub(crate) dlib_path: String,
    pub(crate) test_name: String,
    pub(crate) message: String,
    pub(crate) backtrace: Backtrace,
}

impl SuiteResult {
    pub(crate) fn into_failures(self) -> Vec<Failure> {
        let mut result = vec![];

        for file_result in self.file_results {
            for test_result in file_result.test_results {
                match test_result {
                    TestResult::Failed {
                        name,
                        message,
                        backtrace,
                    } => result.push(Failure {
                        dlib_path: file_result.dlib_path.clone(),
                        test_name: name,
                        message,
                        backtrace,
                    }),
                    _ => {}
                }
            }
        }

        result
    }
}
