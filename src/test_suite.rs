use crate::{file::File, file_group::FileGroup};

type TestFn = extern "C" fn() -> ();

pub struct Test {
    pub dlib_path: String,
    pub name: String,
    pub f: TestFn,
}

pub trait TestSuite {
    fn each_test<F>(&self, f: F)
    where
        F: Fn(Test);
}

impl TestSuite for File {
    fn each_test<F>(&self, f: F)
    where
        F: Fn(Test),
    {
        for (name, test) in &self.tests {
            f(Test {
                dlib_path: self.dlib_path.clone(),
                name: name.clone(),
                f: *test,
            });
        }
    }
}

impl TestSuite for FileGroup {
    fn each_test<F>(&self, f: F)
    where
        F: Fn(Test),
    {
        for file in &self.files {
            file.each_test(&f);
        }
    }
}
