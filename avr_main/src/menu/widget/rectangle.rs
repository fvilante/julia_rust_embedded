use core::cell::Cell;

use crate::menu::point::Point;

/// Rectangle internal spec
/// X increases from left to right, Y increases from up to  bottom.
struct RectInternal {
    // uppper-left corner (inclusive)
    x_min: u8,
    y_max: u8,
    // bottom-right corner (exclusive)
    x_max: u8,
    y_min: u8,
}

/// Represents a geometric rectangle
pub struct Rectangle {
    coords: Cell<RectInternal>,
}

impl Rectangle {
    /// X increases from left to right, Y increases from up to  bottom.
    pub fn new_from_raw(x_min: u8, y_max: u8, x_max: u8, y_min: u8) -> Self {
        Self {
            coords: Cell::new(RectInternal {
                x_min,
                y_min,
                x_max,
                y_max,
            }),
        }
    }

    /// X increases from left to right, Y increases from up to  bottom.
    pub fn new(left_upper: Point, right_bottom: Point) -> Self {
        let Point { x: x_min, y: y_max } = left_upper;
        let Point { x: x_max, y: y_min } = right_bottom;
        Self::new_from_raw(x_min, y_max, x_max, y_min)
    }

    /// Crates a rectangle with zero_area positioned in origin
    pub fn new_empty() -> Self {
        Self::new_from_raw(0, 0, 0, 0)
    }
}
