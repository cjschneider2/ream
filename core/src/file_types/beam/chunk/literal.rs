
pub struct LiteralChunk {
    pub name: &'static str,
    pub decompressed_size: u32,
    pub data: Vec<u8>,
}

impl LiteralChunk {
    pub fn new() -> LiteralChunk {
        LiteralChunk {
            name: "LitT",
            decompressed_size: 0,
            data: Vec::new(),
        }
    }
}
