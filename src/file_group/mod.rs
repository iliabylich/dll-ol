use crate::file::{File, FileError};

pub struct FileGroup {
    pub(crate) files: Vec<File>,
}

impl FileGroup {
    pub fn new(paths: &[String]) -> Result<Self, FileError> {
        let mut files = vec![];
        for path in paths {
            let file = File::new(path)?;
            files.push(file);
        }
        Ok(Self { files })
    }
}
