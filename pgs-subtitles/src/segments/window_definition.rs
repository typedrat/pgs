use deku::ctx::Endian;
use deku::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::shared::{Rect, WindowId};

#[deku_derive(DekuRead)]
#[deku(endian = "endian", ctx = "endian: Endian")]
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct WindowDefinition {
    #[deku(temp)]
    num_windows: u8,

    #[deku(count = "num_windows")]
    pub windows: Vec<Window>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DekuRead, DekuWrite)]
#[deku(endian = "endian", ctx = "endian: Endian")]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Window {
    pub window_id: WindowId,
    pub bounds: Rect,
}
