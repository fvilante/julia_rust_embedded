use heapless::Vec;

use super::edit_mode::EditMode;
use super::widget::Widget;
use crate::board::keyboard::KeyCode;
use crate::menu::accessor::Accessor;
use crate::menu::canvas::Canvas;
use crate::menu::flash::FlashString;
use crate::menu::point::Point;
use crate::menu::ratangular_wave::RectangularWave;
use crate::menu::widget::widget::Editable;
use lib_1::utils::cursor::Cursor;

pub type OptionsBuffer = Vec<FlashString, 5>;

pub fn make_options_buffer_from_array<const ArraySize: usize>(
    options_list: [FlashString; ArraySize],
) -> OptionsBuffer {
    let mut options: OptionsBuffer = Vec::new();
    for item in options_list {
        options.push(item);
    }
    options
}

pub struct Optional {
    options: OptionsBuffer,
    editing_selection: Cursor,
    original_selection: Cursor,
    blink: RectangularWave,
}

impl Optional {
    pub fn new(initial_selection: Cursor, options: OptionsBuffer) -> Self {
        const T_ON: u64 = 600;
        const T_OFF: u64 = 300;
        Self {
            options: options.clone(),
            editing_selection: initial_selection.clone(),
            original_selection: initial_selection,
            blink: RectangularWave::new(T_ON, T_OFF),
        }
    }

    /// helper function
    fn __blinks_char_if_in_editing_mode(
        &self,
        canvas: &mut Canvas,
        char: char,
        is_in_editing_mode: bool,
    ) {
        const empty_char: char = ' ';
        if is_in_editing_mode {
            //blinks
            if self.blink.read() {
                canvas.print_char(char);
            } else {
                canvas.print_char(empty_char);
            }
        } else {
            //do not blink
            canvas.print_char(char);
        }
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

impl Widget for Optional {
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
        const is_in_editing_mode: bool = true; // TODO: remove the necessity of this constant if and when possible
        canvas.set_cursor(start_point);
        const open_brackets: char = '[';
        const close_brackets: char = ']';
        let current_index = self.editing_selection.get_current();
        self.__blinks_char_if_in_editing_mode(canvas, open_brackets, is_in_editing_mode);
        let flash_string = self.options[current_index as usize];
        canvas.print_flash_str(flash_string);
        self.__blinks_char_if_in_editing_mode(canvas, close_brackets, is_in_editing_mode);
    }
}
