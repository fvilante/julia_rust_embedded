use crate::{menu::{flash::FlashString, point::Point, canvas::Canvas}, board::keyboard::KeyCode};

use super::widget::Widget;


pub struct Caption {
    text: FlashString,
    start_point: Point,
}

impl Caption {
    pub fn new(start_point: Point, text: FlashString) -> Self {
        Self {
            text,
            start_point,
        }
    }

    pub fn set_caption(&mut self, text: FlashString) {
        self.text = text;
    }
}

impl Caption {
    pub fn send_key(&mut self, _key: KeyCode) { 
        // ignore key
    }

    pub fn update(&mut self) {
        // do nothing
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.set_cursor(self.start_point);
        for byte in self.text.chars() {
            canvas.print_char(byte as char);
        }
    }
}