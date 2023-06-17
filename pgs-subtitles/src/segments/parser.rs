use winnow::binary::{le_u16, le_u32};
use winnow::combinator::{dispatch, fail, success};
use winnow::prelude::*;
use winnow::sequence::preceded;
use winnow::token::any;

use crate::segments::types::*;

pub fn timestamp(input: &[u8]) -> IResult<&[u8], PGSTimestamp> {
    le_u32
        .map(PGSTimestamp::from_raw_timestamp)
        .context("PGS timestamp")
        .parse_next(input)
}

pub fn segment_type(input: &[u8]) -> IResult<&[u8], PGSSegmentType> {
    dispatch! {any;
        0x14 => success(PGSSegmentType::PDS),
        0x15 => success(PGSSegmentType::ODS),
        0x16 => success(PGSSegmentType::PCS),
        0x17 => success(PGSSegmentType::WDS),
        0x80 => success(PGSSegmentType::END),
        _    => fail::<_, PGSSegmentType, _>
    }
    .context("segment type ID")
    .parse_next(input)
}

pub fn header(input: &[u8]) -> IResult<&[u8], PGSHeader> {
    preceded("PG", (timestamp, timestamp, segment_type, le_u16))
        .map(
            |(presentation_timestamp, decoding_timestamp, segment_type, segment_size)| {
                PGSHeader::new(
                    presentation_timestamp,
                    decoding_timestamp,
                    segment_type,
                    segment_size,
                )
            },
        )
        .parse_next(input)
}
