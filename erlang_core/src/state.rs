use config::Configuration;

pub struct State {
    conf: Configuration,
}

pub fn new_from_config(conf: Configuration) -> State {
    State {
        conf: conf,
    }
}
