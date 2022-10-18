use avr_progmem::progmem;

use crate::{board::keyboard::KeyCode, menu::{point::Point, flash::FlashString}};

use super::widget::Widget;


progmem! {
    //                             1234567890123456789012345678901234567890
    static progmem string LINE2 = "Selecione modo de programacao desejado";
    static progmem string LINE3 = "MANUAL    EXECUCAO    PROGRAMA";
}

#[derive(PartialEq)]
enum State {
    IDLE,
    MANUAL,
    EXECUCAO,
    PROGRAMA,
}

pub struct MainMenu {
    current_state: State,
}

impl MainMenu {
    pub fn new() -> Self {
        Self {
            current_state: State::IDLE,
        }
    }

    fn get_line_helper(line_number: u8) -> (Point, FlashString) {
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
}

impl Widget for MainMenu {
    
    fn send_key(&mut self, key: crate::board::keyboard::KeyCode) {
        match key {
            KeyCode::KEY_MANUAL => {
                self.current_state = State::MANUAL;
            }

            KeyCode::KEY_EXECUCAO => {
                self.current_state = State::EXECUCAO;
            }

            KeyCode::KEY_PROGRAMA => {
                self.current_state = State::PROGRAMA;
            }

            _ => {

            }
        }
    }

    fn update(&mut self) {
        
    }

    fn draw(&self, canvas: &mut crate::menu::canvas::Canvas) {
        if self.current_state == State::IDLE {
            canvas.clear();
            for line_number in 0..2 {
                let ( point, flash_string ) = Self::get_line_helper(line_number);
                canvas.set_cursor(point);
                canvas.print_flash_str(flash_string);
            }
        } else {
            canvas.clear();
        }
        
    }
}