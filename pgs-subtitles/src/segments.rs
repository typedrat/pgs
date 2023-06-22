use winnow::combinator::repeat;
use winnow::error::Error;
use winnow::prelude::*;
use winnow::stream::Bytes;

use self::types::PGSSegment;

pub mod parser;
pub mod types;

pub fn parse_segments(input: &[u8]) -> Result<Vec<PGSSegment>, winnow::error::Error<String>> {
    repeat(1.., parser::segment)
        .parse(Bytes::new(input))
        .map_err(|Error { input, kind }| Error {
            input: input.to_string(),
            kind,
        })
}
