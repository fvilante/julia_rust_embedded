use core::slice::Iter;
use heapless::Vec;
use crate::{board::keyboard::KeyCode, menu::{canvas::Canvas, point::Point}};
use super::{menu_item::MenuItem, cursor::Cursor};

//represents the lines of the 40x2 LCD display
#[derive(PartialEq, Copy, Clone)]
pub enum LcdLine {
    Line0,
    Line1,
}

use LcdLine::Line0;
use LcdLine::Line1;

impl LcdLine {
    pub fn iterator() -> impl Iterator<Item = LcdLine> {
        [Line0, Line1].iter().copied()
    }
    
    pub fn as_u8(&self) -> u8 {
        match self {
            Line0 => 0,
            Line1 => 1,
        }
    }
}

pub type MenuItemGetter = fn() -> MenuItem;

pub type MenuList = Vec<MenuItemGetter,10>;

pub struct SubMenu {
    menu_list: MenuList,    // all itens of submenu
    menu_items: Vec<MenuItem,2>, // first and second lcd lines
    current_lcd_line_selected: LcdLine,  // lcd line reference
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
            current_lcd_line_selected: Line0,
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

    fn get_menu_item_mut(&mut self, line: LcdLine) -> &mut MenuItem {
        let index = line.as_u8() as usize;
        self.menu_items.get_mut(index).unwrap()
    }

    fn get_menu_item(&self, line: LcdLine) -> &MenuItem {
        let index = line.as_u8() as usize;
        self.menu_items.get(index).unwrap()
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

    // if is in edit mode for some line returns Some<Line> else None
    fn get_line_being_edited(&self) -> Option<LcdLine> {
        for line in LcdLine::iterator() {
            let is_editing_some_line = self.get_menu_item(line).is_in_edit_mode();
            if is_editing_some_line {
                return Some(line);
            }
        }
        None
    }

    // false = line0, true = line1
    fn set_editing_mode_for_line(&mut self, line: LcdLine, value: bool) {
        self.get_menu_item_mut(line).set_edit_mode(value)
    }

    /// helper function to draw submenu cursor on screen
    fn draw_menu_item_selector(&self, line: LcdLine, canvas: &mut Canvas) {
        const EDITING_CURSOR: char = '*';
        const NAVIGATING_CURSOR: char = '>';
        // position cursor
        canvas.set_cursor(Point::new(0,line.as_u8()));
        // draw selector char
        match self.get_line_being_edited() {
            Some(line) => {
                canvas.print_char(EDITING_CURSOR);
            }
            None => {
                canvas.print_char(NAVIGATING_CURSOR);
            }
        }
    }

}


impl SubMenu {
    pub fn send_key(&mut self, key: KeyCode) {
        let is_editing_some_line = self.get_line_being_edited();

        match is_editing_some_line {
            //is editing some line
            Some(current_line) => {
                // delegate keys 
                self.get_menu_item_mut(current_line).send_key(key);
            }

            //not editing any line
            None => {
                // navigate menu
                match key {
                    KeyCode::KEY_DIRECIONAL_PARA_BAIXO => {
                        match self.current_lcd_line_selected {
                            LcdLine::Line0 => { self.current_lcd_line_selected = Line1;},
                            LcdLine::Line1 => { self.scroll_down(); },
                        }
                    },
                    KeyCode::KEY_DIRECIONAL_PARA_CIMA => {
                        match self.current_lcd_line_selected {
                            LcdLine::Line0 => { self.scroll_up(); },
                            LcdLine::Line1 => { self.current_lcd_line_selected = Line0;},
                            
                        }
                    },
                    KeyCode::KEY_ENTER => {
                        let line = self.current_lcd_line_selected;
                        self.get_menu_item_mut(line).set_edit_mode(true);
                    }
                    _ => { }
                }
            }
        }
        
    }

    pub fn update(&mut self) {
        for line in LcdLine::iterator() {
            self.get_menu_item_mut(line).update();
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        // clear screen
        canvas.clear();
        // draw menu item selector
        let current_line = self.current_lcd_line_selected;
        self.draw_menu_item_selector(current_line, canvas);
        // draw menu items
        for line in LcdLine::iterator() {
            self.get_menu_item(line).draw(canvas, line);
        }
    }
}
