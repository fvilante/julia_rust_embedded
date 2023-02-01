use super::menu_item::menu_item::MenuItemWidget;
use crate::{
    board::keyboard::KeyCode,
    menu::{
        canvas::Canvas,
        point::Point,
        sub_menu_handle::{MenuStorage, SubMenuHandle},
    },
};

use lib_1::utils::common::usize_to_u8_clamper;
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

/// Controls the state of the navigation on sub menu, which is what is the selected line in the list of items.
///
/// TODO: The memory footprint size this struct may be optimized going from 6 bytes to at least 3 bytes if I made a custom Cursor
/// because `Cursor::Start` is always zero, and `Cursor:End` of `lcd_line_cursor` is always 2 or const statically defined.  
/// TODO: Change name to `SubmenuNavigationCursor`
#[derive(Copy, Clone)]
pub struct NavigationState {
    /// Controls the line of menu (see: LcdLine) which is current selected.
    lcd_line_cursor: Cursor,
    /// First line to render in the lcd screen in relation to the list of menu items
    first_line_to_render: Cursor,
}

impl NavigationState {
    pub fn new_from_submenu_len(number_of_menu_items: u8) -> Self {
        /// This application uses a LCD 40 collumns by 2 Lines in future this may be generalized
        const TOTAL_NUMBER_OF_LINES_IN_LCD: u8 = 2;
        Self {
            lcd_line_cursor: {
                const DEFAULT_INITIAL_LINE_SELECTED: u8 = 0;
                Cursor::new(
                    0,
                    TOTAL_NUMBER_OF_LINES_IN_LCD,
                    DEFAULT_INITIAL_LINE_SELECTED,
                )
            },
            first_line_to_render: {
                let default_initial_menu_item = 0;
                Cursor::from_range(
                    0..number_of_menu_items as usize - (TOTAL_NUMBER_OF_LINES_IN_LCD - 1) as usize,
                    default_initial_menu_item,
                )
            },
        }
    }

    /// Scrolls menu down, return true if it the scrooll has already been exausted
    pub fn scroll_down(&mut self) -> bool {
        let has_exhausted = self.first_line_to_render.next();
        has_exhausted
    }

    /// Scrolls menu up, return true if it the scrooll has already been exausted
    pub fn scroll_up(&mut self) -> bool {
        let has_exhausted = self.first_line_to_render.previous();
        has_exhausted
    }

    pub fn key_down(&mut self) {
        let is_exhasted = self.lcd_line_cursor.next();
        if is_exhasted {
            self.scroll_down();
        };
    }

    pub fn key_up(&mut self) {
        let is_exhasted = self.lcd_line_cursor.previous();
        if is_exhasted {
            self.scroll_up();
        };
    }

    pub fn get_current_lcd_line(&self) -> LcdLine {
        LcdLine::from(self.lcd_line_cursor.get_current())
    }

    /// Finds for a given LcdLine the index of the item currently selected (in respect to menu list)
    pub fn get_current_index_for(&self, line: LcdLine) -> u8 {
        let lcd_line = line as u8;
        let first_line_to_render = self.first_line_to_render.get_current();
        let item_index = lcd_line + first_line_to_render;
        item_index
    }
}

pub struct SubMenuRender<'a> {
    /// List of all submenu items.
    menu_storage: &'a MenuStorage<'a>,
    current_menu: SubMenuHandle,
    navigation_state: NavigationState,
    /// State of widgets which are currently mounted on screen.
    mounted: [MenuItemWidget<'a>; 2], // TOTAL_NUMBER_OF_LINES_IN_LCD as usize],
}

impl<'a> SubMenuRender<'a> {
    pub fn new(submenu_handle: SubMenuHandle, menu_storage: &'a MenuStorage) -> Self {
        let menu_handle_length = usize_to_u8_clamper(menu_storage.len(submenu_handle));

        Self {
            menu_storage,
            mounted: [
                menu_storage.get_item(submenu_handle, 0).unwrap(),
                menu_storage.get_item(submenu_handle, 1).unwrap(),
            ],
            navigation_state: NavigationState::new_from_submenu_len(menu_handle_length),
            current_menu: submenu_handle,
        }
    }

