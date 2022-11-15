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
    Resting,       // Client is responsible to changes to FirstScreeen state
    FirstScreen,
    LastScreen,     // Server is responsible to changes to Resting state
}

pub struct ManualModeMenu {
    pub current_state: ManualModeState,
}

impl ManualModeMenu {
    pub fn new() -> Self {
        Self { 
            current_state: ManualModeState::FirstScreen,
        }
    }

}


impl Widget for ManualModeMenu {
    fn send_key(&mut self, key: KeyCode) {

        match self.current_state {
            ManualModeState::Resting => {
                
            },
            ManualModeState::FirstScreen => {
                if key == KeyCode::KEY_ESC {
                    //back
                    self.current_state = ManualModeState::Resting;
                } else /* Non_ESC key */{ 
                    //continue
                    self.current_state = ManualModeState::LastScreen;
                }
            },
            ManualModeState::LastScreen => {
                self.current_state = ManualModeState::Resting;
            },
        }
    
    }

    fn update(&mut self) {

    }

    fn draw(&self, canvas: &mut Canvas) {
        fn helper_get_first_screen(line_number: u8) -> (Point, FlashString) {
            if line_number == 0 {
                let line0 = FlashString::new(&LINE0);
                let col0 = ((40 - line0.len()) / 2).try_into().unwrap_or(0);
                (Point::new(col0,0), line0)
            } else {
                let line1 = FlashString::new(&LINE1);
                let col1 = ((40 - line1.len()) / 2).try_into().unwrap_or(0);
                (Point::new(col1,1), line1)
            }
        }

        fn helper_get_last_screen(line_number: u8) -> (Point, FlashString) {
            if line_number == 0 {
                let line0 = FlashString::new(&LINE2);
                let col0 = ((40 - line0.len()) / 2).try_into().unwrap_or(0);
                (Point::new(col0,0), line0)
            } else {
                let line1 = FlashString::new(&LINE3);
                let col1 = ((40 - line1.len()) / 2).try_into().unwrap_or(0);
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

        if self.current_state == ManualModeState::FirstScreen {
            draw1(canvas);
        } else if self.current_state == ManualModeState::LastScreen {
            draw2(canvas);
        }
    }
}
