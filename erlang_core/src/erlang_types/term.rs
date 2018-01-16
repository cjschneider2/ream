/// HEADER representation:
///
///	aaaaaaaaaaaaaaaaaaaaaaaaaatttt00	arity:26, tag:4
///
/// HEADER tags:
///
///		0000	ARITYVAL
///		0001	BINARY_AGGREGATE				|
///		001x	BIGNUM with sign bit			|
///		0100	REF								|
///		0101	FUN				                | THINGS
///		0110	FLONUM				            |
///		0111	EXPORT							|
///		1000	REFC_BINARY	    |		        |
///		1001	HEAP_BINARY	    | BINARIES	    |
///		1010	SUB_BINARY	    |		        |
///		1011	Not used; see comment below
///		1100	EXTERNAL_PID  |					|
///		1101	EXTERNAL_PORT | EXTERNAL THINGS |
///		1110	EXTERNAL_REF  |					|
///		1111	MAP
///
/// COMMENTS:
///
/// - The tag is zero for arityval and non-zero for thing headers.
/// - A single bit differentiates between positive and negative bignums.
/// - If more tags are needed, the REF and and EXTERNAL_REF tags could probably
///   be combined to one tag.

use std::collections::HashMap;

/// NOTE: This probably won't ever change
const TERM_VERSION_MAGIC: u8 = 131; // was `130` in Erlang 4.2

enum ExternalTags {
    // Types
    SmallInteger = 97, // 'a'
    Integer = 98, // 'b'
    Float = 99, // 'c'
    Atom = 100, // 'd'
    SmallAtom = 115, // 's'
    Reference = 101, // 'e'
    NewReference = 114, // 'r'
    NewerReference = 90, // 'Z'
    Port = 102, // 'f'
    NewPort = 121, // 'y'
    NewFloat = 70, // 'F'
    Pid = 103, // 'g'
    NewPid = 88, // 'X'
    SmallTuple = 104, // 'h'
    LargeTuple = 105, // 'i'
    Nil = 106, // 'j'
    String = 107, // 'k'
    List = 108, // 'l'
    Binary = 109, // 'm'
    BitBinary = 77, // 'M'
    SmallBig = 110, // 'n'
    LargeBig = 111, // 'o'
    NewFun = 112, // 'p'
    Export = 113, // 'q'
    Map = 116, // 't'
    Fun = 117, // 'u'
    AtomUtf8 = 118, // 'v'
    SmallAtomUtf8 = 119, // 'w'
    // Others
    DistHeader = 68, // 'D'
    AtomCacheRef = 82, // 'R'
    AtomInternalRef2 = 73, // 'I'
    AtomInternalRef3 = 75, // 'K'
    BinaryInternalRef = 74, // 'J'
    BitBinaryInternalRef = 76, // 'L'
    Compressed = 80, // 'P'
}

struct Atom {
}

struct AtomCache {
    header_size: usize,
    size: usize,
    long_atoms: usize,
    cache: HashMap<usize, Atom>
}
