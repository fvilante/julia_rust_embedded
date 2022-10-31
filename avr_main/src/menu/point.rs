
#[derive(Copy, Clone)]
pub struct Point {
    pub x: u8,
    pub y: u8,
}

impl Point {
    pub fn new(x: u8, y: u8) -> Self {
        Self {
            x,
            y,
        }
    }
}

pub struct Point1d {
    pub pos: u8,
}

impl Point1d {
    pub fn new(pos: u8) -> Self {
        Self {
            pos,
        }
    }
}