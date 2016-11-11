extern crate ream;

use std::env::args;

use ream::erl_start;
use ream::Configuration;

fn parse_arg(arg: String) {
    println!("\t{}", arg);
}

fn main() {

    // Parse the configuration command line arguments
    println!("🕵 : Command line args");
    let argv = args();
    for arg in argv {
        parse_arg(arg);
    }

    // Start the emulator
    let conf = Configuration::defaults();
    erl_start(conf);
}
