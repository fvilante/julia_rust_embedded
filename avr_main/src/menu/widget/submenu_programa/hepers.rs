/// Helper type to represent each lines of the 40x2 LCD display.
///
/// You may consider this type just to avoid to cast the two lcd lines direct to a `u8` type.
#[derive(PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum LcdLine {
    Line0 = 0,
    Line1 = 1,
}

impl LcdLine {
    pub fn iterator() -> impl Iterator<Item = LcdLine> {
        [LcdLine::Line0, LcdLine::Line1].iter().copied()
    }
}

impl From<u8> for LcdLine {
    fn from(value: u8) -> Self {
        match value {
            0 => LcdLine::Line0,
            1 => LcdLine::Line1,
            _ => LcdLine::Line0, // default
        }
    }
}
