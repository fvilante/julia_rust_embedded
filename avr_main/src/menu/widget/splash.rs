use avr_progmem::progmem;
use lib_1::protocol::transport::transport_layer::TransportLayer;

use crate::{
    board::{keyboard::KeyCode, lcd},
    menu::{canvas::Canvas, flash::FlashString, model::DataStorage, point::Point},
    microcontroler::{delay::delay_ms, timer::now},
};

use super::widget::Widget;

progmem! {
    static progmem string TEXT0 = "Posijet Industria e Comercio Ltda.";
    pub static progmem string POR_FAVOR_AGUARDE_CARGA_DO_PROGRAMA_X = "Por favor aguarde a carga do programa X";
    static progmem string TEXT2 = "Por favor aguarde a carga do programa Y";
}

// SPLASH SCREEN RECIEPE (from on original TTC3100 Z80):
//      * lcd_cursor(col=4,lin=0) - print "Posijet Industria e Comercio Ltda."
//      * ~2 segundos
//      * clearscreen
//      * lcd_cursor(col=0,lin=1) - print "Por favor aguarde a carga do programa X"
//      * ~2 segundos se equipto estiver conectado ou 0.5 segundos se nao estiver
//      * clearcreeen
//      * lcd_cursor(col=0,lin=0) - print "Por favor aguarde a carga do programa X"
//      * ~2 segundos se equipto estiver conectado ou 0.5 segundos se nao estiver
//      * GO_TO_MENU_PRINCIPAL

#[repr(u8)]
#[derive(PartialEq, Clone, Copy)]
enum State {
    Initial = 0,
    BrandName = 1,
    LoadingX = 2,
    LoadingY = 3,
    End = 4,
}

impl State {
    fn from_u8(state: u8) -> Self {
        match state {
            0 => Self::Initial,
            1 => Self::BrandName,
            2 => Self::LoadingX,
            3 => Self::LoadingY,
            4 => Self::End,
            _ => Self::End,
        }
    }

    fn as_u8(&self) -> u8 {
        *self as u8
    }

    fn next_state(&self) -> Self {
        let current_state = self;
        let next_state = current_state.as_u8() + 1;
        State::from_u8(next_state)
    }
}

pub struct Splash<'a> {
    current_state: State,
    next_state_time_point: u32,
    model: &'a DataStorage,
    transport: &'a TransportLayer<'a>,
}

impl<'a> Splash<'a> {
    pub fn new(model: &'a DataStorage, transport: &'a TransportLayer<'a>) -> Self {
        let initial_state = State::Initial;
        Self {
            current_state: initial_state,
            next_state_time_point: now() as u32 + Self::get_time_to_wait_in(initial_state),
            model,
            transport,
        }
    }

    /// gets time interval (in miliseconds) to wait until reach next state
    fn get_time_to_wait_in(current_state: State) -> u32 {
        match current_state {
            State::Initial => 0,
            State::BrandName => 2000,
            State::LoadingX => 2000,
            State::LoadingY => 0,
            State::End => 0,
        }
    }

    pub fn is_running(&self) -> bool {
        self.current_state != State::End
    }
}

impl Splash<'_> {
    pub fn send_key(&mut self, _key: KeyCode) {
        // do nothing
    }

    pub fn update(&mut self) {
        let has_finished = self.current_state == State::End;
        if !has_finished {
            if now() as u32 > self.next_state_time_point {
                self.current_state = self.current_state.next_state();
                let time_interval = Self::get_time_to_wait_in(self.current_state.clone());
                self.next_state_time_point = now() as u32 + time_interval;
            }
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.clear();
        match self.current_state {
            State::Initial => {}
            State::BrandName => {
                canvas.set_cursor(Point::new(4, 0));
                canvas.print_iterable(FlashString::new(&TEXT0));
            }
            State::LoadingX => {
                canvas.set_cursor(Point::new(0, 1));
                canvas.print_iterable(FlashString::new(&POR_FAVOR_AGUARDE_CARGA_DO_PROGRAMA_X));
                // TODO: Move this effect to `update` method when possible
                for response in self.model.send_all(&self.transport) {
                    if let Err(e) = response {
                        lcd::clear();
                        lcd::set_cursor(0, 0);
                        lcd::print("Erro de comunicacao serial");
                        delay_ms(4000);
                        break;
                    }
                }
            }
            State::LoadingY => {
                canvas.set_cursor(Point::new(0, 0));
                canvas.print_iterable(FlashString::new(&TEXT2));
            }
            State::End => {
                // do nothing
            }
        }
    }
}
