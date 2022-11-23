
use core::str::{FromStr, CharIndices};
use core::ops::Range;

use alloc::borrow::ToOwned;
use heapless::{ 
    Vec,
    String,
};
use lib_1::utils::common::convert_u16_to_string_decimal;

use crate::board::lcd;
use crate::{menu::{point::Point, ratangular_wave::RectangularWave, canvas::Canvas, accessor::{Accessor }}, board::keyboard::KeyCode};

use super::optional::{Optional, OptionsBuffer};
use super::{edit_mode::EditMode, widget::Widget, widget::Editable, cursor::Cursor};


const MAX_NUMBER_OF_CHARS_IN_BUFFER: usize = 6;

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
    // initial condition
    pub initial_cursor_position: usize, 
}

impl EditionBuffer {
    pub fn new(buffer: FieldBuffer, initial_cursor_position: usize) -> Self {
        Self {
            cursor: Cursor::new(0..buffer.len(), initial_cursor_position),
            buffer,
            initial_cursor_position,
        }
    }

    pub fn clone(&self) -> Self {
        let buffer: FieldBuffer = String::from_str(self.buffer.as_str()).unwrap();
        let cursor = self.cursor.clone();
        Self {
            cursor,
            buffer,
            initial_cursor_position: self.initial_cursor_position,
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

    /// Reset cursor to its default initial position
    /// NOTE: This method is not necessary the same as begin() method
    pub fn reset_cursor(&mut self) {
        self.cursor = Cursor::new(0..self.buffer.len(), self.initial_cursor_position);
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
 
struct Numerical<'a> {                  
    edition_buffer: EditionBuffer, 
    valid_range: Range<u16>,       
    number_of_digits: usize,        
    // initial values
    initial_edition_buffer: EditionBuffer, 
    accessor: Accessor<'a, u16>         
} 

impl<'a> Numerical<'a> {
    pub fn new(edition_buffer: EditionBuffer, valid_range: Range<u16>, number_of_digits: usize, accessor: Accessor<'a,u16>) -> Self {
        Self {
            edition_buffer: edition_buffer.clone(),
            valid_range,
            number_of_digits,
            initial_edition_buffer: edition_buffer,
            accessor,
        }
    }

    pub fn set_u16(&mut self, value: u16) {
        let initial_cursor_position = self.edition_buffer.initial_cursor_position;
        let field_buffer = convert_u16_to_FieldBuffer(value, self.number_of_digits);
        self.edition_buffer = EditionBuffer::new(field_buffer, initial_cursor_position);
    }

    pub fn to_u16(&self) -> u16 {
        let value = convert_FieldBuffer_to_u16(self.edition_buffer.buffer.clone());
        value
    }

    pub fn set_edition_buffer(&mut self, edition_buffer: EditionBuffer) {
        self.edition_buffer = edition_buffer;

    }

    pub fn to_u16_normalized(&self) -> u16 {
        let value = self.to_u16();
        let min = self.valid_range.start;
        let max = self.valid_range.end;
        let value_clamped = value.clamp(min, max);
        value_clamped
    }

    pub fn get_current_cursor_index(&self) -> usize {
        self.edition_buffer.cursor.get_current()
    }

    pub fn reset_cursor(&mut self) {
        self.edition_buffer.reset_cursor()
    }

    pub fn char_indices(&self) -> CharIndices {
        self.edition_buffer.buffer.char_indices()
    }
}

impl Numerical<'_> {
    pub fn save_edition(&mut self) {
        let normalized_value = self.to_u16_normalized();
        self.accessor.set(normalized_value); // saves data to accessor
        self.set_u16(normalized_value); // saves displayed data
        self.reset_cursor();
    }

    pub fn abort_edition(&mut self) {
        let original_value = self.accessor.get(); 
        self.set_u16(original_value); // resets displayed data
        self.reset_cursor();
    }


}

impl Numerical<'_> {
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

pub struct NumericalField<'a> { 
    numerical: Numerical<'a>,   
    blink: RectangularWave, 
}

impl<'a> NumericalField<'a> {
    pub fn new(accessor: Accessor<'a,u16>, initial_cursor_position: usize, number_of_digits: usize, valid_range: Range<u16>) -> Self {
        let value = accessor.get();
        let array = convert_u16_to_FieldBuffer(value, number_of_digits);
        let edition_buffer = EditionBuffer::new(array.clone(), initial_cursor_position);
        Self {
            numerical: Numerical::new(edition_buffer, valid_range, number_of_digits, accessor),
            blink: RectangularWave::new(600,300),
        }
    }
}

impl NumericalField<'_> {
    pub fn save_edition(&mut self) {
        self.numerical.save_edition();
    }

