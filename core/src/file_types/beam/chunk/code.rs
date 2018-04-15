
pub struct CodeChunk {
    pub name: &'static str,
    pub size: u32,
    pub sub_size: u32,
    pub instruction_set: u32,
    pub opcode_max: u32,
    pub label_count: u32,
    pub function_count: u32,
    pub code: Vec<u8> // chunk_size - sub_size
}

impl CodeChunk {
    pub fn new() -> CodeChunk {
        CodeChunk {
            name: "Code",
            size: 0,
            sub_size: 0,
            instruction_set: 0,
            opcode_max: 0,
            label_count: 0,
            function_count: 0,
            code: Vec::new(),
        }
    }
}
