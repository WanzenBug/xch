# xch

A Rust library and CLI program to exchange the content of paths. If possible, this try
to change contents atomically.

Current release: 0.2.1

***Note**: Currently only supports atomic exchange on Windows or on Linux using nightly rust*

## CLI

The program `xch` can be used to e**XCH**ange the content of two paths. 

### Install
`cargo install xch`

### Usage
```
USAGE:
    xch [FLAGS] <PATH1> <PATH2>

FLAGS:
    -h, --help          Prints help information
    -n, --non-atomic    Use non atomic exchange if atomic is not available
    -V, --version       Prints version information

ARGS:
    <PATH1>    One path to exchange
    <PATH2>    The other path to exchange

```

After the program call, `first/path` will point to the previous content of `second/path` and vice versa. 
This not only works for files but also for directories or one file and a directory.

By default all changes are made atomically, you can never observe one change without the other (e.g. 
if `first/path` points to the old content of `second/path`, `second/path` also points to the
old content of `first/path`). This only works on Windows and when using the `nightly` feature 
on Linux (`cargo install xch --features nightly` when using the nightly toolchain).

On other platforms such as any BSD or Linux using the stable toolchain, you always need to specify
`--non-atomic` to get results.

## Crate

The functionality is also available as a crate. Add `xch = "0.2.1"` to your `Cargo.toml`.
Then you need to import the crate to your code
```Rust
extern crate libxch;
```
Then you can start exchanging the content of files
```Rust
if let Err(e) = libxch::xch("file1", "path/to/file2") {
    // Error handling here
}
```
This is is the equivalent of running `xch` in default mode, i.e. it only works on Windows and Linux.
use `libxch::xch_non_atomic("file1", "path/to/file2")` to get a portable, but non-atomic exchange.

[Documentation](https://docs.rs/xch)

## License

Licensed under [MIT license](./LICENSE)

## Future

Here are some missing features that will hopefully be added in the future:
- MacOS atomic exchange using the `ExchangeData` syscall.
- Better error messages
- More options.
