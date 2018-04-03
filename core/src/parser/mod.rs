
use std::io::Cursor;

use byteorder::{BigEndian as BE, ReadBytesExt};

use error::Error;

use util::align_by_four;

use file_types::beam::{Beam, Chunk};

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
    let mut chunk_idx = 0;
    while pos < size_left as u64 {
        println!("\nchunk # {}", chunk_idx);


        let mut chunk = Chunk::new();
        {
            chunk.name = [cursor.read_u8()?,
                cursor.read_u8()?,
                cursor.read_u8()?,
                cursor.read_u8()?];
            chunk.size = align_by_four(cursor.read_u32::<BE>()?);
            for _ in 0..chunk.size {
                chunk.data.push(cursor.read_u8().unwrap());
            }
        }
        beam.chunks.push(chunk);

        pos = cursor.position();
        chunk_idx += 1;
    }

    Ok(beam)
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
