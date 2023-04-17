#[derive(Debug, Clone)]
pub struct Test {
    pub dlib_path: String,
    pub name: String,
    pub f: extern "C" fn() -> (),
}
