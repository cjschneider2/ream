#![allow(dead_code)]

// External Modules
extern crate byteorder;

// Program modules
mod util;
mod error;
mod init;
mod config;
mod state;
mod bif;
mod erlang_types;
mod parser;
mod file_types;

use init::init;

pub use config::Configuration;

// Exported Library API Functions
pub fn erl_start(conf: Configuration) {
    // Debug
    println!("Using the following configuration:");
    println!("{:#?}", conf);

    let _state = init(conf);

    unimplemented!();
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//     }
// }
