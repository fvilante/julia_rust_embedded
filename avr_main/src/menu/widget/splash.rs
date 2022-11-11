use avr_progmem::progmem;

use crate::{microcontroler::timer::now, menu::{flash::FlashString, point::Point, canvas::Canvas}, board::keyboard::KeyCode};

use super::{widget::Widget, cursor::Cursor};


progmem! {
    static progmem string TEXT0 = "Posijet Industria e Comercio Ltda.";
    static progmem string TEXT1 = "Por favor aguarde a carga do programa X";
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
    pub fn from_u8(state: u8) -> Self {
        match state {
            0 => Self::Initial,
            1 => Self::BrandName,
            2 => Self::LoadingX,
            3 => Self::LoadingY,
            4 => Self::End,
            _ => Self::End,
        }
    }

    pub fn as_u8(&self) -> u8 {
        *self as u8
    }

    pub fn next_state(&self) -> Self {
        let current_state = self;
        let next_state = current_state.as_u8() + 1;
        State::from_u8(next_state)
    }
}

pub struct Splash {
    current_state: State,
    next_state_time_point: u64,
    //cursor: Cursor,
}

impl Splash {
    pub fn new() -> Self {
        let initial_state = State::Initial;
        Self { 
           current_state: initial_state,
           next_state_time_point: now() + Self::get_time_to_wait_in(initial_state), 
           //cursor: Cursor::new(State::Initial..State::End, State::Initial),
        }
    }

    /// gets interval time to wait until reach next state
    fn get_time_to_wait_in(current_state: State) -> u64 {
        match current_state {
            State::Initial => 0,
            State::BrandName => 2000,
            State::LoadingX => 2000,
            State::LoadingY => 2000,
            State::End => 0, 
        }
    }

}

impl Widget for Splash {
    fn send_key(&mut self, key: KeyCode) {
        let has_finished = self.current_state == State::End;
        if has_finished == false {
            // do nothing
        } else {
            // delegate / by-pass
        }
    }

    fn update(&mut self) {
        let has_finished = self.current_state == State::End;
        if !has_finished {
            if now() > self.next_state_time_point {
                self.current_state = self.current_state.next_state();
                let time_interval = Self::get_time_to_wait_in(self.current_state.clone());
                self.next_state_time_point = now() + time_interval;
            }
        } else {
            // delegate
        }
        
    }

    fn draw(&self, canvas: &mut Canvas) {
        canvas.clear();
        match self.current_state {
            State::Initial => { },
            State::BrandName => canvas.print_xy(Point::new(4, 0), FlashString::new(&TEXT0)),
            State::LoadingX => canvas.print_xy(Point::new(0, 1), FlashString::new(&TEXT1)),
            State::LoadingY => canvas.print_xy(Point::new(0, 0), FlashString::new(&TEXT2)),
            State::End => {
                //delegate / by-pass
            },
        }
    }
}