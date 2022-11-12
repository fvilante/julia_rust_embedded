use crate::board::keyboard::KeyCode;
use super::{widget::{splash::Splash, widget::Widget}, canvas::Canvas};



pub struct MenuManager<'a> {
    root_widget: &'a mut dyn Widget, 
}

impl<'a> MenuManager<'a> {
    pub fn new(root_widget: &'a mut dyn Widget) -> Self {
        Self {
            root_widget,
        }
    }
}

impl Widget for MenuManager<'_> {
    fn send_key(&mut self, key: KeyCode) {
        self.root_widget.send_key(key)
    }

    fn update(&mut self) {
        self.root_widget.update()
    }

    fn draw(&self, canvas: &mut Canvas) {
        self.root_widget.draw(canvas)
    }
}