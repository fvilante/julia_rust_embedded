use super::{
    super::menu_item::menu_item::MenuItemWidget, hepers::LcdLine,
    navigation_state::NavigationStateModel,
};
use crate::fatal_error;
use crate::geometry::point::Point;
use crate::menu::widget::widget::Widget;
use crate::microcontroler::ratangular_wave::RectangularWave;
use crate::{
    board::{keypad::KeyCode, lcd},
    menu::{
        screen_buffer::ScreenBuffer,
        widget::submenu_programa::spec::{MenuProgramaAreanaSelector, MenuProgramaArena},
    },
    microcontroler::delay::delay_ms,
};
use cross_platform::utils::numerical::usize_to_u8_clamper;
use heapless::Vec;

/////////////////////////////////

/// Responsible to render the menu on the screen
///
/// TODO: Improve error handling
pub struct MenuProgramaControler<'a> {
    /// Storage for of all submenus.
    menu_arena: &'a MenuProgramaArena<'a>,
    /// Points to the current menu being displayed on screen
    current_menu: MenuProgramaAreanaSelector,
    /// State of widgets which are currently mounted and therefore visible on screen.
    /// TODO: Is not necessary to have two MenuItemWidget states on memory but just one. Introduce some logic when possible
    /// to optimize this.
    mounted: [MenuItemWidget<'a>; 2], // TOTAL_NUMBER_OF_LINES_IN_LCD as usize],
    /// Stores the path of menu jumps that user perform, so you can go back to previous menu
    navigation_path: Vec<MenuProgramaAreanaSelector, 7>,
    /// Main menu reads this bit, if it is set then it will render the main menu. When menu_menu pass control
    /// do menu_programa it resets this bit, and then menu_program is responsible to set it when it want to give
    /// back control to main_menu.
    /// TODO: Improve this communication methodology
    pub must_return_to_main_menu: bool,
    /// Blinks navigation cursor the select each item of the menu
    /// TODO: Move the blink code to the Caption widget
    blink: RectangularWave,
}

impl<'a> MenuProgramaControler<'a> {
    pub fn new(
        current_menu: MenuProgramaAreanaSelector,
        menu_arena: &'a MenuProgramaArena,
    ) -> Self {
        // Configuring character blinking
        const T_ON: u16 = 500;
        const T_OFF: u16 = 500;
        let blink = RectangularWave::new(T_ON, T_OFF);

        // Mount menu itens that will be visible on the screen
        let (Some(fist_menu_item), Some(second_menu_item)) = (
            menu_arena.get_item(current_menu, 0), 
            menu_arena.get_item(current_menu, 2),
        ) else {
            // NOTE: currently we do not accept submenus with less then 2 menu_items.
            // TODO: Accept submenus with less then 2 menu_items.
            fatal_error!(100);
        };

        Self {
            menu_arena,
            mounted: [fist_menu_item, second_menu_item],
            current_menu,
            navigation_path: Vec::new(),
            must_return_to_main_menu: false,
            blink,
        }
    }

    /// Gets a copy of the Navigation State.
    /// NOTE: Any modification on the copy will not reflect in the official state.
    /// TODO: Refactor this concept when possible.
    fn get_navigation_state(&self) -> NavigationStateModel {
        self.menu_arena
            .get_navigation_state(self.current_menu)
            .get()
    }

    /// Updates the navigation state of current sub_menu by applying update_fn on it
    fn update_navigation_state(
        &self,
        update_fn: fn(NavigationStateModel, menu_length: u8) -> NavigationStateModel,
    ) {
        let menu_length = usize_to_u8_clamper(self.menu_arena.len(self.current_menu));
        let current_nav_state = self.get_navigation_state();
        let updated_nav_state = update_fn(current_nav_state, menu_length);
        self.menu_arena
            .get_navigation_state(self.current_menu)
            .set(updated_nav_state)
    }

    /// Mount widgets that are being renderized
    fn mount(&mut self) {
        for lcd_line in LcdLine::iterator() {
            let index = self.get_navigation_state().get_current_index_for(lcd_line) as usize;
            let Some(menu_item_widget) = self
                .menu_arena
                .get_item(self.current_menu, index)
                else {
                    // TODO: Improve error handling
                    // Mounting error
                    lcd::clear();
                    lcd::print("Err91"); // menu mounting error
                    delay_ms(4000);
                    fatal_error!(103);
                };
            if let Some(elem) = self.mounted.get_mut(lcd_line as u8 as usize) {
                // mount item
                *elem = menu_item_widget;
            } else {
                // TODO: Improve error handling
                // Mounting error
                lcd::clear();
                lcd::print("Err92"); // menu mounting error
                delay_ms(4000);
                fatal_error!(104);;
            }
        }
    }

