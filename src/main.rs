// Copyright 2017 Moritz Wanzenböck.
//
// Licensed under the MIT License <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

use libxch;

use std::env;

use libxch::{xch, xch_non_atomic};

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const PROG_INFO: &'static str = concat!("xch ", env!("CARGO_PKG_VERSION"), "\n", env!("CARGO_PKG_AUTHORS"), "\nA utility to quickly swap the content of files or directories.\n");

const USAGE: &'static str = "USAGE:
    xch [FLAGS] [--] <PATH1> <PATH2>

FLAGS:
    -h, --help          Prints help information
    -n, --non-atomic    Use non atomic exchange if atomic is not available
    -V, --version       Prints version information

ARGS:
    <PATH1>    One path to exchange
    <PATH2>    The other path to exchange";

fn main() {
    let mut args = env::args_os().skip(1);
    let mut show_help = false;
    let mut show_version = false;
    let mut non_atomic = false;
    let mut treat_as_path = false;
    let mut paths = Vec::new();

    while let Some(arg) = args.next() {
        if !treat_as_path && (arg == "-h" || arg == "--help") {
            show_help = true;
            continue;
        }
        if !treat_as_path && (arg == "-V" || arg == "--version") {
            show_version = true;
            continue;
        }
        if !treat_as_path && (arg == "-n" || arg == "--non-atomic") {
            non_atomic = true;
            continue;
        }
        if !treat_as_path && arg == "--" {
            treat_as_path = true;
            continue;
        }

        paths.push(arg);
    }

    if show_help {
        println!("{}", PROG_INFO);
        println!("{}", USAGE);
        return;
    }

    if show_version {
        println!("xch {}", VERSION);
        return;
    }

    if paths.len() < 2 {
        eprintln!("error: need exactly two path to exchange, got {} instead", paths.len());
        println!("{}", USAGE);
        ::std::process::exit(1);
    }

    if paths.len() > 2 {
        eprintln!("error: need exactly two path to exchange, got {} instead", paths.len());
        eprintln!("-> first extra argument was: {:?}", paths[2]);
        println!("{}", USAGE);
        ::std::process::exit(1);
    }

    let path1 = paths.remove(0);
    let path2 = paths.remove(0);

    let xch_result = if non_atomic {
        xch_non_atomic(path1, path2)
    } else {
        xch(path1, path2)
    };
    let exit_code = match xch_result {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("error: could not swap files: {}", e);
            1
        }
    };
    std::process::exit(exit_code);
}
