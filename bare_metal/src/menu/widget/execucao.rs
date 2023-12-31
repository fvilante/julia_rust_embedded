//menu "execucao"

use crate::geometry::point::Point;
use crate::string::flash::FlashString;
use crate::{board::keypad::KeyCode, menu::screen_buffer::ScreenBuffer};
use avr_progmem::progmem;
use cross_platform::protocol::transport::transport_layer::{
    new_proposal::Displacement, TransportLayer,
};

use super::widget::Widget;

progmem! {
    //                             1234567890123456789012345678901234567890
    static progmem string LINE0 = "Posicao atual:";
    static progmem string LINE1 = "X =       mm"; //"${nnnn}    Y=${nnnn}";
}

pub struct MenuExecucaoControler<'a> {
    transport: &'a TransportLayer<'a>,
}

impl<'a> MenuExecucaoControler<'a> {
    pub fn new(transport: &'a TransportLayer<'a>) -> Self {
        Self { transport }
    }

    fn get_line_helper(line_number: u8) -> (Point, impl IntoIterator<Item = u8>) {
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

impl<'a> Widget for MenuExecucaoControler<'a> {
    fn send_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::KEY_START => {
                let _unwrap = self.transport.start(); // TODO: The use of `.unwrap` method is provoking flash size explosion, check why
            }
            KeyCode::KEY_STOP => {
                let _unwrap = self.transport.stop(); // TODO: The use of `.unwrap` method is provoking flash size explosion, check why
            }
            _ => {}
        }
    }

    fn update(&mut self) {}

    fn draw(&self, screen_buffer: &mut ScreenBuffer, _start_point: Point) {
        screen_buffer.clear();
        // draw screen frame
        for line_number in 0..2 {
            let (point, text) = Self::get_line_helper(line_number);
            screen_buffer.set_cursor(point);
            screen_buffer.print(text);
        }
        // draw current position
        let posicao_atual = self.transport.posicao_atual();
        if let Ok(Displacement(posicao_atual)) = posicao_atual {
            screen_buffer.set_cursor(Point::new(18, 1));
            screen_buffer.print_u16(posicao_atual);
        }
    }
}
