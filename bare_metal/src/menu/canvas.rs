use lib_1::utils::numerical::convert_u16_to_string_decimal;

use crate::board::lcd;
use crate::geometry::point::Point;
use crate::printable::Printable;

///TODO: Reimplement it using [`Cursor`]'s type
///TODO: Test this type because maybe something in it is not working
struct CursorPosition {
    point: Point,
}

impl CursorPosition {
    fn new(point: Point) -> Self {
        Self { point }
    }

    fn from_index(index: u8) -> Self {
        let col: u8 = index % 40;
        let row: u8 = index / 40;
        let point = Point::new(col, row);
        Self { point }
    }

    fn get_index(&self) -> u8 {
        let Point { x: col, y: row } = self.point;
        let index = col + (40 * row);
        index
    }

    /// Set cursor in the specified point position
    fn set(&mut self, point: Point) -> &mut Self {
        self.point = point;
        self
    }

    fn increment(&mut self) -> &mut Self {
        let mut index = self.get_index();
        index += 1;
        let new_point = Self::from_index(index.clamp(0, 79)).point;
        self.point = new_point;
        self
    }
}

/// A memory representation of the LCD display.
///
/// Its function is to make possible do cache displayed information reducing
/// screen flackering. You decide how many frames per second you want to send this to screen through the method [`render`]
/// TODO: Rename to ScreenBuffer
pub struct Canvas {
    cursor_position: CursorPosition, // for screen_buffer_input
    screen_buffer_input: [u8; 80],
}

impl Canvas {
    pub fn new() -> Self {
        lcd::lcd_initialize();
        Self {
            cursor_position: CursorPosition::new(Point::new(0, 0)),
            screen_buffer_input: [' ' as u8; 80],
        }
    }

    /// Prints one single char on screen and increments cursor to the right (wrapping)
    pub fn print_char(&mut self, char: char) {
        let index = self.cursor_position.get_index();
        self.screen_buffer_input[index as usize] = char as u8; //TODO: check if this convertion is safe
        self.cursor_position.increment();
    }

    pub fn print_u8(&mut self, data: u8) {
        self.print_char(data as char)
    }

    /// Places cursor at the given position
    pub fn set_cursor(&mut self, point: Point) {
        self.cursor_position.set(point);
    }

    /// Clear all canvas
    pub fn clear(&mut self) {
        const CLEARING_CHAR: u8 = ' ' as u8;
        self.screen_buffer_input.fill(CLEARING_CHAR);
        self.cursor_position.set(Point::new(0, 0));
    }

    pub fn print(&mut self, data_to_print: impl Printable) {
        for byte in data_to_print.into_iter() {
            self.print_u8(byte);
        }
    }

    pub fn print_u16(&mut self, value: u16) {
        let string = convert_u16_to_string_decimal(value);
        for current_char in string.chars() {
            self.print_char(current_char)
        }
    }

    // output part

    /// The purpose of this routine is to avoid unecessary writings to LCD.
    /// It swaps two lcd buffers: The output_buffer represents current state of lcd and
    /// input_buffer represent the desired state of lcd
    pub fn render(&mut self) {
        // The current implementation of this function is very! very! simplified, it may be improved later
        lcd::set_cursor(0, 0);
        for byte in self.screen_buffer_input {
            lcd::print_u8(byte);
        }
    }
}
