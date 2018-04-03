//! This module describes the Erlang `Atom` datatype. From the Erlang
//! documentation atoms are described as:
//!     An atom is a literal, a constant with name.
//!     An atom is to be enclosed in single quotes (') if it does not begin
//!     with a lower-case letter or if it contains other characters than
//!     alphanumeric characters, underscore (_), or @.
//! Although atoms in Erlang are generally written in the Latin-1 character set
//! unicode characters are technically allowed. This implmenmtation force the
//! use of UTF-8 based unicode. There is no other format; if the latin-1 format
//! is requested it will be computed at runtime.
//!

use std::collections::HashSet;

//use error::Error;

const MAX_ATOM_CHARACTERS: usize = 255;
const MAX_ATOM_SIZE_LIMIT: usize = (4 * MAX_ATOM_CHARACTERS);
const ATOM_LIMIT: usize = (1024 * 1024);
const MIN_ATOM_TABLE_SIZE: usize = 8192;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Atom {
    name: String,
    len: usize,
}

pub struct AtomTable {
    table: HashSet<Atom>,
}

impl AtomTable {
    pub fn new() -> AtomTable {
        AtomTable {
            table: HashSet::new(),
        }
    }

    pub fn table_elements(&self) -> usize {
        self.table.len()
    }

    pub fn table_size(&self) -> usize {
        self.table.iter()
            .fold(0, |acc, ref entry | {acc + entry.len})
    }

    pub fn put(&mut self, atom: Atom) {
        let _res: bool = self.table.insert(atom);
    }

    pub fn dump_atoms(&self) -> Vec<String> {
        self.table.iter()
            .map(|ref entry| entry.name.clone() )
            .collect()
    }
}