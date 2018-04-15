
pub struct CompilerInfoChunk {
    pub name: &'static str,
    pub data: Vec<u8>,
}

impl CompilerInfoChunk {
    pub fn new() -> CompilerInfoChunk {
        CompilerInfoChunk {
            name: "CInf",
            data: Vec::new(),
        }
    }
}
