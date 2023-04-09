use crate::{
    loader::{Loader, TestFn},
    parser::Parser,
};
use std::collections::HashMap;

mod error;
use error::NewRunnerError;

#[derive(Debug)]
pub struct Runner {
    dlib_path: String,
    tests: HashMap<String, TestFn>,
    dl: Loader,
}

impl Runner {
    pub fn new(dlib_path: &str) -> Result<Self, NewRunnerError> {
        let content = std::fs::read(dlib_path)?;
        let symbols = Parser::new(&content)
            .parse_test_symbols()
            .unwrap_or_default();
        let dl = Loader::new(dlib_path)?;

        let mut tests = HashMap::new();
        for symbol in symbols {
            let f = dl.get_symbol(&symbol)?;
            tests.insert(symbol, f);
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

    let mut keys = runner.tests.keys().collect::<Vec<_>>();
    keys.sort_unstable();
    assert_eq!(
        keys,
        vec!["__ol_test_crash", "__ol_test_fail", "__ol_test_pass"]
    );
}

#[test]
fn test_new_err() {
    crate::assertions::trigger_inclusion();

    let runner = Runner::new("./unknown.dylib");
    assert!(runner.is_err());
    assert_eq!(runner.unwrap_err(), NewRunnerError::NoDylib);
}
