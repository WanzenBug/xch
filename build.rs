#[cfg(target_os = "linux")]
mod platform {
    use cc;

    pub fn build() {
        cc::Build::new()
            .file("src/platform/linux.c")
            .warnings(true)
            .warnings_into_errors(true)
            .static_flag(true)
            .compile("linux_xch_syscall");
    }
}

#[cfg(not(target_os = "linux"))]
mod platform {
    pub fn build () { }
}

fn main() {
    platform::build()
}
