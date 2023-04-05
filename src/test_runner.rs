use crate::{
    dl::{Dl, TestFn},
    find_test_symbols,
};

#[derive(Debug)]
pub struct TestRunner<'a> {
    dlib_path: &'a str,
    tests: Vec<TestFn>,
    dl: Dl,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NoDylib,
    FailedToLoadDyLib(String),
    MalformedDylib(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl<'a> TestRunner<'a> {
    pub fn new(dlib_path: &'a str) -> Result<Self, Error> {
        let symbols = find_test_symbols(&std::fs::read(dlib_path).map_err(|_err| Error::NoDylib)?)
            .unwrap_or_default();
        let dl = Dl::new(dlib_path).map_err(|err| Error::FailedToLoadDyLib(err))?;

        let mut tests = vec![];
        for symbol in symbols {
            eprintln!("Loading {:?}", symbol);
            let f = dl
                .get_symbol(&symbol)
                .map_err(|err| Error::MalformedDylib(err))?;
            tests.push(f);
        }

        Ok(Self {
            dlib_path,
            tests,
            dl,
        })
    }
}

#[test]
fn test_new_ok() {
    let _runner = TestRunner::new("./fixtures/mach-o-binary.dylib").unwrap();

    let runner = TestRunner::new("./unknown.dylib");
    assert!(runner.is_err());
    assert_eq!(runner.unwrap_err(), Error::NoDylib);
}
