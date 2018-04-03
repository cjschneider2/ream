
pub struct Chunk {
    pub name: [u8; 4],
    pub size: u32,
    pub data: Vec<u8>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            name: [0u8; 4],
            size: 0,
            data: Vec::new(),
        }
    }

    pub fn name_as_str(&self) -> &str {
        use std::str::from_utf8;
        from_utf8(&self.name).unwrap()
    }
}

pub struct Beam {
    pub chunks: Vec<Chunk>,
}

impl Beam {
    pub fn new() -> Beam {
        Beam {
            chunks: Vec::new(),
        }
    }
}
