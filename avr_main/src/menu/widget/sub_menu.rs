use super::menu_item::{MenuItem, MenuItemArgs};
use crate::{
    board::keyboard::KeyCode,
    menu::{canvas::Canvas, point::Point},
    unwrap_option,
};
use core::{cell::Cell, ops::Range, slice::Iter};
use heapless::Vec;
use lib_1::arena::arena::Arena;
use lib_1::utils::cursor::Cursor;

/// Helper type to represent each lines of the 40x2 LCD display.
///
/// You may consider this type just to avoid to cast the two lcd lines direct to a `u8` type.
#[derive(PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum LcdLine {
    Line0 = 0,
    Line1 = 1,
}

impl LcdLine {
    pub fn iterator() -> impl Iterator<Item = LcdLine> {
        [LcdLine::Line0, LcdLine::Line1].iter().copied()
    }
}

impl From<u8> for LcdLine {
    fn from(value: u8) -> Self {
        match value {
            0 => LcdLine::Line0,
            1 => LcdLine::Line1,
            _ => LcdLine::Line0, // default
        }
    }
}

pub type MenuList = Vec<MenuItemArgs, 6>;

pub struct SubMenu<'a> {
    /// List of all submenu items.
    menu_list: MenuList,
    /// Controls the line of menu (see: LcdLine) which is current selected.
    lcd_line_cursor: Cursor,
    /// First line to render in the lcd screen in relation to the [`MenuList`].
    first_line_to_render: Cursor,
    /// State of widgets which are currently mounted on screen.
    mounted: [MenuItem<'a>; 1],
}

impl<'a> SubMenu<'a> {
    pub fn new(mut menu_list: MenuList) -> Self {
        let size = menu_list.len();
        let mounted_0 = MenuItem::from_menu_args(&menu_list[0]);
        Self {
            menu_list,
            lcd_line_cursor: {
                const number_of_lcd_lines: u8 = 2;
                const initial_line_selected: u8 = 0;
                Cursor::new(0, number_of_lcd_lines, initial_line_selected)
            },
            first_line_to_render: {
                let default_initial_menu_item = 0;
                Cursor::from_range(0..size - 1, default_initial_menu_item)
            },
            mounted: {
                //let mounted_1 = MenuItem::from_menu_args(&menu_list[1]);
                [mounted_0] //, mounted_1]
            },
        }
    }

    fn get_current_lcd_line(&self) -> LcdLine {
        LcdLine::from(self.lcd_line_cursor.get_current())
    }

    /// Returns the index that points on the element in the `MenuList` that should be rendered in the equivalente
    /// `LcdLine` position on Lcd display.
    fn get_current_index(&self, line: LcdLine) -> u8 {
        let lcd_index = line as u8;
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

    /// If is in edit mode for some line returns Some(LcdLine) else None.
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
        canvas.set_cursor(Point::new(0, line as u8));
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
        if let Some(line_being_edited) = self.get_line_being_edited() {
            // if is editing some line, delegate keys to sub widgets.
            self.get_menu_item_mut(line_being_edited).send_key(key);
        } else {
            // if not editing any line we are responsible to show up/down menu navigation.
            match key {
                KeyCode::KEY_DIRECIONAL_PARA_BAIXO => {
                    let already_on_bottom = self.lcd_line_cursor.next();
                    if already_on_bottom {
                        self.scroll_down()
                    }
                }

                KeyCode::KEY_DIRECIONAL_PARA_CIMA => {
                    let already_on_top = self.lcd_line_cursor.previous();
                    if already_on_top {
                        self.scroll_up()
                    }
                }

                KeyCode::KEY_ENTER => {
                    // Enters edit mode on sub-widgets.
                    let line = self.get_current_lcd_line();
                    self.get_menu_item_mut(line).set_edit_mode(true);
                }

                _ => {
                    // ignore other keys
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
        let current_line = self.get_current_lcd_line();
        self.draw_menu_item_selector(current_line, canvas);
        // draw menu items
        for line in LcdLine::iterator() {
            self.get_menu_item(line).draw(canvas, line);
        }
    }
}
