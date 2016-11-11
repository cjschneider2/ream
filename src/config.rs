/// This structure contains the options for the initialization of the REAM.
/// The approach is different from the BEAM which parses the command line options
/// as a part of the startup, but doing this make configuration if starting from
/// an already running program difficult to separate the program's options from
/// the once which should be required by REAM.

#[derive(Debug)]
pub struct Configuration {
    ncpu: usize,
    proc_tab_size: usize,
    port_tab_size: usize,
    time_correction: bool,
    time_warp_mode: bool,
}

impl Configuration {
    pub fn defaults() -> Configuration {
        Configuration {
            ncpu: 4,
            proc_tab_size: 2 * (1 << 15),
            port_tab_size: 1_000_000,
            time_correction: false,
            time_warp_mode: false,
        }
    }
}
