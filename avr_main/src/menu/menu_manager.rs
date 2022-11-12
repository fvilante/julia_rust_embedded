use crate::board::keyboard::KeyCode;
use super::{widget::{splash::Splash, widget::Widget}, canvas::Canvas};


pub struct MenuManager<'a> {
    root_widget: Option<&'a mut dyn Widget>, 
}

impl<'a> MenuManager<'a> {
    pub fn new(root_widget: Option<&'a mut dyn Widget>) -> Self {
        Self {
            root_widget,
        }
    }
}

impl Widget for MenuManager<'_> {
    fn send_key(&mut self, key: KeyCode) {
        if let Some(widget) = &mut self.root_widget {
            (*widget).send_key(key)
        }
    }

    fn update(&mut self) {
        if let Some(widget) = &mut self.root_widget {
            (*widget).update()

        }    }

    fn draw(&self, canvas: &mut Canvas) {
        if let Some(widget) = &self.root_widget {
            (*widget).draw(canvas)
        }
    }
}