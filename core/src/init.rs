use config::Configuration;
use state::State;
use state::new_from_config;

pub fn init(conf: Configuration) -> State {
    let state = new_from_config(conf);

    // NOTE: The BEAM initializes things in the following order:
    //        - bif unique
    //        - monitors
    //        - time(time correction, time warp mode)
    //        - sys. common misc.
    //        - process(num_cpu, proc_tab_size)
    //        - scheduling(num_sch., num_sec. online,
    //                     num_dirty_sch., num dirty online)
    //        - late init time sup
    //        - cpu topology
    //        - gc
    //        - alloc late init
    //        - trace
    //        - bits
    //        - code ix
    //        - fun table
    //        - atom table
    //        - export table
    //        - module table
    //        - register table
    //        - message
    //        - bif info
    //        - ddll
    //        - emulator
    //        - ptab
    //        - binary
    //        - bp
    //        - db(db_spin_count)
    //        - node tables
    //        - dist
    //        - drv thr
    //        - async
    //        - io(port table size)
    //        - load
    //        - bif
    //        - bif_checksum
    //        - bif_binary
    //        - bif_re
    //        - unicode
    //        - external
    //        - map
    //        - beam.rs bif load
    //        - delay trap
    //        - late init process
    //        - packet parser
    //        - nif
    //        - msacc

    state
}
