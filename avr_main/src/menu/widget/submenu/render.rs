use super::{
    super::menu_item::menu_item::MenuItemWidget, hepers::LcdLine, navigation_state::NavigationState,
};
use crate::{
    board::keyboard::KeyCode,
    menu::{
        canvas::Canvas,
        point::Point,
        widget::submenu::spec::{MenuStorage, SubMenuHandle},
    },
};
use heapless::Vec;
use lib_1::utils::common::usize_to_u8_clamper;

/////////////////////////////////

pub struct SubMenuRender<'a> {
    /// List of all submenu items.
    menu_storage: &'a MenuStorage<'a>,
    current_menu: SubMenuHandle,
    /// State of widgets which are currently mounted on screen.
    mounted: [MenuItemWidget<'a>; 2], // TOTAL_NUMBER_OF_LINES_IN_LCD as usize],
    /// Stores the path of menu jumps that user perform, so you can go back to previous menu
    navigation_path: Vec<SubMenuHandle, 7>,
}

impl<'a> SubMenuRender<'a> {
    pub fn new(submenu_handle: SubMenuHandle, menu_storage: &'a MenuStorage) -> Self {
        Self {
            menu_storage,
            mounted: [
                menu_storage.get_item(submenu_handle, 0).unwrap(),
                menu_storage.get_item(submenu_handle, 1).unwrap(),
            ],
            current_menu: submenu_handle,
            navigation_path: Vec::new(),
        }
    }

    /// Gets a copy of the Navigation State.
    /// NOTE: Any modification on the copy will not reflect in the official state.
    /// TODO: Refactor this concept when possible.
    fn get_navigation_state(&self) -> NavigationState {
        self.menu_storage
            .get_navigation_state(self.current_menu)
            .get()
    }

    /// Updates the navigation state of current sub_menu by applying update_fn on it
    fn update_navigation_state(
        &self,
        update_fn: fn(NavigationState, menu_length: u8) -> NavigationState,
    ) {
        let menu_length = usize_to_u8_clamper(self.menu_storage.len(self.current_menu));
        let current_nav_state = self.get_navigation_state();
        let updated_nav_state = update_fn(current_nav_state, menu_length);
        self.menu_storage
            .get_navigation_state(self.current_menu)
            .set(updated_nav_state)
    }

    /// Mount widgets that are being renderized
    fn mount(&mut self) {
        for lcd_line in LcdLine::iterator() {
            let index = self.get_navigation_state().get_current_index_for(lcd_line) as usize;
            let menu_item_widget = self
                .menu_storage
                .get_item(self.current_menu, index)
                .unwrap();
            if let Some(elem) = self.mounted.get_mut(lcd_line as u8 as usize) {
                // mount item
                *elem = menu_item_widget;
            } else {
                // Mounting error
                panic!("E2");
            }
        }
    }

    /// Get mounted item for a particular line (mutable reference)
    fn get_mounted_item_for_line(&mut self, lcd_line: LcdLine) -> &mut MenuItemWidget<'a> {
        if let Some(elem) = self.mounted.get_mut(lcd_line as u8 as usize) {
            return elem;
        } else {
            // Mounting error
            panic!("E1");
        }
    }

    /// Get mounted item in the current line which is selected by user
    fn get_current_selected_mounted_item(&mut self) -> &mut MenuItemWidget<'a> {
        let line = self.get_navigation_state().get_current_lcd_line();
        let current_menu_item = self.get_mounted_item_for_line(line);
        current_menu_item
    }

    fn key_down(&mut self) {
        self.update_navigation_state(|mut nav_state, menu_length| {
            nav_state.key_down(menu_length);
            nav_state
        });
        self.mount();
    }

    fn key_up(&mut self) {
        self.update_navigation_state(|mut nav_state, _menu_length| {
            nav_state.key_up();
            nav_state
        });
        self.mount();
    }

    /// Changes current submenu
    fn go_to_submenu(&mut self, submenu_handle: SubMenuHandle) {
        self.current_menu = submenu_handle;
        self.mount();
    }

    fn go_to_child(&mut self, child: SubMenuHandle) {
        // do nothing if child is pointing to itself
        if self.current_menu != child {
            // saves parent
            let parent = self.current_menu;
            match self.navigation_path.push(parent) {
                Ok(_) => (),
                // ERROR DESCRIPTION: `navigation_path` must be redimensioned to higher capacity.
                Err(_) => panic!("E14"),
            }
            // go to child
            self.go_to_submenu(child)
        }
    }

    fn back_to_parent(&mut self) {
        // pops parent from navigation path
        let parent = match self.navigation_path.pop() {
            Some(parent) => parent,
            None => self.current_menu,
        };
        // go to parent
        self.go_to_submenu(parent)
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
                        self.go_to_child(child_handle);
                    } else {
                        // Enters edit mode on sub-widgets.
                        current_menu_item.set_edit_mode(true);
                    }
                }

                KeyCode::KEY_ESC => {
                    self.back_to_parent();
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
        let line = self.get_navigation_state().get_current_lcd_line();
        self.draw_menu_item_selector(line, canvas);
        // draw menu items
        for line in LcdLine::iterator() {
            self.get_mounted_item_for_line(line).draw(canvas, line);
        }
    }
}
