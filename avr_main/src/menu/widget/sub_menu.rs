use core::{slice::Iter, cell::Cell};
use heapless::Vec;
use lib_1::arena::arena::Arena;
use crate::{board::keyboard::KeyCode, menu::{canvas::Canvas, point::Point}, unwrap_option};
use super::{menu_item::{MenuItem, MenuItemArgs}, cursor::Cursor};

//represents the lines of the 40x2 LCD display
#[derive(PartialEq, Copy, Clone)]
#[repr(u8)]
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
}

pub type MenuList = Vec<MenuItemArgs,6>;

pub struct SubMenu<'a> {
    menu_list: MenuList,    // all itens of submenu
    current_lcd_line_selected: LcdLine,  // lcd line reference
    first_line_to_render: Cursor, // line of the vector 'MenuList' which must be the first line to render in the first line of the lcd
    mounted: [MenuItem<'a>;1],
}


impl<'a> SubMenu<'a> {
    pub fn new(mut menu_list: MenuList) -> Self {
        let size = menu_list.len();
        let default_initial_menu_item = 0;
        let mounted_0 = MenuItem::from_menu_args(&menu_list[0]);
        //let mounted_1 = MenuItem::from_menu_args(&menu_list[1]);
        Self {
            menu_list,
            current_lcd_line_selected: Line0,
            first_line_to_render: Cursor::from_range(0..size-1, default_initial_menu_item),
            mounted: [mounted_0], //, mounted_1],
        }
    }

    fn get_current_index(&self, line: LcdLine) -> usize {
        let lcd_index = line as usize;
        let line_index = self.first_line_to_render.get_current();
        let index = lcd_index + line_index;
        index
    }

    fn get_menu_item_mut(&mut self, line: LcdLine) -> &mut MenuItem<'a> {
        //let index = self.get_current_index(line);
        //self.menu_list.get_mut(index).unwrap()
        &mut self.mounted[0]
    }

    fn get_menu_item(&self, line: LcdLine) -> &MenuItem<'a> {
        //let index = self.get_current_index(line);
        //self.menu_list.get(index).unwrap()]
        &self.mounted[0]
    }

    fn scroll_down(&mut self) {
        self.first_line_to_render.next();
    }

    fn scroll_up(&mut self) {
        self.first_line_to_render.previous();
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
        canvas.set_cursor(Point::new(0,line as u8));
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


impl<'a> SubMenu<'a> {
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
