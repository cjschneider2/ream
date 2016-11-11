use config::Configuration;
use state::State;
use state::new_from_config;

pub fn init(conf: Configuration) -> State {
    let mut state = new_from_config(conf);
    state
}