    /// Get mounted item for a particular lcd line (mutable reference)
    fn get_mounted_item_for_lcd_line_mut(&mut self, lcd_line: LcdLine) -> &mut MenuItemWidget<'a> {
        if let Some(elem) = self.mounted.get_mut(lcd_line as u8 as usize) {
            return elem;
        } else {
            // Mounting error
            fatal_error!(105);
        }
    }

    fn get_mounted_item_for_lcd_line(&self, lcd_line: LcdLine) -> &MenuItemWidget<'a> {
        if let Some(elem) = self.mounted.get(lcd_line as u8 as usize) {
            return elem;
        } else {
            // Mounting error
            fatal_error!(106);
        }
    }

    /// Get mounted item for the lcd line selected by the user
    fn get_monted_item_for_current_lcd_line(&mut self) -> &mut MenuItemWidget<'a> {
        let line = self.get_navigation_state().get_current_lcd_line();
        let current_menu_item = self.get_mounted_item_for_lcd_line_mut(line);
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
    fn go_to_menu(&mut self, menu_selector: MenuProgramaAreanaSelector) {
        self.current_menu = menu_selector;
        self.mount();
    }

    fn go_to_child(&mut self, child: MenuProgramaAreanaSelector) {
        // do nothing if child is pointing to itself
        if self.current_menu != child {
            // saves parent
            let parent = self.current_menu;
            match self.navigation_path.push(parent) {
                Ok(_) => (),
                // ERROR DESCRIPTION: `navigation_path` must be redimensioned to higher capacity.
                Err(_) => fatal_error!(107),
            }
            // go to child
            self.go_to_menu(child)
        }
    }

    fn back_to_parent(&mut self) {
        // pops parent from navigation path
        let parent = match self.navigation_path.pop() {
            Some(parent) => parent,
            None => {
                self.must_return_to_main_menu = true;
                self.current_menu
            }
        };
        // go to parent
        self.go_to_menu(parent)
    }

    /// If is in edit mode for some line returns Some(LcdLine) else None.
    fn get_line_being_edited(&self) -> Option<LcdLine> {
        for line in LcdLine::iterator() {
            let is_editing_some_line = self.get_mounted_item_for_lcd_line(line).is_in_edit_mode();
            if is_editing_some_line {
                return Some(line);
            }
        }
        None
    }

    /// helper function to draw submenu cursor on screen
    fn draw_menu_item_selector(&self, line: LcdLine, screen_buffer: &mut ScreenBuffer) {
        const EDITING_CURSOR: u8 = b'*';
        const NAVIGATING_CURSOR: u8 = b'>';
        const EMPTY_CURSOR: u8 = b' ';
        // position cursor
        screen_buffer.set_cursor(Point::new(0, line as u8));
        // draw selector char
        match self.get_line_being_edited() {
            Some(_line) => {
                screen_buffer.print_u8(EDITING_CURSOR);
            }
            None => {
                let is_time_to_blink = self.blink.read();
                if is_time_to_blink {
                    screen_buffer.print_u8(NAVIGATING_CURSOR);
                } else {
                    screen_buffer.print_u8(EMPTY_CURSOR)
                }
            }
        }
    }
}

impl Widget for MenuProgramaControler<'_> {
    /// TODO: Improve this code when possible
    fn send_key(&mut self, key: KeyCode) {
        if let Some(line_being_edited) = self.get_line_being_edited() {
            // if is editing some line, delegate keys to sub widgets.
            let current_menu_item = self.get_mounted_item_for_lcd_line_mut(line_being_edited);
            current_menu_item.send_key(key);
            // if Enter key on a submenu with parameter, after editing the
            // parameter, we need to change to the submenu accordingly.
            if key == KeyCode::KEY_ENTER {
                // we assume here that the field has already had its content saved

                if let Some(child_handle) = current_menu_item.child {
                    current_menu_item.set_edit_mode(false);
                    self.go_to_child(child_handle)
                }
            }
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
                    let current_menu_item = self.get_monted_item_for_current_lcd_line();

                    let has_field = current_menu_item.point_and_field.is_some();

                    if let Some(child_handle) = current_menu_item.child {
                        // TEMP CODE: if current mitem has a child submenu, opens it.
                        if !has_field {
                            // if it is a pure simple submenu (without parameter) jump straight to the submenu on enter
                            self.go_to_child(child_handle)
                        } else {
                            // if sub menu has field (is not simple submenu) then process it first
                            current_menu_item.set_edit_mode(true);
                        };
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

    fn update(&mut self) {
        // updates the blinker
        self.blink.update();
        // updates each line
        for line in LcdLine::iterator() {
            self.get_mounted_item_for_lcd_line_mut(line).update();
        }
    }

    /// TODO: Check if it is possible to remove the unused `_start_point` argument.
    fn draw(&self, screen_buffer: &mut ScreenBuffer, _start_point: Point) {
        // clear screen
        screen_buffer.clear();
        // draw menu item selector
        let line = self.get_navigation_state().get_current_lcd_line();
        self.draw_menu_item_selector(line, screen_buffer);
        // draw menu items
        for line in LcdLine::iterator() {
            self.get_mounted_item_for_lcd_line(line)
                .draw(screen_buffer, line);
        }
    }
}
