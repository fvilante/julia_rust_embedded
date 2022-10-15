
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