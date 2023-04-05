use avr_progmem::progmem;
use lib_1::protocol::transport::transport_layer::TransportLayer;

use crate::{
    board::{keyboard::KeyCode, lcd},
    menu::{canvas::Canvas, flash::FlashString, model::DataStorage, point::Point},
};

use super::{
    execucao::MenuExecucao,
    manual_mode::{ManualModeMenu, ManualModeState},
    submenu::render::SubmenuProgramaRender,
    widget::Widget,
};

progmem! {
    //                             1234567890123456789012345678901234567890
    static progmem string LINE0 = "Selecione modo de programacao desejado";
    static progmem string LINE1 = "MANUAL    EXECUCAO    PROGRAMA";
}

#[derive(PartialEq, Clone, Copy)]
pub enum State {
    MainMenu,
    Manual,
    Execucao,
    Programa,
}

pub struct MainMenu<'a> {
    current_state: State,
    menu_manual: ManualModeMenu<'a>,
    menu_execucao: MenuExecucao<'a>,
    menu_programa: SubmenuProgramaRender<'a>,
    transport: &'a TransportLayer<'a>,
    model: &'a DataStorage,
    //program_mode: IWidget,
}

impl<'a> MainMenu<'a> {
    pub fn new(
        menu_manual: ManualModeMenu<'a>,
        menu_execucao: MenuExecucao<'a>,
        menu_programa: SubmenuProgramaRender<'a>,
        transport: &'a TransportLayer<'a>,
        model: &'a DataStorage,
    ) -> Self {
        Self {
            current_state: State::MainMenu,
            menu_manual,
            menu_execucao,
            menu_programa,
            transport,
            model,
        }
    }

    fn draw_main_menu(&self, canvas: &mut Canvas) {
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

        canvas.clear();
        for line_number in 0..2 {
            let (point, flash_string) = get_line_helper(line_number);
            canvas.set_cursor(point);
            canvas.print_flash_str(flash_string);
        }
    }
}

impl<'a> MainMenu<'a> {
    pub fn send_key(&mut self, key: KeyCode) {
        match self.current_state {
            State::MainMenu => match key {
                KeyCode::KEY_MANUAL => self.current_state = State::Manual,
                KeyCode::KEY_EXECUCAO => self.current_state = State::Execucao,
                KeyCode::KEY_PROGRAMA => self.current_state = State::Programa,
                _ => {}
            },
            State::Manual => self.menu_manual.send_key(key),
            State::Execucao => {
                if key == KeyCode::KEY_ESC {
                    self.current_state = State::MainMenu
                } else {
                    self.menu_execucao.send_key(key)
                }
            }
            State::Programa => {
                self.menu_programa.send_key(key) // TODO: How can I do to return from `menu programa`
            }
        }
    }

    pub fn update(&mut self) {
        match self.current_state {
            State::MainMenu => {}
            State::Manual => {
                if self.menu_manual.current_state == ManualModeState::Resting {
                    self.menu_manual.current_state = ManualModeState::FirstScreen; // reset state
                    self.current_state = State::MainMenu
                } else {
                    self.menu_manual.update()
                }
            }
            State::Execucao => self.menu_execucao.update(),
            State::Programa => {
                if self.menu_programa.must_return_to_main_menu {
                    self.current_state = State::MainMenu;
                    self.menu_programa.must_return_to_main_menu = false;
                    // send all data to cmpp when transitioning from menu_programa to main_menu
                    // TODO: Place the below text in the Flash
                    lcd::clear();
                    lcd::set_cursor(0, 1);
                    lcd::print("Por favor aguarde a carga do programa X");
                    for _response in self.model.send_all(&self.transport) {}
                    // saves data into the eeprom
                    self.model.save_to_eeprom();
                } else {
                    self.menu_programa.update()
                }
            }
        }
    }

    pub fn draw(&mut self, canvas: &mut Canvas, start_point: Point) {
        match self.current_state {
            State::MainMenu => self.draw_main_menu(canvas),
            State::Manual => self.menu_manual.draw(canvas, start_point),
            State::Execucao => self.menu_execucao.draw(canvas, start_point),
            State::Programa => self.menu_programa.draw(canvas),
        }
    }
}
