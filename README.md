# xch

A Rust library and CLI program to exchange the content of paths.

Current release: 0.2.0

*Note: Currently only runs on windows or on Linux using nightly rust*

## CLI

The program `xch` can be used to e**XCH**ange the content of two paths. 

### Install
`cargo install xch`

### Usage
```bash
xch first/path second/path
```

After the program call, `first/path` will point to the previous content of `second/path` and vice versa. 
This not only works for files but also for directories or one file and a directory.

All changes are made atomically, you can never observe one change without the other (e.g. 
if `first/path` points to the old content of `second/path`, `second/path` also points to the
old content of `first/path`).

## Crate

The functionality is also available as a crate. Add `xch = "0.2.0"` to your `Cargo.toml`.
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

## License

Licensed under [MIT license](./LICENSE)

## Future

Here are some missing features that will hopefully be added in the future:
- Linux support
- MacOS support
- *BSD support
