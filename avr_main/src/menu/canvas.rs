use heapless::String;

use super::{flash::FlashString, point::Point};
use crate::{board::lcd, utils::generic_string::GenericString};

///TODO: Reimplement it using [`Cursor`]'s type
struct CursorPosition {
    point: Point,
}

impl CursorPosition {
    fn new() -> Self {
        Self {
            point: Point::new(0, 0),
        }
    }

    fn new_from_point(point: Point) -> Self {
        Self { point }
    }

    fn from_index(index: usize) -> Self {
        let col: u8 = (index % 40).try_into()./*clamp(0,79).*/unwrap_or(0);
        let row: u8 = (index / 40).try_into()./*clamp(0,1).*/unwrap_or(0);
        let point = Point::new(col, row);
        Self { point }
    }

    fn get_index(&self) -> usize {
        let Point { x: col, y: row } = self.point;
        let index = col + (40 * row);
        index.into()
    }

    fn set_point(&mut self, point: Point) -> &mut Self {
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
///
/// For more see: [double buffer](https://en.wikipedia.org/wiki/Multiple_buffering#Double_buffering_in_computer_graphics)
pub struct Canvas {
    is_initialized: bool,
    cursor_position: CursorPosition, // for screen_buffer_input
    screen_buffer_input: [u8; 80],
    screen_buffer_output: [u8; 80],
}

impl Canvas {
    pub fn new() -> Self {
        lcd::lcd_initialize();
        Self {
            is_initialized: true,
            cursor_position: CursorPosition::new_from_point(Point::new(0, 0)),
            screen_buffer_input: [' ' as u8; 80],
            screen_buffer_output: ['x' as u8; 80],
        }
    }

    // input part

    pub fn print(&mut self, str: &str) {
        for char in str.chars() {
            self.print_char(char)
        }
    }

    pub fn print_char(&mut self, char: char) {
        let index = self.cursor_position.get_index();
        self.screen_buffer_input[index as usize] = char as u8; //TODO: check if this convertion is safe
        self.cursor_position.increment();
        //lcd::print_char(char);
    }

    pub fn print_flash_str(&mut self, flash_string: FlashString) {
        for (char, _index) in flash_string.chars_indices() {
            self.print_char(char as char);
        }
    }

    pub fn set_cursor(&mut self, point: Point) {
        self.cursor_position.set_point(point);
        //lcd::setCursor(col, row);
    }

    pub fn clear(&mut self) {
        self.screen_buffer_input = [' ' as u8; 80];
        self.cursor_position.set_point(Point::new(0, 0));
        //lcd::clear();
    }

    // Attention: this function is designed to be used with 'String' from the 'heapless' library
    pub fn print_string<const N: usize>(&mut self, string: String<N>) {
        for char in string.chars() {
            self.print_char(char);
        }
    }

    pub fn print_xy(&mut self, point: Point, generic_string: GenericString) {
        self.set_cursor(point);
        self.print_generic_string(generic_string)
    }

    pub fn print_generic_string(&mut self, string: GenericString) {
        for byte in string.iter() {
            self.print_char(byte as char)
        }
    }

    // output part

    /// The purpose of this routine is to avoid unecessary writings to LCD.
    /// It swaps two lcd buffers: The output_buffer represents current state of lcd and
    /// input_buffer represent the desired state of lcd
    pub fn render(&mut self) {
        /// The current implementation of this function is very! very! simplified, it may be improved later    
        lcd::setCursor(0, 0);
        for byte in self.screen_buffer_input {
            lcd::print_u8(byte);
        }
    }
}
