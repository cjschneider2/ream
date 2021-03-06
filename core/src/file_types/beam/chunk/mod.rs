
pub mod atom;
pub mod code;
pub mod export;
pub mod import;
pub mod string;
pub mod local_fn;
pub mod compiler_info;
pub mod literal;

#[derive(Debug)]
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
    DebugInfo,         // Dbgi
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
