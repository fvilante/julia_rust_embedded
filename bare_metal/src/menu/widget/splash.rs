use avr_progmem::progmem;
use cross_platform::protocol::transport::transport_layer::TransportLayer;

use crate::geometry::point::Point;
use crate::menu::model::{send_all, CmppData};
use crate::string::flash::FlashString;
use crate::{
    board::{keypad::KeyCode, lcd},
    menu::{model::DataModel, screen_buffer::ScreenBuffer},
    microcontroler::{delay::delay_ms, timer::now},
};

pub fn show_communication_error_message() {
    lcd::clear();
    lcd::set_cursor(0, 0);
    lcd::print("Erro de comunicacao serial");
    delay_ms(4000);
}

progmem! {
    static progmem string TEXT0 = "Posijet Industria e Comercio Ltda.";
    pub static progmem string POR_FAVOR_AGUARDE_CARGA_DO_PROGRAMA_X = "Por favor aguarde a carga do programa X";
    pub static progmem string POR_FAVOR_AGUARDE_CARGA_DO_PROGRAMA_Y = "Por favor aguarde a carga do programa Y";
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
    Loading = 2,
    End = 3,
}

impl State {
    fn from_u8(state: u8) -> Self {
        match state {
            0 => Self::Initial,
            1 => Self::BrandName,
            2 => Self::Loading,
            3 => Self::End,
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
    model: &'a DataModel,
    transport_x: &'a TransportLayer<'a>,
    transport_y: &'a TransportLayer<'a>,
}

impl<'a> Splash<'a> {
    pub fn new(
        model: &'a DataModel,
        transport_x: &'a TransportLayer<'a>,
        transport_y: &'a TransportLayer<'a>,
    ) -> Self {
        let initial_state = State::Initial;
        Self {
            current_state: initial_state,
            next_state_time_point: now() as u32 + Self::get_time_to_wait_in(initial_state),
            model,
            transport_x,
            transport_y,
        }
    }

    /// gets time interval (in miliseconds) to wait until reach next state
    fn get_time_to_wait_in(current_state: State) -> u32 {
        match current_state {
            State::Initial => 0,
            State::BrandName => 2000,
            State::Loading => 2000,
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

    pub fn draw(&self, screen_buffer: &mut ScreenBuffer) {
        screen_buffer.clear();
        match self.current_state {
            State::Initial => {}
            State::BrandName => {
                screen_buffer.set_cursor(Point::new(4, 0));
                screen_buffer.print(FlashString::new(&TEXT0));
            }
            State::Loading => send_all_and_show_user_info_on_screen(
                self.model,
                //screen_buffer,
                self.transport_x,
                self.transport_y,
            ),

            State::End => {
                // do nothing
            }
        }
    }
}

/// TODO: Eventually this function should be place in a better location instead of in this module.
pub fn send_all_and_show_user_info_on_screen(
    model: &DataModel,
    //screen_buffer: &mut ScreenBuffer,
    transport_x: &TransportLayer,
    transport_y: &TransportLayer,
) {
    // **************************
    // Send all data to X-Axis
    // **************************

    lcd::clear();
    lcd::set_cursor(0, 1);
    lcd::print("Por favor aguarde a carga do programa X");

    // TODO: Choose the right `arquivo de eixo` and `config de eixo` to send. Consider
    // the cases when the system have more than one axis, and more than one program
    let cmpp_data_x = CmppData {
        arquivo_de_eixo: &model.arquivo_de_eixo_00,
        configuracao_de_eixo: &model.configuracao_do_eixo_x,
    };
    send_all(&transport_x, &cmpp_data_x);

    // **************************
    // Send all data to Y-Axis
    // **************************

    lcd::clear();
    lcd::set_cursor(0, 0);
    lcd::print("Por favor aguarde a carga do programa Y");

    // TODO: Choose the right `arquivo de eixo` and `config de eixo` to send. Consider
    // the cases when the system have more than one axis, and more than one program
    let cmpp_data_y = CmppData {
        arquivo_de_eixo: &model.arquivo_de_eixo_00,
        configuracao_de_eixo: &model.configuracao_do_eixo_y,
    };
    send_all(&transport_y, &cmpp_data_y);
}
