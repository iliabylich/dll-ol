use crate::{
    reporter::Reporter,
    test_suite::{test_group::TestGroup, Test, Tests},
};

pub struct TestSuite {
    pub(crate) files: Vec<TestGroup>,
}

impl TestSuite {
    pub fn new(paths: &[String]) -> Self {
        let mut files = vec![];
        for path in paths {
            let file = TestGroup::new(path);
            files.push(file);
        }
        Self { files }
    }

    pub fn run(&self) {
        Reporter::report_suite_started();
        for file in &self.files {
            file.run();
        }
        Reporter::report_suite_finished();
    }
}

impl Tests for TestSuite {
    fn tests(&self) -> Vec<Test> {
        let mut tests = vec![];
        for file in &self.files {
            tests.extend(file.tests());
        }
        tests
    }
}
