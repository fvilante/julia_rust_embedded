use avr_progmem::progmem;
use cross_platform::protocol::transport::transport_layer::{new_proposal::ActivationState, TransportLayer};

use super::widget::Widget;
use crate::geometry::point::Point;
use crate::string::flash::FlashString;
use crate::{board::keypad::KeyCode, menu::canvas::Canvas};
progmem! {
    //                            1234567890123456789012345678901234567890
    static progmem string LINE0 = "Aperte qualquer tecla para entrar";
    static progmem string LINE1 = "em modo manual ou ESC para retornar...";
    static progmem string LINE2 = "Equipamento em modo manual";
    static progmem string LINE3 = "pressione qualquer tecla para retornar...";

}

#[derive(PartialEq, Copy, Clone)]
pub enum ManualModeState {
    Resting,     // client is responsible to change from Rsting to FirstScreen state
    FirstScreen, // start screen
    LastScreen,  // Server is responsible to changes to Resting state
}

pub struct ManualModeMenuControler<'a> {
    pub current_state: ManualModeState,
    transport: &'a TransportLayer<'a>,
}

impl<'a> ManualModeMenuControler<'a> {
    pub fn new(transport: &'a TransportLayer<'a>) -> Self {
        Self {
            current_state: ManualModeState::FirstScreen,
            transport,
        }
    }
}

impl Widget for ManualModeMenuControler<'_> {
    fn send_key(&mut self, key: KeyCode) {
        match self.current_state {
            ManualModeState::Resting => {}
            ManualModeState::FirstScreen => {
                if key == KeyCode::KEY_ESC {
                    //Esc pressed, then go back
                    self.current_state = ManualModeState::Resting;
                } else {
                    //else continue
                    self.current_state = ManualModeState::LastScreen;
                }
            }
            ManualModeState::LastScreen => {
                self.current_state = ManualModeState::Resting;
            }
        }
    }

    fn update(&mut self) {}

    fn draw(&self, canvas: &mut Canvas, _start_point: Point) {
        if self.current_state == ManualModeState::FirstScreen {
            canvas.clear();
            canvas.set_cursor(Point::new(0, 0));
            canvas.print(FlashString::new(&LINE0));
            canvas.set_cursor(Point::new(0, 1));
            canvas.print(FlashString::new(&LINE1));
        } else if self.current_state == ManualModeState::LastScreen {
            canvas.clear();
            canvas.set_cursor(Point::new(0, 0));
            canvas.print(FlashString::new(&LINE2));
            canvas.set_cursor(Point::new(0, 1));
            canvas.print(FlashString::new(&LINE3));
            //TODO: below effect should be in `update` and not in `draw` method
            //TODO: The use of `.unwrap` method is provoking flash size explosion, check why
            let _unwrap = self.transport.stop_serial().set(ActivationState::Activated);
            let _unwrap = self
                .transport
                .pausa_serial()
                .set(ActivationState::Activated);
        }
    }
}
