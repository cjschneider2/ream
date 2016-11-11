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

pub struct Atom {
    name: String,
    len: usize,
}
