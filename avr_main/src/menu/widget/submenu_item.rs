use crate::{menu::{point::Point, flash::FlashString, canvas::Canvas}, board::keyboard::KeyCode};

use super::{Caption, field::Field, Widget, widget::Editable};

use heapless::{
    String,
};


pub struct SubMenuItem {
    caption: Caption,
    field: Field,
}

impl SubMenuItem {
    /// NOTE: client should put point1 and point2 in the same line
    pub fn new(point1: Point, text: FlashString, point2: Point, array: String<10>) -> Self {
        Self {
            caption: Caption::new(point1, text),
            field: Field::new(point2, array),
        }
    }

    pub fn set_caption(&mut self, text: FlashString) {
        self.caption.set_caption(text);
    }
}

impl Widget for SubMenuItem {
    fn send_key(&mut self, key: KeyCode) {
        self.field.send_key(key);
    }

    fn update(&mut self) {
        self.caption.update();
        self.field.update();
    }

    fn draw(&self, canvas: &mut Canvas) {
        self.caption.draw(canvas);
        self.field.draw(canvas);
    }
}

impl Editable for SubMenuItem {
    fn set_edit_mode(&mut self, value: bool) {
        self.field.set_edit_mode(value);
    }

    fn is_in_edit_mode(&self) -> bool {
        self.field.is_in_edit_mode()
    }
}