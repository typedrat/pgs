use deku::ctx::Endian;
use deku::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::shared::{ColorId, PaletteId};

#[deku_derive(DekuRead)]
#[deku(endian = "endian", ctx = "endian: Endian, size: u16")]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PaletteDefinition {
    pub id: PaletteId,
    pub palette_version: u8,
    #[deku(bytes_read = "(size - 2)")]
    pub palette_entries: Vec<PaletteEntry>,
}

#[deku_derive(DekuRead)]
#[deku(endian = "endian", ctx = "endian: Endian")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PaletteEntry {
    id: ColorId,

    luma: u8,
    chroma_red: u8,
    chroma_blue: u8,
    alpha: u8,
}
