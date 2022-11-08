
use core::str::FromStr;
use core::ops::Range;

use alloc::borrow::ToOwned;
use heapless::{ 
    Vec,
    String,
};
use lib_1::utils::common::convert_u16_to_string_decimal;

use crate::{menu::{point::Point, ratangular_wave::RectangularWave, canvas::Canvas}, board::keyboard::KeyCode};

use super::{edit_mode::EditMode, widget::Widget, widget::Editable, cursor::Cursor};

pub type Setter = fn(u16);
pub type Getter = fn() -> u16;

const MAX_NUMBER_OF_CHARS_IN_BUFFER: usize = 10;

pub type FieldBuffer = String<MAX_NUMBER_OF_CHARS_IN_BUFFER>;

struct EditionBuffer {
    buffer: FieldBuffer,
    cursor: Cursor,
}

impl EditionBuffer {
    pub fn new(buffer: FieldBuffer, cursor_position: usize) -> Self {
        Self {
            cursor: Cursor::new(0..buffer.len(), cursor_position),
            buffer,
        }
    }


    pub fn change_cursor_item_to(&mut self, new_char: char) -> &mut Self {
        let current_cursor = self.cursor.get_current();
        let mut s: FieldBuffer = String::new();
        for (index, current_char) in self.buffer.char_indices() {
            if index == current_cursor {
                s.push(new_char).unwrap();
            } else {
                s.push(current_char).unwrap();
            }
        }
        self.buffer = s.to_owned();
        self  
    }

    /// increment_cursor_safe
    pub fn move_cursor_right(&mut self) -> &mut Self {
        self.cursor.next();
        self
    }

    /// decrement_cursor_safe
    pub fn move_cursor_left(&mut self) -> &mut Self {
        self.cursor.previous();
        self
    }

    pub fn move_cursor_begin(&mut self) -> &mut Self {
        self.cursor.begin();
        self
    }
    
    pub fn move_cursor_end(&mut self) -> &mut Self {
        self.cursor.end();
        self
    }

    pub fn addAndMoveRight(&mut self, item: char) -> &mut Self {
        self
            .change_cursor_item_to(item)
            .move_cursor_right()
    }

}

//just a type convertion
fn convert_u16_to_FieldBuffer(data: u16, number_of_digits: usize) -> FieldBuffer {
    const blacket_char:char = '0';
    let s = convert_u16_to_string_decimal(data);
    let mut base: FieldBuffer = String::from_str(s.as_str()).unwrap();
    let mut temp: FieldBuffer = String::new();
    //leading zeros
    for _ in base.len()..number_of_digits {
        temp.push(blacket_char);
    }
    //actal number
    for char in base.chars() {
        temp.push(char);
    }
    temp
}

///NOTE: If error defaults to 0
fn convert_FieldBuffer_to_u16(data: FieldBuffer) -> u16 {
    let res = data.parse::<u16>().unwrap_or(0);
    res
}

//Make possible to edit a position of memory using Lcd display and keyboard
//esc abort edition, and enter confirm edition
pub struct Field {
    edition_buffer: EditionBuffer,
    blink: RectangularWave<u32>,
    edit_mode: EditMode,
    initial_cursor_position: usize,
    setter: Setter,
    getter: Getter,
    valid_range: Range<u16>,
    number_of_digits: usize,
}

impl Field {
    pub fn new(setter: Setter, getter: Getter, initial_cursor_position: usize, number_of_digits: usize, valid_range: Range<u16>) -> Self {
        let array = convert_u16_to_FieldBuffer(getter(), number_of_digits);
        Self {
            setter,
            getter,
            edition_buffer: EditionBuffer::new(array.clone(), initial_cursor_position),
            blink: RectangularWave::new(400,700),
            edit_mode: EditMode::new(false),
            initial_cursor_position,
            valid_range,
            number_of_digits,
        }
    }


    fn __saves_data(&mut self, data: FieldBuffer) {
        let value = convert_FieldBuffer_to_u16(data);
        let min = self.valid_range.start;
        let max = self.valid_range.end;
        let value_clamped = value.clamp(min, max);
        (self.setter)(value_clamped);
        let field_buffer = convert_u16_to_FieldBuffer(value_clamped, self.number_of_digits);
        self.edition_buffer = EditionBuffer::new(field_buffer.clone(), self.initial_cursor_position);
    }

