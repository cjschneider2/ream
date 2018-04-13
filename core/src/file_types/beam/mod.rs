
pub mod atom_chunk;
pub mod code_chunk;
pub mod export_chunk;
pub mod import_chunk;
pub mod string_chunk;
pub mod chunk;

pub struct Beam {
    /* Undefined chunks; Remove eventually */
    pub chunks: Vec<chunk::Chunk>,

    /* Mandatory */
    pub atoms:  atom_chunk::AtomChunk,     // Atom, AtU8
    pub code:   code_chunk::CodeChunk,     // Code
    pub string: string_chunk::StringChunk, // StrT
    pub import: import_chunk::ImportChunk, // ImpT
    pub export: export_chunk::ExportChunk, // ExpT

    /* Optional */
    // pub FunT:
    // pub LitT:
    // pub Attr:
    // pub CInf:
    // pub Line:
    // pub AtU8: It's a part of atoms; there are no non-utf8 atoms implmented
}

impl Beam {
    pub fn new() -> Beam {
        Beam {
            atoms:  atom_chunk::AtomChunk::new(),
            code:   code_chunk::CodeChunk::new(),
            string: string_chunk::StringChunk::new(),
            export: export_chunk::ExportChunk::new(),
            import: import_chunk::ImportChunk::new(),
            chunks: Vec::new(),
        }
    }
}
