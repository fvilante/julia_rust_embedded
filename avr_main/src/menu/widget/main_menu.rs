use avr_progmem::progmem;

use crate::{board::keyboard::KeyCode, menu::{point::Point, flash::FlashString}};

use super::widget::Widget;


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

pub struct MainMenu;

impl MainMenu {

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

    pub fn draw(canvas: &mut crate::menu::canvas::Canvas) {
        canvas.clear();
        for line_number in 0..2 {
            let ( point, flash_string ) = Self::get_line_helper(line_number);
            canvas.set_cursor(point);
            canvas.print_flash_str(flash_string);
        }
    }
}