// Copyright 2017 Moritz Wanzenböck.
//
// Licensed under the MIT License <LICENSE or http://opensource.org/licenses/MIT>.
// This file may not be copied, modified, or distributed except according to those terms.

extern crate clap;
extern crate libxch;

use clap::{Arg, App};
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
use libxch::xch;

fn main() {
    let matches = App::new("xch")
        .version(VERSION)
        .author(AUTHORS)
        .about("A utility to quickly swap the content of files or directories.")
        .arg(Arg::with_name("PATH1")
            .help("One path to exchange")
            .required(true))
        .arg(Arg::with_name("PATH2")
            .help("The other path to exchange")
            .required(true))
        .get_matches();

    let path1 = matches.value_of("PATH1").expect("clap should have covered this");
    let path2 = matches.value_of("PATH2").expect("clap should have covered this");

    let exit_code = match xch(path1, path2) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Could not swap files: {}", e);
            1
        }
    };
    std::process::exit(exit_code);
}
