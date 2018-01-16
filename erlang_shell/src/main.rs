extern crate erlang_core;
extern crate clap;

use clap::{App};

use erlang_core::erl_start;
use erlang_core::Configuration;

fn main() {

    // Parse the configuration command line arguments
    let _matches =
        App::new("Erlang Shell")
            .version("0.0.1")
            .about("... it's the erlang shell...")
            .get_matches();

    // Start the emulator
    let conf = Configuration::defaults();
    erl_start(conf);
}
