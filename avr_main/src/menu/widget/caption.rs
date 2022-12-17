use crate::{
    board::keyboard::KeyCode,
    menu::{canvas::Canvas, flash::FlashString, point::Point},
};

use super::widget::Widget;

pub struct Caption {
    text: FlashString,
}

impl Caption {
    pub fn new(text: FlashString) -> Self {
        Self { text }
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

    pub fn draw(&self, canvas: &mut Canvas, start_point: Point) {
        canvas.set_cursor(start_point);
        for (byte, _index) in self.text.chars_indices() {
            canvas.print_char(byte as char);
        }
    }
}
