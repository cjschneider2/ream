// Program modules
mod util;

// standard library imports
// extern library imports
// local imports

// Library API Structures

/// This structure contains the options for the initialization of the REAM.
/// The approach is different from the BEAM which parses the command line options
/// as a part of the startup, but doing this make configuration if starting from
/// an already running program difficult to separate the program's options from
/// the once which should be required by REAM.
#[derive(Debug)]
pub struct Configuration {
    foo: usize,
}

impl Configuration {
    pub fn defaults() -> Configuration {
        Configuration {
            foo: 0
        }
    }
}


// Library API Functions
pub fn erl_start(conf: Configuration) {
    // Debug
    //println!("Using the following configuration:");
    println!("{:?}", conf);

    unimplemented!();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