    /// disconsider edited value and reset edition cursor
    fn __abort_edition(&mut self) {
        let previous_value = (self.getter)(); // original value
        let field_buffer = convert_u16_to_FieldBuffer(previous_value, self.number_of_digits);
        self.edition_buffer = EditionBuffer::new(field_buffer, self.initial_cursor_position);
    }
}

impl Field {

    pub fn send_key(&mut self, key: KeyCode) {     
        
        if self.is_in_edit_mode() {

            let effect = match key {
                // cancel edition
                KeyCode::KEY_ESC => {
                    self.set_edit_mode(false); // terminate edition
                    self.__abort_edition(); 
                    Some(())
                }
                
                // saves edition
                KeyCode::KEY_ENTER => {
                    self.set_edit_mode(false); // terminate edition
                    let field_buffer: FieldBuffer = self.edition_buffer.buffer.clone();
                    self.__saves_data(field_buffer);
                    Some(())
                }
                // navigation_key left and right
                KeyCode::KEY_SETA_BRANCA_ESQUERDA => { self.edition_buffer.move_cursor_left(); Some(()) }, 
                KeyCode::KEY_SETA_BRANCA_DIREITA => { self.edition_buffer.move_cursor_right(); Some(()) },
                KeyCode::KEY_DIRECIONAL_PARA_DIREITA => { self.edition_buffer.move_cursor_right(); Some(()) },
                KeyCode::KEY_DIRECIONAL_PARA_ESQUERDA => { self.edition_buffer.move_cursor_left(); Some(()) },
                // edidtion key
                KeyCode::KEY_0 => { self.edition_buffer.change_cursor_item_to('0').move_cursor_right(); Some(()) },
                KeyCode::KEY_1 => { self.edition_buffer.change_cursor_item_to('1').move_cursor_right(); Some(()) },
                KeyCode::KEY_2 => { self.edition_buffer.change_cursor_item_to('2').move_cursor_right(); Some(()) },
                KeyCode::KEY_3 => { self.edition_buffer.change_cursor_item_to('3').move_cursor_right(); Some(()) },
                KeyCode::KEY_4 => { self.edition_buffer.change_cursor_item_to('4').move_cursor_right(); Some(()) },
                KeyCode::KEY_5 => { self.edition_buffer.change_cursor_item_to('5').move_cursor_right(); Some(()) },
                KeyCode::KEY_6 => { self.edition_buffer.change_cursor_item_to('6').move_cursor_right(); Some(()) },
                KeyCode::KEY_7 => { self.edition_buffer.change_cursor_item_to('7').move_cursor_right(); Some(()) },
                KeyCode::KEY_8 => { self.edition_buffer.change_cursor_item_to('8').move_cursor_right(); Some(()) },
                KeyCode::KEY_9 => { self.edition_buffer.change_cursor_item_to('9').move_cursor_right(); Some(()) },
                KeyCode::KEY_ESC => { self.set_edit_mode(false); Some(()) },
                //everything else
                _ => { None },
            };
    
            // reset the blinker when some key is pressed makes a better visual effect
            if let Some(_) = effect {
                self.blink.reset();
            }  
        } else {
            // ignore keys
        }
    }

    pub fn update(&mut self) {
        // blinks cursor
        self.blink.update();
    }

    pub fn draw(&self, canvas: &mut Canvas, start_point: Point) {
        canvas.set_cursor(start_point);
        for (position,digit) in self.edition_buffer.buffer.char_indices() {
            let blink_char = '_';
            let mut current_char = digit.clone();
            let is_current_char_over_cursor = position == self.edition_buffer.cursor.get_current();
            let is_time_to_blink = self.blink.read() && self.is_in_edit_mode(); // do not blink if it is not in edit mode
            if is_current_char_over_cursor && is_time_to_blink {
                current_char = blink_char;
            } 
            canvas.print_char(current_char);
        };
    }
}

impl Field {
    pub fn set_edit_mode(&mut self, value: bool) {

        self.edit_mode.set_edit_mode(value);
    }

    pub fn is_in_edit_mode(&self) -> bool {
        self.edit_mode.is_in_edit_mode()
    }
}
