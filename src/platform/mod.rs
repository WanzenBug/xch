
#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use self::windows::{xch, PlatformError};

#[cfg(not(windows))]
compile_error!("libxch not supported on this platform!");