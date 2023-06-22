use deku::ctx::Endian;
use deku::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, DekuRead, DekuWrite)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[deku(endian = "endian", ctx = "endian: Endian")]
#[repr(transparent)]
pub struct ObjectId {
    pub(crate) raw_id: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, DekuRead, DekuWrite)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[deku(endian = "endian", ctx = "endian: Endian")]
#[repr(transparent)]
pub struct WindowId {
    pub(crate) raw_id: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, DekuRead, DekuWrite)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[deku(endian = "endian", ctx = "endian: Endian")]
#[repr(transparent)]
pub struct PaletteId {
    pub(crate) raw_id: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, DekuRead, DekuWrite)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[deku(endian = "endian", ctx = "endian: Endian")]
#[repr(transparent)]
pub struct ColorId {
    pub(crate) raw_id: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, DekuRead, DekuWrite)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, DekuRead, DekuWrite)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[deku(endian = "endian", ctx = "endian: Endian")]
pub struct Rect {
    pub origin_x: u16,
    pub origin_y: u16,
    pub width: u16,
    pub height: u16,
}
