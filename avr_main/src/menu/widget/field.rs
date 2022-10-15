
use alloc::borrow::ToOwned;
use heapless::{ 
    Vec,
    String,
};

use crate::{menu::{point::Point, ratangular_wave::RectangularWave, canvas::Canvas}, board::keyboard::KeyCode};

use super::{edit_mode::EditMode, Widget, widget::Editable, cursor::Cursor};



struct BufferedCursor {
    buffer: String<10>,
    cursor: Cursor,
}

impl BufferedCursor {
    pub fn new(buffer: String<10>) -> Self {
        Self {
            cursor: Cursor::new(0..buffer.len()),
            buffer,
        }
    }


    pub fn change_cursor_item_to(&mut self, new_char: char) -> &mut Self {
        let current_cursor = self.cursor.get_current();
        let mut s: String<10> = String::new();
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

    //pub fn move_cursor_begin(&mut self) -> &mut Self {
    //    self.cursor.begin();
    //    self
    //}
    //
    //pub fn move_cursor_end(&mut self) -> &mut Self {
    //   self.cursor.end();
    //    self
    //}

    pub fn addAndMoveRight(&mut self, item: char) -> &mut Self {
        self
            .change_cursor_item_to(item)
            .move_cursor_right()
    }

}

pub struct Field {
    buffer: BufferedCursor,
    blink: RectangularWave,
    start_point: Point,
    edit_mode: EditMode,
}

impl Field {
    pub fn new(start_point: Point, array: String<10>) -> Self {
        Self {
            buffer: BufferedCursor::new(array),
            blink: RectangularWave::new(400,700),
            start_point,
            edit_mode: EditMode::new(false),
        }
    }
}

impl Widget for Field {

    fn send_key(&mut self, key: KeyCode) {     
        
        if self.is_in_edit_mode() {

            let effect = match key {
                // navigation_key left and right
                KeyCode::KEY_SETA_BRANCA_ESQUERDA => { self.buffer.move_cursor_left(); Some(()) }, 
                KeyCode::KEY_SETA_BRANCA_DIREITA => { self.buffer.move_cursor_right(); Some(()) },
                KeyCode::KEY_DIRECIONAL_PARA_DIREITA => { self.buffer.move_cursor_right(); Some(()) },
                KeyCode::KEY_DIRECIONAL_PARA_ESQUERDA => { self.buffer.move_cursor_left(); Some(()) },
                KeyCode::KEY_0 => { self.buffer.change_cursor_item_to('0').move_cursor_right(); Some(()) },
                KeyCode::KEY_1 => { self.buffer.change_cursor_item_to('1').move_cursor_right(); Some(()) },
                KeyCode::KEY_2 => { self.buffer.change_cursor_item_to('2').move_cursor_right(); Some(()) },
                KeyCode::KEY_3 => { self.buffer.change_cursor_item_to('3').move_cursor_right(); Some(()) },
                KeyCode::KEY_4 => { self.buffer.change_cursor_item_to('4').move_cursor_right(); Some(()) },
                KeyCode::KEY_5 => { self.buffer.change_cursor_item_to('5').move_cursor_right(); Some(()) },
                KeyCode::KEY_6 => { self.buffer.change_cursor_item_to('6').move_cursor_right(); Some(()) },
                KeyCode::KEY_7 => { self.buffer.change_cursor_item_to('7').move_cursor_right(); Some(()) },
                KeyCode::KEY_8 => { self.buffer.change_cursor_item_to('8').move_cursor_right(); Some(()) },
                KeyCode::KEY_9 => { self.buffer.change_cursor_item_to('9').move_cursor_right(); Some(()) },
                KeyCode::KEY_ESC => { self.set_edit_mode(false); Some(()) },
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

    fn update(&mut self) {
        self.blink.update();
    }

    fn draw(&self, canvas: &mut Canvas) {
        canvas.set_cursor(self.start_point);
        for (position,digit) in self.buffer.buffer.char_indices() {
            let blink_char = '_';
            let mut current_char = digit.clone();
            let is_current_char_over_cursor = position == self.buffer.cursor.get_current();
            let is_time_to_blink = self.blink.read() && self.is_in_edit_mode(); // do not blink if it is not in edit mode
            if is_current_char_over_cursor && is_time_to_blink {
                current_char = blink_char;
            } 
            canvas.print_char(current_char);
        };
    }
}

impl Editable for Field {
    fn set_edit_mode(&mut self, value: bool) {
        self.edit_mode.set_edit_mode(value);
    }

    fn is_in_edit_mode(&self) -> bool {
        self.edit_mode.is_in_edit_mode()
    }
}