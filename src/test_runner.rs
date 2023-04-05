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

#[cfg(test)]
mod tests {
    use super::{Error, TestRunner};

    #[cfg(target_os = "linux")]
    const FIXTURE: &str = "./fixtures/elf.so";
    #[cfg(target_os = "macos")]
    const FIXTURE: &str = "./fixtures/mach-o-binary.dylib";
    #[cfg(target_os = "windows")]
    const FIXTURE: &str = "./fixtures/pe.dll";

    #[test]
    fn test_new_ok() {
        crate::assertions::trigger_inclusion();

        let _runner = TestRunner::new(FIXTURE).unwrap();

        let runner = TestRunner::new("./unknown.dylib");
        assert!(runner.is_err());
        assert_eq!(runner.unwrap_err(), Error::NoDylib);
    }
}
