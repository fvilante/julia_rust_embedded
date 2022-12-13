use super::{
    canvas::Canvas,
    widget::{
        splash::Splash,
        widget::{IWidget, Widget, WidgetHelper},
    },
};
use crate::board::keyboard::KeyCode;

pub struct MenuManager<'a> {
    root_widget: Option<IWidget<'a>>,
}

impl<'a> MenuManager<'a> {
    pub fn new(root_widget: Option<IWidget<'a>>) -> Self {
        Self { root_widget }
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
