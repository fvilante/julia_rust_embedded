use crate::board::keyboard::KeyCode;
use super::{widget::{splash::Splash, widget::{Widget, WidgetHelper}}, canvas::Canvas};

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

impl<'a> Widget for MenuManager<'a> {
    fn send_key(&mut self, key: KeyCode) {
        WidgetHelper::send_key(&mut self.root_widget, key)
    }

    fn update(&mut self) {
        WidgetHelper::update(&mut self.root_widget)
    }

    fn draw(&self, canvas: &mut Canvas) {
        WidgetHelper::draw(&self.root_widget, canvas)
    }
}