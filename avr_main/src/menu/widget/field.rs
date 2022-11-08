
use core::str::{FromStr, CharIndices};
use core::ops::Range;

use alloc::borrow::ToOwned;
use heapless::{ 
    Vec,
    String,
};
use lib_1::utils::common::convert_u16_to_string_decimal;

use crate::{menu::{point::Point, ratangular_wave::RectangularWave, canvas::Canvas, accessor::{Accessor, AccessorEnum}}, board::keyboard::KeyCode};

use super::{edit_mode::EditMode, widget::Widget, widget::Editable, cursor::Cursor};


const MAX_NUMBER_OF_CHARS_IN_BUFFER: usize = 10;

pub type FieldBuffer = String<MAX_NUMBER_OF_CHARS_IN_BUFFER>;

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

/// TODO: rename to NavigationString (?!)
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

    /// TODO: rename to change_cursor_char
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

struct Numerical {
    edition_buffer: EditionBuffer
} 

impl Numerical {
    pub fn new(edition_buffer: EditionBuffer) -> Self {
        Self {
            edition_buffer,
        }
    }

    pub fn to_u16(&self) -> u16 {
        let value = convert_FieldBuffer_to_u16(self.edition_buffer.buffer.clone());
        value
    }

    pub fn get_current_cursor_index(&self) -> usize {
        self.edition_buffer.cursor.get_current()
    }
}

impl Numerical {
    pub fn save_edition(&mut self, mut accessor: Accessor<u16>) {
        accessor.set(self.to_u16())
    }

    pub fn char_indices(&self) -> CharIndices {
        self.edition_buffer.buffer.char_indices()
    }


}

impl Numerical {
    pub fn send_key(&mut self, key: KeyCode) {
        match key {
            // navigation_key left and right
            KeyCode::KEY_SETA_BRANCA_ESQUERDA => { self.edition_buffer.move_cursor_left(); }, 
            KeyCode::KEY_SETA_BRANCA_DIREITA => { self.edition_buffer.move_cursor_right(); },
            KeyCode::KEY_DIRECIONAL_PARA_DIREITA => { self.edition_buffer.move_cursor_right(); },
            KeyCode::KEY_DIRECIONAL_PARA_ESQUERDA => { self.edition_buffer.move_cursor_left(); },
            // edition keys
            KeyCode::KEY_0 => { self.edition_buffer.change_cursor_item_to('0').move_cursor_right(); },
            KeyCode::KEY_1 => { self.edition_buffer.change_cursor_item_to('1').move_cursor_right(); },
            KeyCode::KEY_2 => { self.edition_buffer.change_cursor_item_to('2').move_cursor_right(); },
            KeyCode::KEY_3 => { self.edition_buffer.change_cursor_item_to('3').move_cursor_right(); },
            KeyCode::KEY_4 => { self.edition_buffer.change_cursor_item_to('4').move_cursor_right(); },
            KeyCode::KEY_5 => { self.edition_buffer.change_cursor_item_to('5').move_cursor_right(); },
            KeyCode::KEY_6 => { self.edition_buffer.change_cursor_item_to('6').move_cursor_right(); },
            KeyCode::KEY_7 => { self.edition_buffer.change_cursor_item_to('7').move_cursor_right(); },
            KeyCode::KEY_8 => { self.edition_buffer.change_cursor_item_to('8').move_cursor_right(); },
            KeyCode::KEY_9 => { self.edition_buffer.change_cursor_item_to('9').move_cursor_right(); },
            //everything else -> do nothing
            _ => { },
        }
    }
}



//Make possible to edit a position of memory using Lcd display and keyboard
//esc abort edition, and enter confirm edition
pub struct Field {
    numerical: Numerical,
    blink: RectangularWave<u32>,
    edit_mode: EditMode,
    initial_cursor_position: usize,
    valid_range: Range<u16>,
    number_of_digits: usize,
    accessor: Accessor<u16>,
}

impl Field {
    pub fn new(accessor: Accessor<u16>, initial_cursor_position: usize, number_of_digits: usize, valid_range: Range<u16>) -> Self {
        let value = accessor.get();
        let array = convert_u16_to_FieldBuffer(value, number_of_digits);
        let edition_buffer = EditionBuffer::new(array.clone(), initial_cursor_position);
        Self {
            numerical: Numerical::new(edition_buffer),
            blink: RectangularWave::new(400,700),
            edit_mode: EditMode::new(false),
            initial_cursor_position,
            valid_range,
            number_of_digits,
            accessor,
        }
    }


    fn __saves_data(&mut self) {
        let value = self.numerical.to_u16();
        let min = self.valid_range.start;
        let max = self.valid_range.end;
        let value_clamped = value.clamp(min, max);
        self.accessor.set(value_clamped);
        let field_buffer = convert_u16_to_FieldBuffer(value_clamped, self.number_of_digits);
        self.numerical = Numerical::new(EditionBuffer::new(field_buffer.clone(), self.initial_cursor_position));
    }

    /// disconsider edited value and reset edition cursor
    fn __abort_edition(&mut self) {
        let previous_value = self.accessor.get(); // original value
        let field_buffer = convert_u16_to_FieldBuffer(previous_value, self.number_of_digits);
        self.numerical = Numerical::new(EditionBuffer::new(field_buffer, self.initial_cursor_position));
    }
}

impl Field {

    pub fn send_key(&mut self, key: KeyCode) {     
        
        if self.is_in_edit_mode() {

            match key {
                // cancel edition
                KeyCode::KEY_ESC => {
                    self.set_edit_mode(false); // terminate edition
                    self.__abort_edition(); 
                }
                
                // saves edition
                KeyCode::KEY_ENTER => {
                    self.set_edit_mode(false); // terminate edition
                    self.__saves_data();
                }

                 //delegate everything else
                _ => self.numerical.send_key(key),
                
            };
    
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
        for (position,digit) in self.numerical.char_indices() {
            let blink_char = '_';
            let mut current_char = digit.clone();
            let is_current_char_over_cursor = position == self.numerical.get_current_cursor_index();
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
