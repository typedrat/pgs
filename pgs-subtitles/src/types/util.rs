use serde::{Deserialize, Serialize};

/// A 16-bit point in 2D space.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Point {
    pub x: u16,
    pub y: u16,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Rect {
    left: u16,
    right: u16,
    top: u16,
    bottom: u16,
}

impl Rect {
    pub fn new(x_offset: u16, y_offset: u16, width: u16, height: u16) -> Self {
        Self {
            left: x_offset,
            right: x_offset + width,
            top: y_offset,
            bottom: y_offset + height,
        }
    }

    pub fn width(&self) -> u16 {
        self.right - self.left
    }

    pub fn height(&self) -> u16 {
        self.bottom - self.top
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct ObjectId(u16);

impl ObjectId {
    pub fn new(id: u16) -> Self {
        Self(id)
    }

    pub fn raw_value(&self) -> u16 {
        self.0
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct WindowId(u8);

impl WindowId {
    pub fn new(id: u8) -> Self {
        Self(id)
    }

    pub fn raw_value(&self) -> u8 {
        self.0
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct PaletteId(u8);

impl PaletteId {
    pub fn new(id: u8) -> Self {
        Self(id)
    }

    pub fn raw_value(&self) -> u8 {
        self.0
    }
}
