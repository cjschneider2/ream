pub mod chunk;

use self::chunk::{
    atom::AtomChunk,
    code::CodeChunk,
    string::StringChunk,
    import::ImportChunk,
    export::ExportChunk,
    local_fn::LocalFnChunk,
    compiler_info::CompilerInfoChunk,
    literal::LiteralChunk,
};

pub struct Beam {
    /* Undefined chunks; Remove eventually */
    pub chunks: Vec<chunk::Chunk>,

    /* Mandatory */
    pub atoms:  AtomChunk,   // Atom, AtU8
    pub code:   CodeChunk,   // Code
    pub string: StringChunk, // StrT
    pub import: ImportChunk, // ImpT
    pub export: ExportChunk, // ExpT

    /* Optional */
    pub local_fn: LocalFnChunk,
    // pub FunT:
    pub literal: LiteralChunk, // LitT
    // pub Attr:
    pub compiler_info: CompilerInfoChunk, // CInf
    // pub Line:
    // pub AtU8: It's a part of atoms; there are no non-utf8 atoms implmented
}

impl Beam {
    pub fn new() -> Beam {
        Beam {
            /* Mandatory */
            atoms:  AtomChunk::new(),
            code:   CodeChunk::new(),
            string: StringChunk::new(),
            export: ExportChunk::new(),
            import: ImportChunk::new(),
            /* Optional */
            local_fn: LocalFnChunk::new(),
            compiler_info: CompilerInfoChunk::new(),
            literal: LiteralChunk::new(),
            /* Undefined */
            chunks: Vec::new(),
        }
    }
}
