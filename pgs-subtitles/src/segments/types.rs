//! Rust representation of the Presentation Graphics Stream format used for subtitles on
//! Blu-Rays.
//!
//! For more information see [this blog post][blog-post] and [US Patent 8350870B2][patent].
//!
//! [blog-post]: https://blog.thescorpius.com/index.php/2017/07/15/presentation-graphic-stream-sup-files-bluray-subtitle-format/
//! [patent]: https://patents.google.com/patent/US8350870B2/en

use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::types::util::*;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct PGSTimestamp(Duration);

impl PGSTimestamp {
    /// Construct a `PGSTimestamp` from a [`u32`] containing a raw PGS timestamp.
    pub fn from_raw_timestamp(ts: u32) -> PGSTimestamp {
        PGSTimestamp(Duration::from_nanos(u64::from(ts) * 1_000_000 / 90))
    }

    /// Access the timestamp's value as a [`Duration`].
    pub fn duration(&self) -> Duration {
        self.0
    }
}

//

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum PGSSegmentType {
    /// Presentation Composition Segment (PCS)
    PCS,

    /// Window Definition Segment (WDS)
    WDS,

    /// Palette Definition Segment (PDS)
    PDS,

    /// Object Definition Segment (ODS)
    ODS,

    /// End of Display Set Segment (END)
    END,
}

//

#[derive(Serialize, Deserialize, Debug)]
pub struct PGSHeader {
    presentation_timestamp: PGSTimestamp,
    decoding_timestamp: PGSTimestamp,
    segment_type: PGSSegmentType,
    segment_size: u16,
}

impl PGSHeader {
    pub fn new(
        presentation_timestamp: PGSTimestamp,
        decoding_timestamp: PGSTimestamp,
        segment_type: PGSSegmentType,
        segment_size: u16,
    ) -> Self {
        PGSHeader {
            presentation_timestamp,
            decoding_timestamp,
            segment_type,
            segment_size,
        }
    }

    pub fn presentation_timestamp(&self) -> PGSTimestamp {
        self.presentation_timestamp
    }

    pub fn decoding_timestamp(&self) -> PGSTimestamp {
        self.decoding_timestamp
    }

    pub fn segment_type(&self) -> PGSSegmentType {
        self.segment_type
    }

    pub fn segment_size(&self) -> u16 {
        self.segment_size
    }
}

//

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum PGSSegment {
    /// Presentation Composition Segment (PCS)
    PCS(PresentationControlSegment),

    /// Window Definition Segment (WDS)
    WDS,

    /// Palette Definition Segment (PDS)
    PDS,

    /// Object Definition Segment (ODS)
    ODS,

    /// End of Display Set Segment (END)
    END,
}

//

#[derive(Serialize, Deserialize, Debug)]
pub struct PresentationControlSegment {
    presentation_timestamp: PGSTimestamp,
    decoding_timestamp: PGSTimestamp,
    width: u16,
    height: u16,
    framerate: u8,
    composition_number: u16,
    composition_type: PCSCompositionType,
    is_palette_update_only: bool,
    palette_id: PaletteId,
    composition_objs: Vec<PCSComposition>,
}

impl PresentationControlSegment {
    pub fn new(
        presentation_timestamp: PGSTimestamp,
        decoding_timestamp: PGSTimestamp,
        width: u16,
        height: u16,
        framerate: u8,
        composition_number: u16,
        composition_type: PCSCompositionType,
        is_palette_update_only: bool,
        palette_id: PaletteId,
        composition_objs: Vec<PCSComposition>,
    ) -> Self {
        Self {
            presentation_timestamp,
            decoding_timestamp,
            width,
            height,
            framerate,
            composition_number,
            composition_type,
            is_palette_update_only,
            palette_id,
            composition_objs,
        }
    }

    pub fn presentation_timestamp(&self) -> PGSTimestamp {
        self.presentation_timestamp
    }

    pub fn decoding_timestamp(&self) -> PGSTimestamp {
        self.decoding_timestamp
    }

    pub fn width(&self) -> u16 {
        self.width
    }

    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn framerate(&self) -> u8 {
        self.framerate
    }

    pub fn composition_number(&self) -> u16 {
        self.composition_number
    }

    pub fn is_palette_update_only(&self) -> bool {
        self.is_palette_update_only
    }

    pub fn palette_id(&self) -> PaletteId {
        self.palette_id
    }

    pub fn composition_objs(&self) -> &[PCSComposition] {
        &self.composition_objs
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PCSCompositionType {
    /// This defines a new display. The Epoch Start contains all functional segments needed to display a new
    /// composition on the screen.
    EpochStart,

    /// This defines a display refresh, which is used to compose in the middle of the Epoch. It includes functional
    /// segments with new objects to be used in a new composition, replacing old objects with the same Object ID.
    AcquisitionPoint,

    /// This defines a display update, and contains only functional segments with elements that are different from the
    /// preceding composition.
    ///
    /// It’s mostly used to stop displaying objects on the screen by defining a composition with no composition objects
    /// (a value of zero in the Number of Composition Objects flag) but also used to define a new composition with new
    /// objects and objects defined since the Epoch Start.
    Normal,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PCSComposition {
    object_id: ObjectId,
    window_id: WindowId,
    object_position: Point,
    crop_window: Option<Rect>,
}

impl PCSComposition {
    pub fn new(
        object_id: ObjectId,
        window_id: WindowId,
        object_position: Point,
        crop_window: Option<Rect>,
    ) -> Self {
        Self {
            object_id,
            window_id,
            object_position,
            crop_window,
        }
    }

    pub fn object_id(&self) -> ObjectId {
        self.object_id
    }

    pub fn window_id(&self) -> WindowId {
        self.window_id
    }

    pub fn object_position(&self) -> Point {
        self.object_position
    }

    pub fn crop_window(&self) -> Option<Rect> {
        self.crop_window
    }
}
