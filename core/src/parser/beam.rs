
use std::io::Cursor;

use byteorder::{BigEndian as BE, ReadBytesExt};

use error::Error;

use util::align_by_four;

use file_types::beam::{Beam, Chunk, ChunkType};

const IFF_HEADER: u32 = 0x464F5231; // "FOR1" as u32::BE
const FORM_TYPE: u32 = 0x4245414D; // "BEAM" as u32::BE

pub fn load_beam_from_bytes(bytes: &[u8]) -> Result<Beam, Error> {
    let bytes_len = bytes.len();
    let mut cursor = Cursor::new(bytes);

    // Read BEAM header
    let header: u32 = cursor.read_u32::<BE>()?;
    if IFF_HEADER != header {
        return Err(Error::Str("Header err: no `FOR1` match".into()));
    }

    let size_left: usize = cursor.read_u32::<BE>()? as usize;
    if size_left != bytes_len - 8 {
        println!("byte len: {}\nReported: {}", bytes_len - 8, size_left);
        return Err(Error::Str("Header err: Reported size != measured".into()));
    }

    let form: u32 = cursor.read_u32::<BE>()?;
    if FORM_TYPE != form {
        return Err(Error::Str("Header err: no `BEAM` match".into()));
    }

    let mut beam = Beam::new();

    // Read Beam Chunks
    let mut pos = cursor.position();
    while pos < size_left as u64 {
        let mut chunk = Chunk::new();
        {
            match parse_chunk_name_as_str(&mut cursor)? {
                "Atom" => parse_atom_table(&mut cursor),
                "AtU8" => chunk.kind = ChunkType::AtomU8,
                "Code" => chunk.kind = ChunkType::Code,
                "StrT" => chunk.kind = ChunkType::StringTable,
                "ImpT" => chunk.kind = ChunkType::ImportTable,
                "ExpT" => chunk.kind = ChunkType::ExportTable,
                "FunT" => chunk.kind = ChunkType::FunctionTable,
                "LitT" => chunk.kind = ChunkType::LiteralTable,
                "LocT" => chunk.kind = ChunkType::LocalFunTable,
                "Attr" => chunk.kind = ChunkType::Attributes,
                "CInf" => chunk.kind = ChunkType::CompilerInfoTable,
                "Dbgi" => chunk.kind = ChunkType::DebugInfo,
                "Line" => chunk.kind = ChunkType::Line,
                unknown => {
                    println!("Unknown BEAM Chunk type: {}", unknown);
                    panic!();
                },
            }
            chunk.size = align_by_four(cursor.read_u32::<BE>()?);
            for _ in 0..chunk.size {
                chunk.data.push(cursor.read_u8().unwrap());
            }
        }
        beam.chunks.push(chunk);

        pos = cursor.position();
    }

    Ok(beam)
}

pub fn parse_chunk_name_as_str(cursor: &mut Cursor<&[u8]>) -> Result<&str, Error> {
    use std::str::from_utf8;
    let name: [u8; 4] = [cursor.read_u8()?,
                         cursor.read_u8()?,
                         cursor.read_u8()?,
                         cursor.read_u8()?];
    from_utf8(&name)
}

fn parse_atom_table(cursor: &mut Cursor<&[u8]>) {

}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_FILE: &[u8] = include_bytes!("../../../tests/parser_tests/erlang.beam");

    #[test]
    fn test_load_erlang_beam () {
        println!();
        load_beam_from_bytes(TEST_FILE).unwrap();
    }
}
