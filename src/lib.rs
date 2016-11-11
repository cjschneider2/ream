// Program modules
mod util;
mod init;
mod config;
mod state;
mod bif;
mod atom;
mod term;

// standard library imports
// extern library imports
// local imports
use init::init;

// Exported Library API Structures
pub use config::Configuration;

// Exported Library API Functions
pub fn erl_start(conf: Configuration) {
    // Debug
    println!("🕵 : Using the following configuration:");
    println!("{:#?}", conf);

    let mut state = init(conf);

    unimplemented!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
