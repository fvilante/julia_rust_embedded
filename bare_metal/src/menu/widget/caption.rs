use crate::geometry::point::Point;
use crate::string::flash::FlashString;
use crate::{
    board::keypad::KeyCode,
    menu::{screen_buffer::ScreenBuffer, widget::widget::Widget},
};
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

impl Widget for Caption {
    fn send_key(&mut self, _key: KeyCode) {
        // ignore key
    }

    fn update(&mut self) {
        // do nothing
    }

    fn draw(&self, screen_buffer: &mut ScreenBuffer, start_point: Point) {
        screen_buffer.set_cursor(start_point);
        for byte in self.text.into_iter() {
            screen_buffer.print_char(byte as char);
        }
    }
}
