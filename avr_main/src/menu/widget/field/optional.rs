use core::cell::Cell;

use heapless::Vec;

use super::super::widget::{Saveble, Widget};
use super::edit_mode::EditMode;
use crate::board::keyboard::KeyCode;

use crate::menu::canvas::Canvas;
use crate::menu::flash::FlashString;
use crate::menu::point::Point;
use crate::menu::ratangular_wave::RectangularWave;
use crate::menu::widget::widget::Editable;
use lib_1::utils::cursor::Cursor;

pub type OptionsBuffer = Vec<FlashString, 5>;

/// TODO: If possible make this function unnecessary and remove it from code. I suppose I'm using
/// it to avoid spread of some `lifetimes`, but I'm not sure it is the best decision.
pub fn make_options_buffer_from_array<const ARRAY_SIZE: usize>(
    options_list: [FlashString; ARRAY_SIZE],
) -> OptionsBuffer {
    let mut options: OptionsBuffer = Vec::new();
    for item in options_list {
        match options.push(item) {
            Ok(_) => {}
            Err(_) => {
                panic!("Err10");
                // Error: Vector size not enough. Change 'OptionsBuffer' size to a higher value.
            }
        }
    }
    options
}

pub struct OptionEditorWidget<'a> {
    options: OptionsBuffer,
    editing_selection: Cursor,
    blink: RectangularWave,
    edit_mode: EditMode,
    variable: &'a Cell<Cursor>,
}

impl<'a> OptionEditorWidget<'a> {
    pub fn new(variable: &'a Cell<Cursor>, options: OptionsBuffer, is_in_edit_mode: bool) -> Self {
        const T_ON: u64 = 600;
        const T_OFF: u64 = 300;
        let initial_value = variable.get();
        Self {
            options: options.clone(),
            editing_selection: initial_value,
            blink: RectangularWave::new(T_ON, T_OFF),
            edit_mode: EditMode::new(is_in_edit_mode),
            variable,
        }
    }

    /// helper function
    fn blinks_char_if_in_editing_mode(
        &self,
        canvas: &mut Canvas,
        char: char,
        is_in_editing_mode: bool,
    ) {
        const EMPTY_CHAR: char = ' ';
        if is_in_editing_mode {
            //blinks
            if self.blink.read() {
                canvas.print_char(char);
            } else {
                canvas.print_char(EMPTY_CHAR);
            }
        } else {
            //do not blink
            canvas.print_char(char);
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
        self.edit_mode.set_edit_mode(value)
    }

    fn is_in_edit_mode(&self) -> bool {
        self.edit_mode.is_in_edit_mode()
    }
}

/* impl Optional<'_> {
    pub fn abort_edition(&mut self) {
        let recupered_info = self.original_cursor.clone();
        self.editing_cursor = recupered_info.clone(); // resets cursor
        *self.variable = recupered_info; // saves it
    }

    pub fn save_edition(&mut self) {
        let info_to_save = self.editing_cursor.clone();
        self.original_cursor = info_to_save.clone();
        *self.variable = info_to_save;
    }
} */

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

    fn draw(&self, canvas: &mut Canvas, start_point: Point) {
        canvas.set_cursor(start_point);
        const OPEN_BRACKETS: char = '[';
        const CLOSE_BRACKETS: char = ']';
        let is_in_editing_mode = self.is_in_edit_mode();
        let current_index = self.editing_selection.get_current();
        self.blinks_char_if_in_editing_mode(canvas, OPEN_BRACKETS, is_in_editing_mode);
        let flash_string = self.options[current_index as usize];
        canvas.print_flash_str(flash_string);
        self.blinks_char_if_in_editing_mode(canvas, CLOSE_BRACKETS, is_in_editing_mode);
    }
}
