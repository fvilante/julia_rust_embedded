use avr_progmem::progmem;

use crate::{board::keyboard::KeyCode, menu::{point::Point, flash::FlashString, canvas::Canvas}};

use super::{widget::{Widget, IWidget}, manual_mode::ManualMode};


progmem! {
    //                             1234567890123456789012345678901234567890
    static progmem string LINE0 = "Selecione modo de programacao desejado";
    static progmem string LINE1 = "MANUAL    EXECUCAO    PROGRAMA";
}

#[derive(PartialEq, Clone, Copy)]
pub enum State {
    MAIN_MENU,
    MANUAL,
    EXECUCAO,
    PROGRAMA,
}


pub struct MainMenu<'a> {
    current_state: State,
    manual_mode_widget: IWidget<'a>,
    //execution_mode: IWidget,
    //program_mode: IWidget,
}

impl<'a> MainMenu<'a> {

    pub fn new(manual_mode: IWidget<'a>) -> Self {
        Self {
            current_state: State::MAIN_MENU,
            manual_mode_widget: manual_mode,
            
        }
    }

    fn draw_main_menu(&self, canvas: &mut Canvas) {
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


        canvas.clear();
        for line_number in 0..2 {
            let ( point, flash_string ) = get_line_helper(line_number);
            canvas.set_cursor(point);
            canvas.print_flash_str(flash_string);
        }
    }

    
}

impl<'a> Widget for MainMenu<'a> {
    
    fn send_key(&mut self, key: KeyCode) {

        match self.current_state {
            State::MAIN_MENU => {
                match key {
                    KeyCode::KEY_MANUAL => self.current_state = State::MANUAL,
                    KeyCode::KEY_EXECUCAO => self.current_state = State::EXECUCAO,
                    KeyCode::KEY_PROGRAMA => self.current_state = State::PROGRAMA,
                    _ => { }
                }
            },
            State::MANUAL => self.manual_mode_widget.send_key(key),
            State::EXECUCAO => todo!(),
            State::PROGRAMA => todo!(),
        }
    }

    fn update(&mut self) {
        match self.current_state {
            State::MAIN_MENU => { },
            State::MANUAL => { self.manual_mode_widget.update()},
            State::EXECUCAO => todo!(),
            State::PROGRAMA => todo!(),
        }
    }

    fn draw(&self, canvas: &mut Canvas) {
        match self.current_state {
            State::MAIN_MENU => { self.draw_main_menu(canvas) },
            State::MANUAL => { self.manual_mode_widget.draw(canvas)},
            State::EXECUCAO => todo!(),
            State::PROGRAMA => todo!(),
        }
    }
}