use heapless::Vec;

use crate::board::keyboard::KeyCode;
use crate::menu::accessor::Accessor;
use crate::menu::canvas::Canvas;
use crate::menu::flash::FlashString;
use crate::menu::point::Point;
use crate::menu::ratangular_wave::RectangularWave;
use crate::menu::widget::widget::Editable;
use super::cursor::Cursor;
use super::edit_mode::EditMode;

type OptionsBuffer = Vec<FlashString,5>;

pub struct Optional {
    edit_mode: EditMode,
    options: OptionsBuffer,
    editing_cursor: Cursor,
    original_cursor: Cursor,
    blink: RectangularWave<u32>,
    accessor: Accessor<Cursor>,
}

impl Optional {

    pub fn new(options: OptionsBuffer, accessor: Accessor<Cursor>) -> Self {
        let cursor = accessor.get();
        Self {
            edit_mode: EditMode::new(false),
            options: options.clone(),
            editing_cursor: cursor.clone(),
            original_cursor: cursor,
            blink: RectangularWave::new(1000,1000),
            accessor,
        }
    }

    /// helper function
    fn __blinks_char_if_in_editing_mode(&self, canvas: &mut Canvas, char: char) {
        const empty_char: char = ' ';
        if self.is_in_edit_mode() {
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

    fn __abort_edition(&mut self) {
        let recupered_info = self.original_cursor.clone();
        self.editing_cursor = recupered_info.clone();   // resets cursor
        self.accessor.set(recupered_info);  // saves it
    }

    fn __saves_data(&mut self) {
        let info_to_save = self.editing_cursor.clone();
        self.original_cursor = info_to_save.clone();  
        self.accessor.set(info_to_save);
    }

}

impl Optional {
    pub fn send_key(&mut self, key: KeyCode) {
        if self.is_in_edit_mode() {

            let effect = match key {
                // cancel edition
                KeyCode::KEY_ESC => {
                    self.set_edit_mode(false);
                    self.__abort_edition();
                    Some(())
                }
                // saves edition
                KeyCode::KEY_ENTER => {
                    self.set_edit_mode(false);
                    self.__saves_data();
                    Some(())
                }

                // navigation_key left and right
                KeyCode::KEY_SETA_BRANCA_ESQUERDA => { self.editing_cursor.previous_wrap_around(); Some(()) }, 
                KeyCode::KEY_SETA_BRANCA_DIREITA => { self.editing_cursor.next_wrap_around(); Some(()) },
                KeyCode::KEY_DIRECIONAL_PARA_DIREITA => { self.editing_cursor.next_wrap_around(); Some(()) },
                KeyCode::KEY_DIRECIONAL_PARA_ESQUERDA => { self.editing_cursor.previous_wrap_around(); Some(()) },

                //everything else
                _ => { None },
            };

            // reset the blinker when some key is pressed makes a better visual effect
            if let Some(_) = effect {
                self.blink.reset();
            }  
        } else {
            // ignore keys
        }

    }

    pub fn update(&mut self) {
        self.blink.update();
    }

    pub fn draw(&self, canvas: &mut Canvas, start_point: Point) {
        canvas.clear();
        const open_brackets: char = '[';
        const close_brackets: char = ']';
        let current_index = self.editing_cursor.get_current();
        self.__blinks_char_if_in_editing_mode(canvas, open_brackets);
        let flash_string = self.options[current_index];
        canvas.print_flash_str(flash_string);
        self.__blinks_char_if_in_editing_mode(canvas, close_brackets);
        
    }
}


impl Optional {
    pub fn set_edit_mode(&mut self, value: bool) {
        self.edit_mode.set_edit_mode(value);
    }

    pub fn is_in_edit_mode(&self) -> bool {
        self.edit_mode.is_in_edit_mode()
    }
}