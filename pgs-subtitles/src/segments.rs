use deku::ctx::Endian;
use deku::prelude::*;
use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod end_of_display_set;
pub mod object_definition;
pub mod palette_definition;
pub mod presentation_composition;
pub mod shared;
pub mod window_definition;

pub use end_of_display_set::*;
pub use object_definition::*;
pub use palette_definition::*;
pub use presentation_composition::*;
pub use window_definition::*;

#[derive(Debug, Clone, DekuRead)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
#[deku(
    ctx = "endian: Endian, segment_type: u8, segment_size: u16",
    id = "segment_type",
    endian = "endian"
)]
pub enum SegmentPayload {
    #[deku(id = "0x14")]
    PDS(#[deku(ctx = "segment_size")] PaletteDefinition),

    #[deku(id = "0x15")]
    ODS(ObjectDefinition),

    #[deku(id = "0x16")]
    PCS(PresentationComposition),

    #[deku(id = "0x17")]
    WDS(WindowDefinition),

    #[deku(id = "0x80")]
    END(EndOfDisplaySet),
}

#[deku_derive(DekuRead)]
#[deku(endian = "big", magic = b"PG")]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Segment {
    pub presentation_timestamp: Timestamp,
    pub decoding_timestamp: Timestamp,

    #[deku(temp)]
    segment_type: u8,
    #[deku(temp)]
    segment_length: u16,

    #[deku(ctx = "*segment_type, *segment_length")]
    pub payload: SegmentPayload,
}

//

#[deku_derive(DekuRead)]
#[deku(endian = "endian", ctx = "endian: Endian")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[repr(transparent)]
pub struct Timestamp {
    raw_value: u32,
}

impl Timestamp {
    pub fn new(raw_value: u32) -> Self {
        Self { raw_value }
    }

    pub fn raw_value(&self) -> u32 {
        self.raw_value
    }
}

impl Display for Timestamp {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let all_millis = self.raw_value / 90;
        let (all_seconds, millis) = (all_millis / 1000, all_millis % 1000);
        let (all_minutes, seconds) = (all_seconds / 60, all_seconds % 60);
        let (hours, minutes) = (all_minutes / 60, all_minutes % 60);

        formatter.write_fmt(format_args!("{hours}h {minutes}m {seconds}.{millis}s"))
    }
}

//

use thiserror::Error;

#[derive(Error, Debug)]
pub enum PGSParseError {
    #[error("internal parse error")]
    InternalParseError(#[from] DekuError),

    #[error("incomplete read! {0} bytes remaining")]
    IncompleteRead(usize),
}

#[derive(DekuRead)]
struct Segments {
    #[deku(bits_read = "deku::rest.len()")]
    contents: Vec<Segment>,
}

pub fn parse_segments(bytes: &[u8]) -> Result<Vec<Segment>, DekuError> {
    let (_rest, segments) = Segments::from_bytes((bytes, 0))?;

    Ok(segments.contents)
}
