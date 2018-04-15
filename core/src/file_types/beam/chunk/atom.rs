
pub struct AtomChunk {
    pub name: &'static str,
    pub atoms: Vec<String>
}

impl AtomChunk {
    pub fn new() -> AtomChunk {
        AtomChunk {
            name: "Atom",
            atoms: Vec::new(),
        }
    }
}

