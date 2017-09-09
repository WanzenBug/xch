
cfg_if! {
    if #[cfg(windows)] {
        mod windows;
        pub use self::windows::{xch, PlatformError};
    } else if #[cfg(all(target_os = "linux", feature = "nightly"))] {
        mod linux;
        pub use self::linux::{xch, PlatformError};
    } else {
        mod not_implemented;
        pub use self::not_implemented::{xch, PlatformError};
    }
}
