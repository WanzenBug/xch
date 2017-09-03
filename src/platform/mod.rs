
#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use self::windows::{xch, PlatformError};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use self::linux::{xch, PlatformError};

#[cfg(not(any(windows, target_os = "linux")))]
compile_error!("libxch not supported on this platform!");
