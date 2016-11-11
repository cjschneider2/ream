/// This structure contains the options for the initialization of the REAM.
/// The approach is different from the BEAM which parses the command line options
/// as a part of the startup, but doing this make configuration if starting from
/// an already running program difficult to separate the program's options from
/// the once which should be required by REAM.

#[derive(Debug)]
pub struct Configuration {
    num_cpu: usize,
    num_schedulers: usize,
    num_dirty_schedulers: usize,
    proc_tab_size: usize,
    port_tab_size: usize,
    time_correction: bool,
    time_warp_mode: bool,
    atom_limit: usize,
    atom_char_limit: usize,
    atom_min_table_size: usize,
    atom_max_table_size: u32, // size for compatibility
    arity_max_val: u32,
    tuple_max_size: usize,
}

impl Configuration {
    pub fn defaults() -> Configuration {
        Configuration {
            num_cpu: 4,
            num_schedulers: 4,
            num_dirty_schedulers: 1,
            proc_tab_size: 2 * (1 << 15),
            port_tab_size: 1_000_000,
            time_correction: false,
            time_warp_mode: false,
            atom_limit: (1024 * 1024),
            atom_char_limit: 255,
            atom_min_table_size: 8192,
            atom_max_table_size: ::std::u32::MAX,
            arity_max_val: (1 << 24) - 1,
            tuple_max_size: (1 << 24) - 1, // techincally == arity_max_val
        }
    }
}
