use crate::test_suite::{
    test_group::{FileError, TestGroup},
    Test, Tests,
};

pub struct TestSuite {
    pub(crate) files: Vec<TestGroup>,
}

impl TestSuite {
    pub fn new(paths: &[String]) -> Result<Self, FileError> {
        let mut files = vec![];
        for path in paths {
            let file = TestGroup::new(path)?;
            files.push(file);
        }
        Ok(Self { files })
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
