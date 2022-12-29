use super::menu_item::{MenuItemArgs, MenuItemWidget};
use crate::{
    board::keyboard::KeyCode,
    menu::{canvas::Canvas, point::Point, sub_menu_handle::SubMenuHandle},
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

const TOTAL_NUMBER_OF_LINES_IN_LCD: u8 = 2;

pub struct SubMenuRender {
    /// List of all submenu items.
    current_menu: SubMenuHandle,
    /// Controls the line of menu (see: LcdLine) which is current selected.
    lcd_line_cursor: Cursor,
    /// First line to render in the lcd screen in relation to the [`MenuList`].
    first_line_to_render: Cursor,
    /// State of widgets which are currently mounted on screen.
    mounted: [MenuItemWidget; 2], // TOTAL_NUMBER_OF_LINES_IN_LCD as usize],
}

impl SubMenuRender {
    pub fn new(mut menu_list: SubMenuHandle) -> Self {
        let menu_list_length = menu_list.len();
        let mounted_0 = menu_list.get_item(0).unwrap();
        let mounted_1 = menu_list.get_item(1).unwrap();

        Self {
            mounted: [mounted_0, mounted_1],
            current_menu: menu_list,
            lcd_line_cursor: {
                const initial_line_selected: u8 = 0;
                Cursor::new(0, TOTAL_NUMBER_OF_LINES_IN_LCD, initial_line_selected)
            },
            first_line_to_render: {
                let default_initial_menu_item = 0;
                Cursor::from_range(
                    0..menu_list_length - (TOTAL_NUMBER_OF_LINES_IN_LCD - 1) as usize,
                    default_initial_menu_item,
                )
            },
        }
    }

    fn get_current_lcd_line(&self) -> LcdLine {
        LcdLine::from(self.lcd_line_cursor.get_current())
    }

    /// Finds the given LcdLine in the MenuList collection (as MenuList index)
    fn get_current_index_for(&self, line: LcdLine) -> u8 {
        let lcd_line = line as u8;
        let first_line_to_render = self.first_line_to_render.get_current();
        let item_index = lcd_line + first_line_to_render;
        item_index
    }

    /// Mount widgets that are being renderized
    fn mount(&mut self) {
        for lcd_line in LcdLine::iterator() {
            let index = self.get_current_index_for(lcd_line) as usize;
            let mut menu_item_widget = self.current_menu.get_item(index).unwrap();
            if let Some(elem) = self.mounted.get_mut(lcd_line as u8 as usize) {
                // mount item
                *elem = menu_item_widget;
            } else {
                panic!("Menu error 02");
            }
        }
    }

    fn get_mounted_item_in_lcd_mut(&mut self, lcd_line: LcdLine) -> &mut MenuItemWidget {
        if let Some(elem) = self.mounted.get_mut(lcd_line as u8 as usize) {
            return elem;
        } else {
            panic!("Menu error 01");
        }
    }

    fn scroll_down(&mut self) {
        let has_exhausted = self.first_line_to_render.next();
        if !has_exhausted {
            self.mount();
        }
    }

    fn scroll_up(&mut self) {
        let has_exhausted = self.first_line_to_render.previous();
        if !has_exhausted {
            self.mount();
        }
    }

    /// If is in edit mode for some line returns Some(LcdLine) else None.
    /// TODO: Remove mutability of self when possible
    fn get_line_being_edited(&mut self) -> Option<LcdLine> {
        for line in LcdLine::iterator() {
            let is_editing_some_line = self.get_mounted_item_in_lcd_mut(line).is_in_edit_mode();
            if is_editing_some_line {
                return Some(line);
            }
        }
        None
    }

    /// helper function to draw submenu cursor on screen
    ///
    /// TODO: remove mutability on self when possible
    fn draw_menu_item_selector(&mut self, line: LcdLine, canvas: &mut Canvas) {
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

impl SubMenuRender {
    pub fn send_key(&mut self, key: KeyCode) {
        if let Some(line_being_edited) = self.get_line_being_edited() {
            // if is editing some line, delegate keys to sub widgets.
            self.get_mounted_item_in_lcd_mut(line_being_edited)
                .send_key(key);
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
                    self.get_mounted_item_in_lcd_mut(line).set_edit_mode(true);
                }

                _ => {
                    // ignore other keys
                }
            }
        }
    }

    pub fn update(&mut self) {
        for line in LcdLine::iterator() {
            self.get_mounted_item_in_lcd_mut(line).update();
        }
    }

    /// TODO: Remove motability of self when possible.
    pub fn draw(&mut self, canvas: &mut Canvas) {
        // clear screen
        canvas.clear();
        // draw menu item selector
        let line = self.get_current_lcd_line();
        self.draw_menu_item_selector(line, canvas);
        // draw menu items
        for line in LcdLine::iterator() {
            self.get_mounted_item_in_lcd_mut(line).draw(canvas, line);
        }
    }
}
