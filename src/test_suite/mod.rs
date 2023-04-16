pub struct Test {
    pub dlib_path: String,
    pub name: String,
    pub f: extern "C" fn() -> (),
}

pub trait TestSuite {
    fn each_test<F>(&self, f: F)
    where
        F: Fn(Test);
}

mod single_file;

mod multi_file;
pub use multi_file::MultiFile;
