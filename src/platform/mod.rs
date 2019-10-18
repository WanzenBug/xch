
#[cfg_attr(target_os = "windows", path="windows.rs")]
#[cfg_attr(target_os = "linux", path="linux.rs")]
#[cfg_attr(all(not(target_os = "windows"), not(target_os = "linux")), path="not_implemented.rs")]
mod platform;

pub use self::platform::{xch, PlatformError};
