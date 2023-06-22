use deku::ctx::Endian;
use deku::prelude::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[deku_derive(DekuRead)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
#[deku(ctx = "_: Endian")]
pub struct EndOfDisplaySet;
