use core::cell::Cell;

use heapless::Vec;

use super::super::widget::{Saveble, Widget};

use crate::board::keypad::KeyCode;

use crate::fatal_error;
use crate::geometry::point::Point;
use crate::menu::screen_buffer::ScreenBuffer;
use crate::menu::widget::widget::Editable;
use crate::microcontroler::ratangular_wave::RectangularWave;
use crate::string::flash::FlashString;
use cross_platform::utils::cursor::Cursor;

// Variable to place the string representing the Optinal Field Parameter in Menu
pub type OptionsBuffer = Vec<FlashString, 3>;

/// TODO: If possible make this function unnecessary and remove it from code. I suppose I'm using
/// it to avoid spread of some `lifetimes`, but I'm not sure it is the best decision.
pub fn make_options_buffer_from_array<const ARRAY_SIZE: usize>(
    options_list: [FlashString; ARRAY_SIZE],
) -> OptionsBuffer {
    let mut options: OptionsBuffer = Vec::new();
    for item in options_list {
        if let Err(_) = options.push(item) {
            // Error: Vector size not enough. Change 'OptionsBuffer' size to a higher value.
            fatal_error!(101);
        }
    }
    options
}

pub struct OptionEditorWidget<'a> {
    options: OptionsBuffer,
    editing_selection: Cursor,
    blink: RectangularWave,
    is_in_edit_mode_: bool,
    variable: &'a Cell<Cursor>,
}

impl<'a> OptionEditorWidget<'a> {
    pub fn new(variable: &'a Cell<Cursor>, options: OptionsBuffer, is_in_edit_mode_: bool) -> Self {
        const T_ON: u16 = 600;
        const T_OFF: u16 = 300;
        let initial_value = variable.get();
        Self {
            options: options.clone(),
            editing_selection: initial_value,
            blink: RectangularWave::new(T_ON, T_OFF),
            is_in_edit_mode_,
            variable,
        }
    }

    /// helper function
    fn blinks_char_if_in_editing_mode(
        &self,
        screen_buffer: &mut ScreenBuffer,
        char: char,
        is_in_editing_mode: bool,
    ) {
        const EMPTY_CHAR: char = ' ';
        if is_in_editing_mode {
            //blinks
            if self.blink.read() {
                screen_buffer.print_char(char);
            } else {
                screen_buffer.print_char(EMPTY_CHAR);
            }
        } else {
            //do not blink
            screen_buffer.print_char(char);
        }
    }
}

impl Saveble for OptionEditorWidget<'_> {
    fn restore_value(&mut self) {
        let initial_value = self.variable.get();
        self.editing_selection = initial_value;
    }

    fn save_value(&mut self) {
        let edited_value = self.editing_selection;
        self.variable.set(edited_value);
    }
}

impl Editable for OptionEditorWidget<'_> {
    fn set_edit_mode(&mut self, value: bool) {
        self.is_in_edit_mode_ = value;
    }

    fn is_in_edit_mode(&self) -> bool {
        self.is_in_edit_mode_
    }
}

impl Widget for OptionEditorWidget<'_> {
    fn send_key(&mut self, key: KeyCode) {
        match key {
            // navigation_key left and right
            KeyCode::KEY_SETA_BRANCA_ESQUERDA => {
                self.editing_selection.previous_wrap_around();
            }
            KeyCode::KEY_SETA_BRANCA_DIREITA => {
                self.editing_selection.next_wrap_around();
            }
            KeyCode::KEY_DIRECIONAL_PARA_DIREITA => {
                self.editing_selection.next_wrap_around();
            }
            KeyCode::KEY_DIRECIONAL_PARA_ESQUERDA => {
                self.editing_selection.previous_wrap_around();
            }

            //everything else
            _ => {}
        };
    }

    fn update(&mut self) {
        self.blink.update();
    }

    fn draw(&self, screen_buffer: &mut ScreenBuffer, start_point: Point) {
        screen_buffer.set_cursor(start_point);
        const OPEN_BRACKETS: char = '[';
        const CLOSE_BRACKETS: char = ']';
        let is_in_editing_mode = self.is_in_edit_mode();
        let current_index = self.editing_selection.get_current();
        self.blinks_char_if_in_editing_mode(screen_buffer, OPEN_BRACKETS, is_in_editing_mode);
        let flash_string = self.options[current_index as usize];
        screen_buffer.print(flash_string);
        self.blinks_char_if_in_editing_mode(screen_buffer, CLOSE_BRACKETS, is_in_editing_mode);
    }
}