    /// Mount widgets that are being renderized
    fn mount(&mut self) {
        for lcd_line in LcdLine::iterator() {
            let index = self.navigation_state.get_current_index_for(lcd_line) as usize;
            let menu_item_widget = self
                .menu_storage
                .get_item(self.current_menu, index)
                .unwrap();
            if let Some(elem) = self.mounted.get_mut(lcd_line as u8 as usize) {
                // mount item
                *elem = menu_item_widget;
            } else {
                panic!("Menu error 02");
            }
        }
    }

    /// Get mounted item for a particular line (mutable reference)
    fn get_mounted_item_for_line(&mut self, lcd_line: LcdLine) -> &mut MenuItemWidget<'a> {
        if let Some(elem) = self.mounted.get_mut(lcd_line as u8 as usize) {
            return elem;
        } else {
            panic!("Menu error 01");
        }
    }

    /// Get mounted item in the current line which is selected by user
    fn get_current_selected_mounted_item(&mut self) -> &mut MenuItemWidget<'a> {
        let line = self.navigation_state.get_current_lcd_line();
        let current_menu_item = self.get_mounted_item_for_line(line);
        current_menu_item
    }

    fn key_down(&mut self) {
        self.navigation_state.key_down();
        self.mount();
    }

    fn key_up(&mut self) {
        self.navigation_state.key_up();
        self.mount();
    }

    /// If is in edit mode for some line returns Some(LcdLine) else None.
    /// TODO: Remove mutability of self when possible
    fn get_line_being_edited(&mut self) -> Option<LcdLine> {
        for line in LcdLine::iterator() {
            let is_editing_some_line = self.get_mounted_item_for_line(line).is_in_edit_mode();
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
            Some(_line) => {
                canvas.print_char(EDITING_CURSOR);
            }
            None => {
                canvas.print_char(NAVIGATING_CURSOR);
            }
        }
    }
}

impl SubMenuRender<'_> {
    pub fn clone_from(&mut self, origin: Self) {
        self.menu_storage = origin.menu_storage;
        self.current_menu = origin.current_menu;
        self.navigation_state = origin.navigation_state;
        self.mounted = origin.mounted;
    }
}

impl SubMenuRender<'_> {
    pub fn send_key(&mut self, key: KeyCode) {
        if let Some(line_being_edited) = self.get_line_being_edited() {
            // if is editing some line, delegate keys to sub widgets.
            self.get_mounted_item_for_line(line_being_edited)
                .send_key(key);
        } else {
            // if not editing any line we are responsible to show up/down menu navigation.
            match key {
                KeyCode::KEY_DIRECIONAL_PARA_BAIXO => {
                    self.key_down();
                }

                KeyCode::KEY_DIRECIONAL_PARA_CIMA => {
                    self.key_up();
                }

                KeyCode::KEY_ENTER => {
                    let current_menu_item = self.get_current_selected_mounted_item();
                    if let Some(child_handle) = current_menu_item.child {
                        // TEMP CODE: if current mitem has a child submenu, opens it.
                        self.clone_from(Self::new(child_handle, self.menu_storage));
                    } else {
                        // Enters edit mode on sub-widgets.
                        current_menu_item.set_edit_mode(true);
                    }
                }

                _ => {
                    // ignore other keys
                }
            }
        }
    }

    pub fn update(&mut self) {
        for line in LcdLine::iterator() {
            self.get_mounted_item_for_line(line).update();
        }
    }

    /// TODO: Remove motability of self when possible.
    pub fn draw(&mut self, canvas: &mut Canvas) {
        // clear screen
        canvas.clear();
        // draw menu item selector
        let line = self.navigation_state.get_current_lcd_line();
        self.draw_menu_item_selector(line, canvas);
        // draw menu items
        for line in LcdLine::iterator() {
            self.get_mounted_item_for_line(line).draw(canvas, line);
        }
    }
}
