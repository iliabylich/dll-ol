use crate::test_suite::{
    single_file::{FileError, SingleFile},
    Test, TestSuite,
};

pub struct MultiFile {
    pub(crate) files: Vec<SingleFile>,
}

impl MultiFile {
    pub fn new(paths: &[String]) -> Result<Self, FileError> {
        let mut files = vec![];
        for path in paths {
            let file = SingleFile::new(path)?;
            files.push(file);
        }
        Ok(Self { files })
    }
}

impl TestSuite for MultiFile {
    fn each_test<F>(&self, f: F)
    where
        F: Fn(Test),
    {
        for file in &self.files {
            file.each_test(&f);
        }
    }
}
