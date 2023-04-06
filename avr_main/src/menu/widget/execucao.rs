//menu "execucao"

use crate::{
    board::keyboard::KeyCode,
    menu::{canvas::Canvas, flash::FlashString, point::Point},
};
use avr_progmem::progmem;
use lib_1::protocol::transport::transport_layer::{new_proposal::Displacement, TransportLayer};

use super::widget::Widget;

progmem! {
    //                             1234567890123456789012345678901234567890
    static progmem string LINE0 = "Posicao atual:";
    static progmem string LINE1 = "X =       mm"; //"${nnnn}    Y=${nnnn}";
}

pub struct MenuExecucao<'a> {
    transport: &'a TransportLayer<'a>,
}

impl<'a> MenuExecucao<'a> {
    pub fn new(transport: &'a TransportLayer<'a>) -> Self {
        Self { transport }
    }

    fn get_line_helper(line_number: u8) -> (Point, FlashString) {
        let line0 = FlashString::new(&LINE0);
        let line1 = FlashString::new(&LINE1);
        let col0 = ((40 - line0.len()) / 2).try_into().unwrap_or(0);
        let col1 = ((40 - line1.len()) / 2).try_into().unwrap_or(0);
        if line_number == 0 {
            (Point::new(col0, 0), line0)
        } else {
            (Point::new(col1, 1), line1)
        }
    }
}

impl<'a> Widget for MenuExecucao<'a> {
    fn send_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::KEY_START => {
                self.transport.start();
            }
            KeyCode::KEY_STOP => {
                //lcd::clear();
                //lcd::print("Juca");
                self.transport.stop();
                //loop { }
            }
            _ => {}
        }
    }

    fn update(&mut self) {}

    fn draw(&self, canvas: &mut Canvas, _start_point: Point) {
        canvas.clear();
        // draw screen frame
        for line_number in 0..2 {
            let (point, flash_string) = Self::get_line_helper(line_number);
            canvas.set_cursor(point);
            canvas.print_flash_str(flash_string);
        }
        // draw current position
        let posicao_atual = self.transport.posicao_atual();
        if let Ok(Displacement(posicao_atual)) = posicao_atual {
            canvas.set_cursor(Point::new(18, 1));
            canvas.print_u16(posicao_atual);
        }
    }
}
