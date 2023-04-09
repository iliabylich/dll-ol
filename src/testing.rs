pub(crate) mod fixture {
    #[cfg(target_os = "linux")]
    pub(crate) const PATH: &str = "./fixtures/elf.so";
    #[cfg(target_os = "macos")]
    pub(crate) const PATH: &str = "./fixtures/mach-o-binary.dylib";
    #[cfg(target_os = "windows")]
    pub(crate) const PATH: &str = "./fixtures/pe.dll";
}