    pub fn abort_edition(&mut self) {
        self.numerical.abort_edition();
    }

}

impl NumericalField<'_> {
    pub fn send_key(&mut self, key: KeyCode) {
        self.numerical.send_key(key)
    }

    pub fn update(&mut self) {
        self.blink.update(); // blinks cursor
    }

    pub fn draw(&self, canvas: &mut Canvas, start_point: Point, is_in_edit_mode: bool) {
        canvas.set_cursor(start_point);
        for (position,digit) in self.numerical.char_indices() {
            let blink_char = '_';
            let mut current_char = digit.clone();
            let is_current_char_over_cursor = position == self.numerical.get_current_cursor_index();
            let is_time_to_blink = self.blink.read() && is_in_edit_mode; // do not blink if it is not in edit mode
            if is_current_char_over_cursor && is_time_to_blink {
                current_char = blink_char;
            } 
            canvas.print_char(current_char);
        };
    }
}


pub enum FieldEnum<'a> {
    Numerical(NumericalField<'a>), 
    Optional(Optional<'a>),
}

impl FieldEnum<'_> {
    pub fn save_edition(&mut self) {
        match self {
            Self::Numerical(x) => x.save_edition(), 
            Self::Optional(x) => x.save_edition(),
        }
    }

    pub fn abort_edition(&mut self) {
        match self {
            Self::Numerical(x) => x.abort_edition(), 
            Self::Optional(x) => x.abort_edition(),
        }
    }
}

impl FieldEnum<'_> {
    pub fn send_key(&mut self, key: KeyCode) {
        match self {
            Self::Numerical(x) => x.send_key(key), 
            Self::Optional(x) => x.send_key(key),
        }
    }

    pub fn update(&mut self) {
        match self {
            Self::Numerical(x) => x.update(), 
            Self::Optional(x) => x.update(),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, start_point: Point, is_in_editing_mode: bool) {
        match self {
            Self::Numerical(x) => x.draw(canvas, start_point, is_in_editing_mode), 
            Self::Optional(x) => x.draw(canvas, start_point, is_in_editing_mode), 
        }
    }
}

//Makes possible to edit a position of memory using Lcd display and keyboard
//esc abort edition, and enter confirm edition
pub struct Field<'a> {
    field_enum: FieldEnum<'a>,
    edit_mode: EditMode,
}

impl<'a> Field<'a> {
    pub fn new(field_enum: FieldEnum<'a>) -> Self {
        Self {
            field_enum,
            edit_mode: EditMode::new(false),
        }
    }

    pub fn from_numerical(accessor: Accessor<'a,u16>, initial_cursor_position: usize, number_of_digits: usize, valid_range: Range<u16>) -> Self {
        let numerical_field = NumericalField::new(accessor, initial_cursor_position, number_of_digits, valid_range);
        let field_enum = FieldEnum::Numerical(numerical_field);
        Self::new(field_enum)
    }

    pub fn from_optional(options: OptionsBuffer, accessor: Accessor<'a,Cursor>) -> Self {
        let optional = Optional::new(options, accessor);
        let field_enum = FieldEnum::Optional(optional);
        Self::new(field_enum)
    }
}

impl Field<'_> {

    pub fn send_key(&mut self, key: KeyCode) {     
        
        if self.is_in_edit_mode() {

            match key {
                // cancel edition
                KeyCode::KEY_ESC => {
                    self.set_edit_mode(false); // terminate edition
                    self.field_enum.abort_edition(); 
                }
                
                // saves edition
                KeyCode::KEY_ENTER => {
                    self.set_edit_mode(false); // terminate edition
                    self.field_enum.save_edition(); 
                }

                 //delegate everything else
                _ => self.field_enum.send_key(key),
                
            };
    
        } 
    }

    pub fn update(&mut self) {
        self.field_enum.update()
    }

    pub fn draw(&self, canvas: &mut Canvas, start_point: Point) {
        let is_in_edit_mode = self.is_in_edit_mode();
        self.field_enum.draw(canvas, start_point, is_in_edit_mode)
    }
}

impl<'a> Field<'a> {
    pub fn set_edit_mode(&mut self, value: bool) {
        self.edit_mode.set_edit_mode(value);
    }

    pub fn is_in_edit_mode(&self) -> bool {
        self.edit_mode.is_in_edit_mode()
    }
}
