use crate::geometry::point::Point;

/// NOTE: It is assumed that any given [`Lcd`] instance is already correctly initialized.
pub trait Lcd {
    fn clear(&self);
    fn print_u8(&self, byte: u8);
    /// Set cursor using the convention x increases to right and y increases to bottom
    /// NOTE: if argument is out of range (position is invisible) then clamp cursor in the last
    /// nearest visible possition.
    /// NOTE: Assuming (first_line, first_collum) = (0,0) => (col, line) = (x, y);
    /// TODO: Consider to make this function infalible (ie: returning perhaps `Option` or `Result` )
    fn set_cursor(&self, point: Point);
}
