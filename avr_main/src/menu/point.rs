#[derive(Copy, Clone)]
pub struct Point<T = u8> {
    pub x: T,
    pub y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

pub struct Point1d<T = u8> {
    pub pos: T,
}

impl<T> Point1d<T> {
    pub fn new(pos: T) -> Self {
        Self { pos }
    }
}
