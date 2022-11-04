
use core::str::FromStr;

use alloc::borrow::ToOwned;
use heapless::{ 
    Vec,
    String,
};
use lib_1::utils::common::convert_u16_to_string_decimal;

use crate::{menu::{point::Point, ratangular_wave::RectangularWave, canvas::Canvas, accessor::Accessor}, board::keyboard::KeyCode};

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

//Make possible to edit a position of memory using Lcd display and keyboard
//esc abort edition, and enter confirm edition
pub struct Field {
    edition_buffer: EditionBuffer,
    blink: RectangularWave<u32>,
    edit_mode: EditMode,
    final_buffer: FieldBuffer,
    last_saved_value_has_been_retrieved: bool,
    initial_cursor_position: usize,
}

impl Field {
    pub fn new(array: FieldBuffer, initial_cursor_position: usize) -> Self {
        Self {
            edition_buffer: EditionBuffer::new(array.clone(), initial_cursor_position),
            blink: RectangularWave::new(400,700),
            edit_mode: EditMode::new(false),
            final_buffer: array,
            last_saved_value_has_been_retrieved: true,
            initial_cursor_position,
        }
    }


    pub fn get_value_if_it_has_changed(&mut self) -> Option<FieldBuffer> {
        if self.last_saved_value_has_been_retrieved == false {
            self.last_saved_value_has_been_retrieved = true;
            Some(self.final_buffer.clone())
        } else {
            None
        }
        
    }
}

impl Field {

    pub fn send_key(&mut self, key: KeyCode) {     
        
        if self.is_in_edit_mode() {

            let effect = match key {
                // save/cancel edition
                KeyCode::KEY_ESC => {
                    self.set_edit_mode(false); // terminate edition
                    self.edition_buffer.cursor.set_current(self.initial_cursor_position); // reset cursor position
                    self.edition_buffer.buffer = self.final_buffer.clone(); // disconsider edited value
                    Some(())
                }
                KeyCode::KEY_ENTER => {
                    self.set_edit_mode(false); // terminate edition
                    self.edition_buffer.cursor.set_current(self.initial_cursor_position); // reset cursor position
                    self.final_buffer = self.edition_buffer.buffer.clone(); // saves value
                    self.last_saved_value_has_been_retrieved = false; // reset flag
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
