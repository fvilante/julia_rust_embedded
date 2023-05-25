use super::{
    execucao::MenuExecucaoControler,
    manual_mode::{ManualModeMenuControler, ManualModeState},
    submenu_programa::menu_programa_controler::MenuProgramaControler,
    widget::Widget,
};
use crate::board::front_panel::FrontPanel;
use crate::board::front_panel::FrontPanelAvrHardware;
use crate::geometry::point::Point;
use crate::string::flash::FlashString;
use crate::{
    board::{keypad::KeyCode, lcd},
    menu::{model::DataModel, screen_buffer::ScreenBuffer},
    microcontroler::delay::delay_ms,
};
use avr_progmem::progmem;
use cross_platform::protocol::transport::transport_layer::TransportLayer;

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

pub struct MainMenu<'a, F: FrontPanel> {
    current_state: State,
    menu_manual_controler: ManualModeMenuControler<'a>,
    menu_execucao_controler: MenuExecucaoControler<'a>,
    menu_programa_controler: MenuProgramaControler<'a>,
    transport: &'a TransportLayer<'a>,
    model: &'a DataModel,
    //TODO: We're just controling 3 Leds (Execucao, Manual, Programa), better would be to wrap
    //the type 'FrontPanel' into an abstract class.
    front_panel_leds: &'a mut F,
}

impl<'a, F: FrontPanel> MainMenu<'a, F> {
    pub fn new(
        menu_manual_controler: ManualModeMenuControler<'a>,
        menu_execucao_controler: MenuExecucaoControler<'a>,
        menu_programa_controler: MenuProgramaControler<'a>,
        transport: &'a TransportLayer<'a>,
        model: &'a DataModel,
        front_panel_leds: &'a mut F,
    ) -> Self {
        Self {
            current_state: State::MainMenu,
            menu_manual_controler,
            menu_execucao_controler,
            menu_programa_controler,
            transport,
            model,
            front_panel_leds,
        }
    }

    fn draw_main_menu(&self, screen_buffer: &mut ScreenBuffer) {
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

        screen_buffer.clear();
        for line_number in 0..2 {
            let (point, flash_string) = get_line_helper(line_number);
            screen_buffer.set_cursor(point);
            screen_buffer.print(flash_string);
        }
    }
}

impl<'a, F: FrontPanel> Widget for MainMenu<'a, F> {
    fn send_key(&mut self, key: KeyCode) {
        match self.current_state {
            State::MainMenu => match key {
                KeyCode::KEY_MANUAL => {
                    self.front_panel_leds.LED_MANUAL(true);
                    self.current_state = State::Manual;
                }
                KeyCode::KEY_EXECUCAO => {
                    self.front_panel_leds.LED_EXECUCAO(true);
                    self.current_state = State::Execucao;
                }
                KeyCode::KEY_PROGRAMA => {
                    self.front_panel_leds.LED_PROGRAMA(true);
                    self.current_state = State::Programa;
                }
                _ => {}
            },
            State::Manual => {
                self.menu_manual_controler.send_key(key);
            }
            State::Execucao => {
                if key == KeyCode::KEY_ESC {
                    self.current_state = State::MainMenu;
                } else {
                    self.menu_execucao_controler.send_key(key)
                }
            }
            State::Programa => {
                self.menu_programa_controler.send_key(key) // TODO: How can I do to return from `menu programa`
            }
        }
    }

    fn update(&mut self) {
        match self.current_state {
            State::MainMenu => {
                // Reset frontend leds
                self.front_panel_leds.LED_MANUAL(false);
                self.front_panel_leds.LED_EXECUCAO(false);
                self.front_panel_leds.LED_PROGRAMA(false);
            }

            State::Manual => {
                if self.menu_manual_controler.current_state == ManualModeState::Resting {
                    self.menu_manual_controler.current_state = ManualModeState::FirstScreen; // reset state
                    self.current_state = State::MainMenu
                } else {
                    self.menu_manual_controler.update()
                }
            }
            State::Execucao => self.menu_execucao_controler.update(),
            State::Programa => {
                if self.menu_programa_controler.must_return_to_main_menu {
                    self.current_state = State::MainMenu;
                    self.menu_programa_controler.must_return_to_main_menu = false;
                    // send all data to cmpp when transitioning from menu_programa to main_menu
                    // TODO: Place the below text in the Flash
                    lcd::clear();
                    lcd::set_cursor(0, 1);
                    lcd::print("Por favor aguarde a carga do programa X");
                    for response in self.model.send_all(&self.transport) {
                        if let Err(_e) = response {
                            lcd::clear();
                            lcd::set_cursor(0, 0);
                            lcd::print("Erro de comunicacao serial");
                            delay_ms(4000);
                            break;
                        }
                    }
                    // saves data into the eeprom
                    self.model.save_to_eeprom();
                } else {
                    self.menu_programa_controler.update()
                }
            }
        }
    }

    fn draw(&self, screen_buffer: &mut ScreenBuffer, start_point: Point) {
        match self.current_state {
            State::MainMenu => self.draw_main_menu(screen_buffer),
            State::Manual => self.menu_manual_controler.draw(screen_buffer, start_point),
            State::Execucao => self
                .menu_execucao_controler
                .draw(screen_buffer, start_point),
            State::Programa => self
                .menu_programa_controler
                .draw(screen_buffer, start_point),
        }
    }
}
