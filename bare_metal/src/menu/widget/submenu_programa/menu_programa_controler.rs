use core::cell::Cell;

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
            menu_arena.get_item(current_menu, 1),
        ) else {
            // TODO: currently we do not accept submenus with less then 2 menu_items, accept it when possible
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

    // -----------------------------------------------------------
    //           GENERAL FUNCTIONS
    // -----------------------------------------------------------

    fn retrieve_current_menu_navigation_state(&self) -> &Cell<NavigationStateModel> {
        self.menu_arena.get_navigation_state(self.current_menu)
    }

    /// Mount widgets that are being renderized
    /// TODO: Consider rename to `redraw`. (Hum! Maybe no because I'm saving the Widgets but not
    /// running the .draw method of it. It represents just some internal `model` change)
    /// TODO: Check if this functoin can be reused in the constructor (Self::new). Because it seems that
    /// the code is duplicated.
    fn mount(&mut self) {
        // Algorithm: For each line of the Lcd recriates the menu_item Widgets based in the
        // current navigation state and overwrite old widgets.
        for lcd_line in LcdLine::iterator() {
            let current_menu_item = self.menu_arena.get_item(
                self.current_menu,
                self.retrieve_current_menu_navigation_state()
                    .get()
                    .get_current_index_for(lcd_line) as usize,
            );
            let mounted = self.mounted.get_mut(lcd_line as u8 as usize);
            if let (Some(menu_item_widget), Some(mounting_place)) = (current_menu_item, mounted) {
                *mounting_place = menu_item_widget;
            } else {
                fatal_error!(103); // Menu mounting error
            };
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
    fn get_selected_menu_item(&mut self) -> &mut MenuItemWidget<'a> {
        let selected_line = self
            .retrieve_current_menu_navigation_state()
            .get()
            .get_current_lcd_line();
        self.get_mounted_item_for_lcd_line_mut(selected_line)
    }

    // -----------------------------------------------------------
    //           MENUS (INTERNAL) NAVIGATION
    // -----------------------------------------------------------

    fn key_down(&mut self) {
        let size_of_menu = self.menu_arena.len(self.current_menu) as u8;
        self.retrieve_current_menu_navigation_state()
            .update(|mut nav| {
                nav.key_down(size_of_menu);
                nav
            });
        self.mount();
    }

    fn key_up(&mut self) {
        self.retrieve_current_menu_navigation_state()
            .update(|mut nav| {
                nav.key_up();
                nav
            });
        self.mount();
    }

    // -----------------------------------------------------------
    //           MENUS (EXTERNAL) NAVIGATION
    // -----------------------------------------------------------

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

    // -----------------------------------------------------------
    //            LINE BEING EDITED
    // -----------------------------------------------------------

    /// If is in edit mode for some line returns Some(LcdLine) else None.
    /// TODO: Why not return Option<&mut MenuItemWidget> instead ?
    fn get_line_being_edited(&self) -> Option<LcdLine> {
        self.mounted
            .iter()
            .filter(|item| item.is_in_edit_mode())
            .enumerate()
            .next()
            .map(|x| (x.0 as u8).into())
    }

    // -----------------------------------------------------------
    //            DRAWING HELPER
    // -----------------------------------------------------------

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

// -----------------------------------------------------------
//            WIDGET IMPLEMENTATION
// -----------------------------------------------------------

impl<'a> Widget for MenuProgramaControler<'a> {
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
                    let current_menu_item = self.get_selected_menu_item();

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
        let line = self
            .retrieve_current_menu_navigation_state()
            .get()
            .get_current_lcd_line();
        self.draw_menu_item_selector(line, screen_buffer);
        // draw menu items
        for line in LcdLine::iterator() {
            self.get_mounted_item_for_lcd_line(line)
                .draw(screen_buffer, line);
        }
    }
}
