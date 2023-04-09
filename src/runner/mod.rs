use crate::{
    loader::{Loader, TestFn},
    parser::Parser,
};

mod error;
use error::NewRunnerError;

#[derive(Debug)]
pub struct Runner {
    dlib_path: String,
    tests: Vec<TestFn>,
    dl: Loader,
}

impl Runner {
    pub fn new(dlib_path: &str) -> Result<Self, NewRunnerError> {
        let content = std::fs::read(dlib_path)?;
        let symbols = Parser::new(&content)
            .parse_test_symbols()
            .unwrap_or_default();
        let dl = Loader::new(dlib_path)?;

        let mut tests = vec![];
        for symbol in symbols {
            let f = dl.get_symbol(&symbol)?;
            tests.push(f);
        }

        Ok(Self {
            dlib_path: dlib_path.to_string(),
            tests,
            dl,
        })
    }
}

#[test]
fn test_new_ok() {
    crate::assertions::trigger_inclusion();

    let runner = Runner::new(crate::fixtures::FOR_CURRENT_PLATFORM).unwrap();
    assert_eq!(runner.tests.len(), 3);
}

#[test]
fn test_new_err() {
    crate::assertions::trigger_inclusion();

    let runner = Runner::new("./unknown.dylib");
    assert!(runner.is_err());
    assert_eq!(runner.unwrap_err(), NewRunnerError::NoDylib);
}
