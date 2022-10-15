use crate::{
    board::keyboard::KeyCode,
    menu::{canvas::Canvas, flash::FlashString, point::Point},
};

use super::{caption::Caption, field::{Field, FieldBuffer}, widget::Editable, widget::Widget};

use heapless::String;

pub struct MenuItem {
    caption: Caption,
    field: Field,
}

impl MenuItem {
    /// NOTE: client should put point1 and point2 in the same line
    pub fn new(point1: Point, text: FlashString, point2: Point, array: FieldBuffer) -> Self {
        Self {
            caption: Caption::new(point1, text),
            field: Field::new(point2, array),
        }
    }

    pub fn set_caption(&mut self, text: FlashString) {
        self.caption.set_caption(text);
    }
}

impl Widget for MenuItem {
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

impl Editable for MenuItem {
    fn set_edit_mode(&mut self, value: bool) {
        self.field.set_edit_mode(value);
    }

    fn is_in_edit_mode(&self) -> bool {
        self.field.is_in_edit_mode()
    }
}
