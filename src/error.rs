
error_chain!{
    foreign_links {
        Fs(::std::io::Error);
        Platform(::platform::PlatformError);
    }

    errors {
        UnsupportedOperation(t: String) {
            description("Unsupported operation")
            display("unsupported operation: {}", t)
        }

        NotImplemented {
            description("This feature is not yet implemented on your platform")
        }
    }
}