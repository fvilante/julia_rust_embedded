use avr_progmem::progmem;

use crate::{microcontroler::timer::now, menu::{flash::FlashString, point::Point, canvas::Canvas}, board::keyboard::KeyCode};

use super::widget::Widget;


progmem! {
    //                            1234567890123456789012345678901234567890
    static progmem string LINE0 = "Aperte qualquer tecla para entrar";
    static progmem string LINE1 = "em modo manual ou ESC para retornar...";
    static progmem string LINE2 = "Equipamento em modo manual";
    static progmem string LINE3 = "pressione qualquer tecla para retornar...";
    
}

#[derive(PartialEq, Copy, Clone)]
pub enum ManualModeState {
    DISABLED,
    FIRST_SCREEN,
    LAST_SCREEN,
}

pub struct ManualMode {
    pub current_state: ManualModeState,
}

impl ManualMode {
    pub fn new() -> Self {
        Self { 
            current_state: ManualModeState::FIRST_SCREEN,
        }
    }

    pub fn send_key(&mut self, key: crate::board::keyboard::KeyCode) {
        if self.current_state == ManualModeState::FIRST_SCREEN {
            if key == KeyCode::KEY_ESC {
                self.current_state = ManualModeState::DISABLED;
            } else {
                //TODO: Put motor in manual mode
                self.current_state = ManualModeState::LAST_SCREEN;
            }
        } else if self.current_state == ManualModeState::LAST_SCREEN {
            self.current_state = ManualModeState::DISABLED;
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        fn helper_get_first_screen(line_number: u8) -> (Point, FlashString) {
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
    
        fn helper_get_last_screen(line_number: u8) -> (Point, FlashString) {
            let line0 = FlashString::new(&LINE2);
            let line1 = FlashString::new(&LINE3);
            let col0 = ((40 - line0.len()) / 2).try_into().unwrap_or(0);
            let col1 = ((40 - line1.len()) / 2).try_into().unwrap_or(0);
            if line_number == 0 {
                (Point::new(col0,0), line0)
            } else {
                (Point::new(col1,1), line1)
            }
        }
        fn draw1(canvas: &mut Canvas) {
            canvas.clear();
            for line_number in 0..2 {
                let ( point, flash_string ) = helper_get_first_screen(line_number);
                canvas.set_cursor(point);
                canvas.print_flash_str(flash_string);
            }
        }
    
        fn draw2(canvas: &mut crate::menu::canvas::Canvas) {
            canvas.clear();
            for line_number in 0..2 {
                let ( point, flash_string ) = helper_get_last_screen(line_number);
                canvas.set_cursor(point);
                canvas.print_flash_str(flash_string);
            }
        }

        if self.current_state == ManualModeState::FIRST_SCREEN {
            draw1(canvas);
        } else if self.current_state == ManualModeState::LAST_SCREEN {
            draw2(canvas);
        }
    }

    

}

