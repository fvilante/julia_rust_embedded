use crate::{board::keyboard::KeyCode, menu::canvas::Canvas};

pub trait Widget {
    fn send_key(&mut self, key: KeyCode);
    fn update(&mut self);
    fn draw(&self, canvas: &mut Canvas);
}

pub type IWidget<'a> = &'a mut dyn Widget;

/// helper to abstracts pointer manipulation
pub struct WidgetHelper;

impl WidgetHelper {
    pub fn send_key(self_widget: &mut Option<IWidget>, key: KeyCode) {
        if let Some(widget) = &mut *self_widget {
            widget.send_key(key)
        }
    }

    pub fn update(self_widget: &mut Option<IWidget>) {
        if let Some(widget) = &mut *self_widget {
            widget.update()
        }
    }

    pub fn draw(self_widget: &Option<IWidget>, canvas: &mut Canvas) {
        if let Some(widget) = &*self_widget {
            widget.draw(canvas)
        }
    }
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
