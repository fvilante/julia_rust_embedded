//! Adapts the hardware driver to the [`Lcd`] trait

use crate::geometry::point::Point;

use super::{driver, interface::Lcd};

/// Basically a wrapper over the hitach hardware lcd [`driver`]
pub struct LcdHardware40x2;

impl LcdHardware40x2 {
    pub fn new() -> Self {
        // initialize hardware lcd
        driver::lcd_initialize();
        LcdHardware40x2
    }
}

impl Lcd for LcdHardware40x2 {
    const MAX_COLS: u8 = 40;
    const MAX_ROWS: u8 = 2;
    fn print_u8(&self, byte: u8) {
        todo!()
    }

    fn set_cursor(&self, point: Point) {
        let Point { x: col, y: row } = point;
        driver::set_cursor(col, row)
    }
}
