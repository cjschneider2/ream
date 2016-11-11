// HEADER representation:
//
//	aaaaaaaaaaaaaaaaaaaaaaaaaatttt00	arity:26, tag:4
//
// HEADER tags:
//
//		0000	ARITYVAL
//		0001	BINARY_AGGREGATE				|
//		001x	BIGNUM with sign bit			|
//		0100	REF								|
//		0101	FUN				                | THINGS
//		0110	FLONUM				            |
//		0111	EXPORT							|
//		1000	REFC_BINARY	    |		        |
//		1001	HEAP_BINARY	    | BINARIES	    |
//		1010	SUB_BINARY	    |		        |
//		1011	Not used; see comment below
//		1100	EXTERNAL_PID  |					|
//		1101	EXTERNAL_PORT | EXTERNAL THINGS |
//		1110	EXTERNAL_REF  |					|
//		1111	MAP
//
// COMMENTS:
//
// - The tag is zero for arityval and non-zero for thing headers.
// - A single bit differentiates between positive and negative bignums.
// - If more tags are needed, the REF and and EXTERNAL_REF tags could probably
//   be combined to one tag.

pub struct Term {
    tag: u32,
}
