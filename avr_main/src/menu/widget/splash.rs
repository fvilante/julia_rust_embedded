use avr_progmem::progmem;

use crate::{
    board::keyboard::KeyCode,
    menu::{canvas::Canvas, point::Point},
    microcontroler::timer::now,
    utils::generic_string::GenericString,
};

use super::widget::{IWidget, Widget};

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

pub struct Splash<'a> {
    current_state: State,
    next_state_time_point: u32,
    /// pointer to the next widget to be loaded
    widget: Option<IWidget<'a>>,
}

impl<'a> Splash<'a> {
    pub fn new(widget: Option<IWidget<'a>>) -> Self {
        let initial_state = State::Initial;
        Self {
            current_state: initial_state,
            next_state_time_point: now() + Self::get_time_to_wait_in(initial_state),
            widget,
        }
    }

    /// gets time interval (in miliseconds) to wait until reach next state
    fn get_time_to_wait_in(current_state: State) -> u32 {
        match current_state {
            State::Initial => 0,
            State::BrandName => 2000,
            State::LoadingX => 2000,
            State::LoadingY => 2000,
            State::End => 0,
        }
    }
}

impl<'a> Widget for Splash<'a> {
    fn send_key(&mut self, key: KeyCode) {
        let has_finished = self.current_state == State::End;
        if has_finished == false {
            // do nothing
        } else {
            // delegate / by-pass to next widget
            if let Some(widget) = &mut self.widget {
                (*widget).send_key(key)
            }
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
            if let Some(widget) = &mut self.widget {
                (*widget).update()
            }
        }
    }

    fn draw(&self, canvas: &mut Canvas, start_point: Point) {
        canvas.clear();
        match self.current_state {
            State::Initial => {}
            State::BrandName => {
                canvas.print_xy(Point::new(4, 0), GenericString::from_flash(&TEXT0))
            }
            State::LoadingX => canvas.print_xy(Point::new(0, 1), GenericString::from_flash(&TEXT1)),
            State::LoadingY => canvas.print_xy(Point::new(0, 0), GenericString::from_flash(&TEXT2)),
            State::End => {
                //delegate / by-pass
                if let Some(widget) = &self.widget {
                    (*widget).draw(canvas, start_point)
                }
            }
        }
    }
}
