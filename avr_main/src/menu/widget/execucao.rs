//menu "execucao"

use avr_progmem::progmem;

use crate::{board::keyboard::KeyCode, menu::{point::Point, flash::FlashString}};

use super::widget::Widget;

progmem! {
    //                             1234567890123456789012345678901234567890
    static progmem string LINE0 = "Posicao atual:";
    static progmem string LINE1 = "X=${nnnn}    Y=${nnnn}";
}

pub struct Execucao {
    pub is_running: bool,
}

impl Execucao {
    pub fn new() -> Self {
        Self {
            is_running: false,
        }
    }

    fn get_line_helper(line_number: u8) -> (Point, FlashString) {
        let line0 = FlashString::new(&LINE0);
        let line1 = FlashString::new(&LINE1);
        let col0 = ((40 - line0.len()) / 2).try_into().unwrap_or(0);
        let col1 = ((40 - line1.len()) / 2).try_into().unwrap_or(0);
        if line_number == 0 {
            (Point::new(col0,0), line0)
        } else {
            (Point::new(col1,1), line1)
        }
    }
}

impl Widget for Execucao {
    fn send_key(&mut self, key: crate::board::keyboard::KeyCode) {
        match key {
            KeyCode::KEY_ESC => {
                self.is_running = false;
            }

            _ => { 

            }
        }
    }

    fn update(&mut self) {
        
    }

    fn draw(&self, canvas: &mut crate::menu::canvas::Canvas) {
        if self.is_running {
            canvas.clear();
            for line_number in 0..2 {
                let ( point, flash_string ) = Self::get_line_helper(line_number);
                canvas.set_cursor(point);
                canvas.print_flash_str(flash_string);
            }
        } else {

        }
        
    }
}
