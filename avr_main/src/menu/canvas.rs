use crate::board::lcd;
use super::point::Point;


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
        Self {
            point,
        }
    }

    fn from_index(index: usize) -> Self {
        let col: u8 = (index % 40).try_into()./*clamp(0,79).*/unwrap_or(0);
        let row: u8 = (index / 40).try_into()./*clamp(0,1).*/unwrap_or(0);
        let point = Point::new(col, row);
        Self {
            point,
        }
    }
    
    fn get_index(&self ) -> usize {
        let Point{x:col, y:row} = self.point;
        let index = col + (40*row);
        index.into()
    }

    fn set_point(&mut self, point: Point) -> &mut Self {
        self.point = point;
        self
    }

    fn increment(&mut self) -> &mut Self {
        let mut index = self.get_index();
        index += 1;
        let new_point = Self::from_index(index.clamp(0, 79)).point ;
        self.point = new_point;
        self
    }
}

pub struct Canvas {
    is_initialized: bool,
    cursor_position: CursorPosition, // for screen_buffer_input
    screen_buffer_input: [u8; 80],
    screen_buffer_output: [u8; 80],
}

impl Canvas  {

    pub fn new() -> Self {
        lcd::lcd_initialize();
        Self {
            is_initialized: true,
            cursor_position: CursorPosition::new_from_point(Point::new(0,0)),
            screen_buffer_input: [' ' as u8; 80],
            screen_buffer_output: ['x' as u8; 80],
        }
    }

    // input part

    pub fn print_char(&mut self, char: char) {
        let index = self.cursor_position.get_index();
        self.screen_buffer_input[index as usize] = char as u8; //TODO: check if this convertion is safe
        self.cursor_position.increment();
        //lcd::print_char(char);
    }

    //fn print_flash_str<const SIZE: usize>(&mut self, prog_mem_pointer: &PmString<SIZE>) {
    //    let s = FlashString::new(prog_mem_pointer);
    //    for char in s.to_string().chars() {
    //        self.print_char(char);
    //    }
    //}


    pub fn set_cursor(&mut self, point: Point) {
        self.cursor_position.set_point(point);
        //lcd::setCursor(col, row);
    }

    pub fn clear(&mut self) {
        self.screen_buffer_input = [' ' as u8; 80];
        self.cursor_position.set_point(Point::new(0,0));
        //lcd::clear();
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