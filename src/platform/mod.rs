
#[cfg_attr(windows, path="windows.rs")]
#[cfg_attr(all(target_os = "linux", feature = "nightly"), path="linux.rs")]
#[cfg_attr(all(not(windows), not(all(target_os = "linux", feature = "nightly"))), path="not_implemented.rs")]
mod platform;

pub use self::platform::{xch, PlatformError};
