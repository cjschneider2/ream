
use std::io::Cursor;
use std::str::from_utf8;

use byteorder::{BigEndian as BE, ReadBytesExt};

use error::Error;

use util::align_by_four;

use file_types::beam::Beam;
use file_types::beam::chunk::{Chunk, ChunkType};
//use file_types::beam::atom_chunk::AtomChunk;

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

        let name: [u8; 4] = [cursor.read_u8()?,
            cursor.read_u8()?,
            cursor.read_u8()?,
            cursor.read_u8()?];
        let name = from_utf8(&name).unwrap();

        match name {
            "Atom" | "AtU8" => parse_atom_chunk(&mut cursor, &mut beam)?,
            "Code" => parse_code_chunk(&mut cursor, &mut beam)?,
            "StrT" => parse_string_chunk(&mut cursor, &mut beam)?,
            "ImpT" => parse_import_chunk(&mut cursor, &mut beam)?,
            "ExpT" => parse_export_chunk(&mut cursor, &mut beam)?,
            "FunT" => parse_chunk(&mut cursor, &mut beam, ChunkType::FunctionTable)?,
            "LitT" => parse_chunk(&mut cursor, &mut beam, ChunkType::LiteralTable)?,
            "LocT" => parse_chunk(&mut cursor, &mut beam, ChunkType::LocalFunTable)?,
            "Attr" => parse_chunk(&mut cursor, &mut beam, ChunkType::Attributes)?,
            "CInf" => parse_chunk(&mut cursor, &mut beam, ChunkType::CompilerInfoTable)?,
            "Dbgi" => parse_chunk(&mut cursor, &mut beam, ChunkType::DebugInfo)?,
            "Line" => parse_chunk(&mut cursor, &mut beam, ChunkType::Line)?,
            unknown => {
                println!("Unknown BEAM Chunk type: {}", unknown);
                panic!();
            },
        }

        pos = cursor.position();
    }

    Ok(beam)
}

fn parse_chunk(
    cursor: &mut Cursor<&[u8]>,
    beam: &mut Beam,
    kind: ChunkType
) -> Result<(), Error> {
    let mut chunk = Chunk::new();
    chunk.kind = kind;
    let chunk_size = cursor.read_u32::<BE>()?;
    chunk.size = chunk_size;
    for _ in 0..chunk.size {
        chunk.data.push(cursor.read_u8().unwrap());
    }
    beam.chunks.push(chunk);

    let padding = align_by_four(chunk_size) - chunk_size;
    for _ in 0..padding {
        assert_eq!(0, cursor.read_u8()?);
    }

    Ok(())
}

fn parse_atom_chunk(
    cursor: &mut Cursor<&[u8]>,
    beam: &mut Beam
) -> Result<(), Error> {
    let chunk_size = cursor.read_u32::<BE>()?;
    let atom_count = cursor.read_u32::<BE>()?;
    let mut total_len = 4u32;
    for _ in 0..atom_count {
        let len = cursor.read_u8()?;
        total_len += len as u32 + 1;
        let mut str = Vec::<u8>::new();
        for _ in 0..len {
            str.push(cursor.read_u8()?);
        }
        beam.atoms.atoms.push(from_utf8(&str)?.to_owned());
    }

    let padding = align_by_four(chunk_size) - chunk_size;
    for _ in 0..padding {
        assert_eq!(0, cursor.read_u8()?);
    }

    assert_eq!(total_len, chunk_size);
    Ok(())
}

fn parse_code_chunk(
    cursor: &mut Cursor<&[u8]>,
    beam: &mut Beam
) -> Result<(), Error> {
    let chunk_size = cursor.read_u32::<BE>()?;
    let header_size = cursor.read_u32::<BE>()?;
    {
        beam.code.instruction_set = cursor.read_u32::<BE>()?; // AKA: "version"
        beam.code.opcode_max = cursor.read_u32::<BE>()?;
        beam.code.label_count = cursor.read_u32::<BE>()?;
        beam.code.function_count = cursor.read_u32::<BE>()?;
    }
    assert_eq!(header_size, 16);

    for _ in 0..(chunk_size - header_size - 4) {
        beam.code.code.push(cursor.read_u8()?);
    }

    let padding = align_by_four(chunk_size) - chunk_size;
    for _ in 0..padding {
        assert_eq!(0, cursor.read_u8()?);
    }

    Ok(())
}

fn parse_import_chunk(
    cursor: &mut Cursor<&[u8]>,
    beam: &mut Beam
) -> Result<(), Error> {
    let chunk_size = cursor.read_u32::<BE>()?;

    let import_count = cursor.read_u32::<BE>()?;

    for _ in 0..import_count {
        beam.import.push_import(
            cursor.read_u32::<BE>()?,
            cursor.read_u32::<BE>()?,
            cursor.read_u32::<BE>()?);
    }

    let padding = align_by_four(chunk_size) - chunk_size;
    for _ in 0..padding {
        assert_eq!(0, cursor.read_u8()?);
    }

    Ok(())
}

fn parse_export_chunk(
    cursor: &mut Cursor<&[u8]>,
    beam: &mut Beam
) -> Result<(), Error> {
    let chunk_size = cursor.read_u32::<BE>()?;

    let export_count = cursor.read_u32::<BE>()?;

    for _ in 0..export_count {
        beam.export.push_export(
            cursor.read_u32::<BE>()?,
            cursor.read_u32::<BE>()?,
            cursor.read_u32::<BE>()?);
    }

    let padding = align_by_four(chunk_size) - chunk_size;
    for _ in 0..padding {
        assert_eq!(0, cursor.read_u8()?);
    }

    Ok(())
}

fn parse_string_chunk(
    cursor: &mut Cursor<&[u8]>,
    beam: &mut Beam
) -> Result<(), Error> {
    let chunk_size = cursor.read_u32::<BE>()?;

    for _ in 0..chunk_size {
        beam.string.data.push(cursor.read_u8()?);
    }

    let padding = align_by_four(chunk_size) - chunk_size;
    for _ in 0..padding {
        assert_eq!(0, cursor.read_u8()?);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_erlang_beam () {
        const TEST_FILE: &[u8] = include_bytes!("../../../tests/parser_tests/erlang.beam");
        println!("\nParsing beam file: {:?}", "../../../tests/parser_tests/erlang.beam");
        load_beam_from_bytes(TEST_FILE).unwrap();
    }
}
