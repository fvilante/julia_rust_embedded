use crate::{menu::canvas::Canvas, board::keyboard::KeyCode};

pub trait Widget {
    fn send_key(&mut self, key: KeyCode);
    fn update(&mut self);
    fn draw(&self, canvas: &mut Canvas);
}


pub trait Editable {
    fn set_edit_mode(&mut self, value: bool);
    fn is_in_edit_mode(&self) -> bool;
    fn toggle_edit_mode(&mut self) {
        if self.is_in_edit_mode() {
            self.set_edit_mode(false);
        } else {
            self.set_edit_mode(true);
        }
    }
}