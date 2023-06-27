use crate::{
    dylib::{loader::Loader, parser::Parser},
    logger::{LogEvent, Logger},
    test::{Test, TestResult},
};

#[derive(Debug)]
pub(crate) struct TestFile {
    dlib_path: String,
    loader: Loader,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TestFileResult {
    pub(crate) dlib_path: String,
    pub(crate) test_results: Vec<TestResult>,
}

impl TestFile {
    pub(crate) fn new(dlib_path: String) -> Self {
        Self {
            loader: Loader::new(&dlib_path),
            dlib_path,
        }
    }

    pub(crate) fn run(self, logger: &mut Logger) -> TestFileResult {
        let symbols = Parser::new(&self.dlib_path).parse_test_symbols();

        logger.push(LogEvent::FileStarted {
            dlib_path: self.dlib_path.clone(),
            total_count: symbols.len(),
        });

        let test_results = symbols
            .into_iter()
            .map(|symbol| Test::new(symbol))
            .map(|test| test.run(&self.loader, logger))
            .collect::<Vec<_>>();

        logger.push(LogEvent::FileFinished);

        TestFileResult {
            dlib_path: self.dlib_path,
            test_results,
        }
    }
}
