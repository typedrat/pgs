use deku::ctx::Endian;
use deku::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::shared::{ObjectId, PaletteId, Point, Rect, WindowId};

#[deku_derive(DekuRead)]
#[deku(endian = "endian", ctx = "endian: Endian")]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PresentationComposition {
    pub width: u16,
    pub height: u16,
    pub framerate: u8,
    pub composition_id: u16,
    pub composition_type: CompositionType,
    #[deku(bits = "1", pad_bits_after = "7")]
    pub is_palette_update_only: bool,
    pub palette_id: PaletteId,
    #[deku(temp)]
    num_objects: u8,
    #[deku(count = "num_objects")]
    pub objects: Vec<CompositionObject>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[deku(type = "u8", endian = "endian", ctx = "endian: Endian")]
pub enum CompositionType {
    Normal = 0x00,
    AcquisitionPoint = 0x40,
    EpochStart = 0x80,
    EpochContinuation = 0xC0,
}

#[deku_derive(DekuRead)]
#[deku(endian = "endian", ctx = "endian: Endian")]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct CompositionObject {
    pub object_id: ObjectId,
    pub window_id: WindowId,
    #[deku(temp, bits = "1")]
    is_cropped: bool,
    #[deku(bits = "1", pad_bits_after = "6")]
    pub is_forced: bool,
    pub position: Point,
    #[deku(cond = "*is_cropped")]
    pub crop_window: Option<Rect>,
}
