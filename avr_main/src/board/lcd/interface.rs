use crate::geometry::{point::Point, rectangle::Rectangle};

/// NOTE: It is assumed that any given [`Lcd`] instance is already correctly initialized.
pub trait Lcd {
    fn print_u8(&self, byte: u8);
    /// Set cursor using the convention x increases to right and y increases to bottom
    /// NOTE: if argument is out of range (position is invisible) then clamp cursor in the last
    /// nearest visible possition.
    /// NOTE: Assuming (first_line, first_collum) = (0,0) => (col, line) = (x, y);
    /// TODO: Consider to make this function infalible (ie: returning perhaps `Option` or `Result` )
    fn set_cursor(&self, point: Point);
    /// Total number of collums of the display
    const MAX_COLS: u8;
    /// Total number of rows of the display
    const MAX_ROWS: u8;
    fn print(&self, data_to_print: impl IntoIterator<Item = u8>) {
        for byte in data_to_print {
            self.print_u8(byte)
        }
    }
}
