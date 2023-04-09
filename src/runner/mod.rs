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
        let content = std::fs::read(dlib_path).map_err(|_err| NewRunnerError::NoDylib)?;
        let symbols = Parser::new(&content)
            .parse_test_symbols()
            .unwrap_or_default();
        let dl = Loader::new(dlib_path).map_err(|err| NewRunnerError::FailedToLoadDyLib(err))?;

        let mut tests = vec![];
        for symbol in symbols {
            eprintln!("Loading {:?}", symbol);
            let f = dl
                .get_symbol(&symbol)
                .map_err(|err| NewRunnerError::MalformedDylib(err))?;
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

    let _runner = Runner::new(crate::fixtures::FOR_CURRENT_PLATFORM).unwrap();

    let runner = Runner::new("./unknown.dylib");
    assert!(runner.is_err());
    assert_eq!(runner.unwrap_err(), NewRunnerError::NoDylib);
}
