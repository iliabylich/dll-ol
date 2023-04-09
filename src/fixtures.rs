pub(crate) const ELF_PATH: &str = "./fixtures/elf.so";
pub(crate) const MACH_O_PATH: &str = "./fixtures/mach-o-binary.dylib";
pub(crate) const PE_PATH: &str = "./fixtures/pe.dll";

#[cfg(target_os = "linux")]
pub(crate) const FOR_CURRENT_PLATFORM: &str = ELF_PATH;
#[cfg(target_os = "macos")]
pub(crate) const FOR_CURRENT_PLATFORM: &str = MACH_O_PATH;
#[cfg(target_os = "windows")]
pub(crate) const FOR_CURRENT_PLATFORM: &str = PE_PATH;
