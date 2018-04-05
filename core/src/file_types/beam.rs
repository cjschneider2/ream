
pub enum ChunkType {
    Unknown,
    Atom,              // Atom
    AtomU8,            // AtU8
    Code,              // Code
    StringTable,       // StrT
    ImportTable,       // ImpT
    ExportTable,       // ExpT
    FunctionTable,     // FunT
    LiteralTable,      // LitT
    LocalFunTable,     // LocT
    CompilerInfoTable, // CInf
    Attributes,        // Attr
    DebugInfo,         // Dbgi // TODO: Check...
    Line,              // Line
}

pub struct Chunk {
    pub kind: ChunkType,
    pub name: [u8; 4],
    pub size: u32,
    pub data: Vec<u8>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            kind: ChunkType::Unknown,
            name: [0u8; 4],
            size: 0,
            data: Vec::new(),
        }
    }
}

pub struct AtomChunk {
    name: &'static str,
    size: usize,
    count: usize,
    atoms: Vec<String>
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
