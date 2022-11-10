use heapless::Vec;

use crate::{board::keyboard::KeyCode, menu::{canvas::Canvas, point::Point}};

use super::{menu_item::MenuItem, cursor::Cursor};

//represents the lines of the 40x2 LCD  display
pub const LINE_0: bool = false;
pub const LINE_1: bool = true;


pub enum MenuItemEnum {
    MenuItem(MenuItem)
}

impl MenuItemEnum {
    pub fn set_edit_mode(&mut self, value: bool) {
        match self {
            MenuItemEnum::MenuItem(m_item) => m_item.set_edit_mode(value),
        }
    }

    pub fn is_in_edit_mode(&self) -> bool {
        match self {
            MenuItemEnum::MenuItem(m_item) => m_item.is_in_edit_mode(),
        }
    }

    pub fn send_key(&mut self, key: KeyCode) {
        match self {
            MenuItemEnum::MenuItem(m_item) => m_item.send_key(key),
        }
    }

    pub fn update(&mut self) {
        match self {
            MenuItemEnum::MenuItem(m_item) => m_item.update(),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, lcd_line: bool) {
        match self {
            MenuItemEnum::MenuItem(m_item) => m_item.draw(canvas, lcd_line),
        }
    }
}





pub type MenuItemEnumGetter = fn() -> MenuItemEnum;

pub type MenuList = Vec<MenuItemEnumGetter,10>;

pub struct SubMenu {
    menu_list: MenuList,    // all itens of submenu
    menu_items: Vec<MenuItemEnum,2>, // first and second lcd lines
    current_line_selected: bool,  // false = line0, true = line1
    first_line_to_render: Cursor, // line of the vector 'MenuList' which must be the first line to render in the first line of the lcd
}


impl SubMenu {
    pub fn new(menu_list: MenuList) -> Self {
        let mut menu_item_0 = menu_list[0]();
        let mut menu_item_1 = menu_list[1]();
        let size = menu_list.len();
        let initial_item_index = 0;
        let mut menu_items = Vec::new();
        menu_items.push(menu_item_0);
        menu_items.push(menu_item_1);
        Self {
            menu_list,
            menu_items,
            current_line_selected: LINE_0,
            first_line_to_render: Cursor::new(0..size-1, initial_item_index),

        }
    }

    fn update_menu_items(&mut self) {
        let index = self.first_line_to_render.get_current();
        let mut menu_item_0 = self.menu_list[index+0]();
        let mut menu_item_1 = self.menu_list[index+1]();
        self.menu_items.clear();
        self.menu_items.push(menu_item_0);
        self.menu_items.push(menu_item_1);
    }

    fn get_menu_item_for_line(&mut self, line: bool) -> &mut MenuItemEnum {
        if line==LINE_0 {
            self.menu_items.get_mut(0).unwrap()
        } else {
            self.menu_items.get_mut(1).unwrap()
        }
    }

    fn scroll_down(&mut self) {
        self.first_line_to_render.next();
        self.update_menu_items();
    }

    fn scroll_up(&mut self) {
        self.first_line_to_render.previous();
        self.update_menu_items();
    }

    //

    // if is in edit mode returns Some<Line> else None
    fn is_editing_some_line(&mut self) -> Option<bool> {
        let is_in_edit_mode_0 = self.get_menu_item_for_line(LINE_0).is_in_edit_mode();
        let is_in_edit_mode_1 = self.get_menu_item_for_line(LINE_1).is_in_edit_mode();
        let is_not_in_edit_mode = !is_in_edit_mode_0 && !is_in_edit_mode_1;
        if is_not_in_edit_mode {
            None
        } else {
            if is_in_edit_mode_0 {
                Some(LINE_0)
            } else {
                Some(LINE_1)
            }
        }
    }

    // false = line0, true = line1
    fn set_editing_mode_for_line(&mut self, line: bool, value: bool) {
        if line == LINE_0 {
            self.get_menu_item_for_line(LINE_0).set_edit_mode(value)
        } else {
            self.get_menu_item_for_line(LINE_1).set_edit_mode(value)
        }
    }

}


impl SubMenu {
    pub fn send_key(&mut self, key: KeyCode) {
        let is_editing_some_line = self.is_editing_some_line();

        match is_editing_some_line {
            //is editing some line
            Some(current_line) => {
                // delegate keys 
                if current_line == LINE_0 {
                    self.get_menu_item_for_line(LINE_0).send_key(key);
                } else { // LINE_1
                    self.get_menu_item_for_line(LINE_1).send_key(key);
                }
            }

            //not editing any line
            None => {
                // navigate menu
                match key {
                    KeyCode::KEY_DIRECIONAL_PARA_BAIXO => {
                        if self.current_line_selected == LINE_0 {
                            self.current_line_selected = LINE_1
                        } else {
                            self.scroll_down();
                        }
                     },
                    KeyCode::KEY_DIRECIONAL_PARA_CIMA => {
                        if self.current_line_selected == LINE_1 {
                            self.current_line_selected = LINE_0
                        } else {
                            self.scroll_up();
                        }
                     },
                    KeyCode::KEY_ENTER => {
                        if self.current_line_selected == LINE_0 {
                            match &mut self.get_menu_item_for_line(LINE_0) {
                                MenuItemEnum::MenuItem(x) => x.set_edit_mode(true),
                            }
                        } else { // LINE_1
                            match &mut self.get_menu_item_for_line(LINE_1) {
                                MenuItemEnum::MenuItem(x) => x.set_edit_mode(true),
                            }
                        }
                    }
                    _ => { }
                }
            }
        }
        
    }

    pub fn update(&mut self) {
        self.get_menu_item_for_line(LINE_0).update();
        self.get_menu_item_for_line(LINE_1).update();
    }

    pub fn draw(&mut self, canvas: &mut Canvas) {
        canvas.clear();
        fn draw_selector(self_: &mut SubMenu, line: bool, canvas: &mut Canvas) {
            fn draw_char(self_: &mut SubMenu, canvas: &mut Canvas) {
                match self_.is_editing_some_line() {
                    Some(_) => canvas.print_char('*'),
                    None => canvas.print_char('>')
                }
            }
            if line == LINE_0 {
                canvas.set_cursor(Point::new(0,0));
                draw_char(self_, canvas);
            } else {
                canvas.set_cursor(Point::new(0,1));
                draw_char(self_, canvas);
            }
        }
        if self.current_line_selected == LINE_0 {
            draw_selector(self, LINE_0, canvas);
        } else {
            draw_selector(self, LINE_1, canvas);
        }
        self.get_menu_item_for_line(LINE_0).draw(canvas, LINE_0);
        self.get_menu_item_for_line(LINE_1).draw(canvas, LINE_1);
    }
}
