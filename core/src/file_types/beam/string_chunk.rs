
pub struct StringChunk {
    pub name: &'static str,
    pub data: Vec<u8>,
}

impl StringChunk {
    pub fn new() -> StringChunk {
        StringChunk {
            name: "StrT",
            data: Vec::new(),
        }
    }
}
