use heapless::String;
use lib_1::utils::common::convert_u16_to_string_decimal;

use super::{flash::FlashString, point::Point};
use crate::board::lcd;

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
///
/// UPDATE:
/// For memory footprint space conditions we have downgrade this canvas from "double buffer" to "single buffer".
/// Expected flickering performance is good.
/// TODO: Rename to ScreenBuffer
pub struct Canvas {
    is_initialized: bool,
    cursor_position: CursorPosition, // for screen_buffer_input
    screen_buffer_input: [u8; 80],
    //screen_buffer_output: [u8; 80],
}

impl Canvas {
    pub fn new() -> Self {
        lcd::lcd_initialize();
        Self {
            is_initialized: true,
            cursor_position: CursorPosition::new_from_point(Point::new(0, 0)),
            screen_buffer_input: [' ' as u8; 80],
            //screen_buffer_output: ['x' as u8; 80],
        }
    }

    // Base input part (These functions are actually what are producing the printing effect on buffer)

    pub fn print_char(&mut self, char: char) {
        let index = self.cursor_position.get_index();
        self.screen_buffer_input[index as usize] = char as u8; //TODO: check if this convertion is safe
        self.cursor_position.increment();
        //lcd::print_char(char);
    }

    pub fn set_cursor(&mut self, point: Point) {
        self.cursor_position.set_point(point);
        //let Point { x: col, y: row } = point;
        //lcd::setCursor(col, row);
    }

    pub fn clear(&mut self) {
        const CLEARING_CHAR: u8 = ' ' as u8;
        for x in &mut self.screen_buffer_input {
            *x = CLEARING_CHAR;
        }
        self.cursor_position.set_point(Point::new(0, 0));
        //lcd::clear();
    }

    // input part

    pub fn print(&mut self, str: &str) {
        for char in str.chars() {
            self.print_char(char)
        }
    }

    pub fn print_flash_str(&mut self, flash_string: FlashString) {
        for (char, _index) in flash_string.chars_indices() {
            self.print_char(char as char);
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
