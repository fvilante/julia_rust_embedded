use crate::board::keyboard::KeyCode;
use super::{widget::{splash::Splash, widget::Widget}, canvas::Canvas};

pub enum MenuViewEnum {
    Splash(Splash)
}

impl MenuViewEnum {
    pub fn send_key(&mut self, key: KeyCode) {
        match self {
            Self::Splash(x) => x.send_key(key),
        }
    }

    pub fn update(&mut self) {
        match self {
            Self::Splash(x) => x.update(),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        match self {
            Self::Splash(x) => x.draw(canvas),
        }
    }
}



pub struct MenuManager {
    menu_view: MenuViewEnum, 
}

impl MenuManager {
    pub fn new(menu_view: MenuViewEnum) -> Self {
        Self {
            menu_view,
        }
    }
}

impl MenuManager {
    pub fn send_key(&mut self, key: KeyCode) {
        self.menu_view.send_key(key)
    }

    pub fn update(&mut self) {
        self.menu_view.update()
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        self.menu_view.draw(canvas)
    }
}