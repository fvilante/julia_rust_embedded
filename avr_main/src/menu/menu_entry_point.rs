use core::str::FromStr;

use alloc::string::ToString;
use heapless::String;
use heapless::Vec;
use lib_1::utils::common::convert_u16_to_string_decimal;
use crate::board::output_expander::OutputExpander;
use crate::board::{lcd, output_expander};
use crate::board::keyboard::KeyCode;
use crate::enviroment::front_panel::FrontPanel;
use crate::microcontroler::delay::delay_ms;
use super::database::DataBase;
use super::flash::FlashString;
use super::keyboard::Keyboard;
use super::canvas::Canvas;
use super::point::Point;
use super::widget::caption::Caption;
use super::widget::execucao;
use super::widget::execucao::Execucao;
use super::widget::field::Field;
use super::widget::field::FieldBuffer;
use super::widget::main_menu;
use super::widget::main_menu::MainMenu;
use super::widget::main_menu::State;
use super::widget::manual_mode::ManualMode;
use super::widget::manual_mode::ManualModeState;
use super::widget::menu_item;
use super::widget::menu_item::MenuItem;
use super::widget::menu_item::MenuItemParsed;
use super::widget::menu_item::parse_menu_item_constructor_string;
use super::widget::splash::Splash;
use super::widget::submenu::Items;
use super::widget::submenu::SubMenu;
use crate::menu::widget::widget::Widget;


use avr_progmem::progmem;

progmem! {

    //                          123456789012345678901234567890123456789 -> 39 characters   
    static progmem string T0 = "Posicao inicial             ${nnnnn} mm";
    static progmem string T1 = "Posicao final               ${nnnnn} mm";
    static progmem string T2 = "Velocidade de avanco      ${nnnnn} mm/s";
    static progmem string T3 = "Velocidade de retorno     ${nnnnn} mm/s";
    static progmem string T4 = "Aceleracao de avanco     ${nnnnn} mm/s2";
    static progmem string T5 = "Aceleracao de reto       ${nnnnn} mm/s2";
    static progmem string T6 = "Numero de mensagens no avanco     ${nn}";
    static progmem string T7 = "Numero de mensagens no retorno    ${nn}";
    static progmem string T8 = "Modo continuo ou passo-a-passo [${alt1}]";
    static progmem string T9 = "Logica do start externo        [${alt2}]";


    static progmem string E0 = "Erro de carga de parametro";
    static progmem string S0 = "Posicao Inicial";
    static progmem string S1 = "Posicao Final";
    static progmem string S2 = "Velocidade de Avanco";
    static progmem string S3 = "Velocidade de Retorno";
    static progmem string S4 = "Aceleracao de Avanco";
    static progmem string S5 = "Aceleracao de Retorno";
    static progmem string S6 = "Start Automatico no Avanco";
    static progmem string S7 = "Start Automatico no Retorno";

    //NOTE: it is possible to load any type in progmem not only strings
    static progmem A0: [u8; 6] = [0,1,2,3,4,5];
    static progmem string ERRO_01 = "Erro de construcao de string";
}


struct SystemEnviroment {
    pub output_expander: OutputExpander,
    pub keyboard: Keyboard,
    pub canvas: Canvas,
}

pub const LINE_0: bool = false;
pub const LINE_1: bool = true;

impl SystemEnviroment {
    pub fn new() -> Self {
        lcd::lcd_initialize();
        let mut output_expander = OutputExpander::new();
        let beep = |on:bool| { OutputExpander::new().BUZZER(on).commit(); };
        let mut keyboard = Keyboard::new(beep);
        let canvas = Canvas::new();
        Self {
            output_expander,
            keyboard,
            canvas,
        }
    }

    pub fn get_front_panel<'a>(&'a mut self) -> FrontPanel<'a> {
        let front_panel: FrontPanel<'a> = FrontPanel::new(&mut self.output_expander);
        front_panel
    }

}

struct SubMenu2 {
    menu_item_0: MenuItem,
    menu_item_1: MenuItem,
    current_selector: bool,  // false = line0, true = line1
}

impl SubMenu2 {
    pub fn new() -> Self {
        let point0a = Point::new(1,0);
        let point0b = Point::new(30,0);
        let text0: FlashString = FlashString::new(&S0);
        let array0: FieldBuffer = String::from_str("0000").unwrap();

        let point1a = Point::new(1,1);
        let point1b = Point::new(30,1);
        let text1: FlashString = FlashString::new(&S1);
        let array1: FieldBuffer = String::from_str("0000").unwrap();

        let mut menu_item_0 = MenuItem::new(point0a, text0, point0b, array0);
        let mut menu_item_1 = MenuItem::new(point1a, text1, point1b, array1);
        menu_item_0.set_edit_mode(false);
        menu_item_1.set_edit_mode(false);
        Self {
            menu_item_0,
            menu_item_1,
            current_selector: LINE_0,
        }
    }

    // false = line0, true = line1
    pub fn get_value_if_it_has_changed(&mut self, line: bool) -> Option<FieldBuffer> {
        if line == LINE_0 {
            self.menu_item_0.get_value_if_it_has_changed()
        } else {
            self.menu_item_1.get_value_if_it_has_changed()
        }
    }

    // false = line0, true = line1
    pub fn set_edit_mode(&mut self, line: bool, value: bool) {
        if line == LINE_0 {
            self.menu_item_0.set_edit_mode(value)
        } else {
            self.menu_item_1.set_edit_mode(value)
        }
    }

    // if is in edit mode returns Some<Line>
    pub fn is_in_edit_mode(&self) -> Option<bool> {
        let is_in_edit_mode_0 = self.menu_item_0.is_in_edit_mode();
        let is_in_edit_mode_1 = self.menu_item_1.is_in_edit_mode();
        let is_not_in_edit_mode = !is_in_edit_mode_0 && !is_in_edit_mode_1;
        if is_not_in_edit_mode {
            None
        } else {
            if is_in_edit_mode_0 {
                Some(LINE_0)
            } else {
                Some(LINE_1)
            }
        }
    }

}


impl SubMenu2 {
    pub fn send_key(&mut self, key: KeyCode) {
        let is_in_edit_mode = self.is_in_edit_mode();

        match is_in_edit_mode {
            //is editing some line
            Some(current_line) => {
                // delegate keys 
                if current_line == LINE_0 {
                    self.menu_item_0.send_key(key);
                } else { // LINE_1
                    self.menu_item_1.send_key(key);
                }
            }

            //not editing any line
            None => {
                // navigate menu
                match key {
                    KeyCode::KEY_DIRECIONAL_PARA_BAIXO => {
                        if self.current_selector == LINE_0 {
                            self.current_selector = LINE_1
                        } else {
                            // overflow
                        }
                     },
                    KeyCode::KEY_DIRECIONAL_PARA_CIMA => {
                        if self.current_selector == LINE_1 {
                            self.current_selector = LINE_0
                        } else {
                            // overflow
                        }
                     },
                    KeyCode::KEY_ENTER => {
                        if self.current_selector == LINE_0 {
                            self.menu_item_0.set_edit_mode(true);
                        } else { // LINE_1
                            self.menu_item_1.set_edit_mode(true);
                        }
                    }
                    _ => { }
                }
            }
        }
        
    }

    pub fn update(&mut self) {
        self.menu_item_0.update();
        self.menu_item_1.update();
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        fn draw_selector(self_: &SubMenu2, line: bool, canvas: &mut Canvas) {
            fn draw_char(self_: &SubMenu2, canvas: &mut Canvas) {
                match self_.is_in_edit_mode() {
                    Some(_) => canvas.print_char('*'),
                    None => canvas.print_char('>')
                }
                
            }
            if line == LINE_0 {
                canvas.set_cursor(Point::new(0,0));
                draw_char(self_, canvas);
                canvas.set_cursor(Point::new(0,1));
                canvas.print_char(' ');
            } else {
                canvas.set_cursor(Point::new(0,0));
                canvas.print_char(' ');
                canvas.set_cursor(Point::new(0,1));
                draw_char(self_, canvas);
            }
        }
        if self.current_selector == LINE_0 {
            draw_selector(self, LINE_0, canvas);
        } else {
            draw_selector(self, LINE_1, canvas);
        }
        self.menu_item_0.draw(canvas);
        self.menu_item_1.draw(canvas);
    }
}

pub fn development_entry_point() -> ! {

    let SystemEnviroment{mut canvas, mut keyboard, ..} = SystemEnviroment::new();

    canvas.render();  

    let mut submenu = SubMenu2::new();

    loop { 

        if let Some(key) = keyboard.get_key() {
            submenu.send_key(key);
        }
        
        submenu.update();
        submenu.draw(&mut canvas);
        canvas.render();
        if let Some(value) = submenu.get_value_if_it_has_changed(false) {
            canvas.set_cursor(Point::new(0,1));
            canvas.print_string(String::from_str("Coletado=").unwrap() as FieldBuffer);
            canvas.print_string(value);
            canvas.render();
            delay_ms(100);
            //submenu.set_edit_mode(LINE_0, true);
            //canvas.render();
            //loop { }
        }
        if let Some(value) = submenu.get_value_if_it_has_changed(true) {
            canvas.set_cursor(Point::new(0,0));
            canvas.print_string(String::from_str("Coletado=").unwrap() as FieldBuffer);
            canvas.print_string(value);
            canvas.render();
            delay_ms(100);
            //submenu.set_edit_mode(LINE_1, true);
            //canvas.render();
            //loop { }
        }
    }


    
    //splash
    let mut splash = Splash::new(4500);
    loop {
        if let Some(key) = keyboard.get_key() {
            splash.send_key(key);
        }
        splash.update();
        splash.draw(&mut canvas);
        if splash.isRunningYet == false {
            break;
        }
        canvas.render();
    }

    

    //main menu
    let mut manual_mode = ManualMode::new();
    let mut current_state: State = State::MAIN_MENU;
    //submenu 'Programa'
    let mut items: Items = Vec::new();
    items.push(FlashString::new(&S0));
    items.push(FlashString::new(&S1));
    items.push(FlashString::new(&S2));
    items.push(FlashString::new(&S3));
    let mut menu = SubMenu::new(items);
    // main loop
    loop {
        match  current_state {

            State::MAIN_MENU => {

                MainMenu::draw(&mut canvas);
                if let Some(key) = keyboard.get_key() {
                    match key {
                        KeyCode::KEY_MANUAL => current_state = State::MANUAL,
                        KeyCode::KEY_EXECUCAO => current_state = State::EXECUCAO,
                        KeyCode::KEY_PROGRAMA => current_state = State::PROGRAMA,
                        _ => { }
                    }
                }
            }

            State::EXECUCAO => {
                
                if let Some(key) = keyboard.get_key() {
                    if key == KeyCode::KEY_ESC {
                        current_state = State::MAIN_MENU;
                    } else {
                        // do nothing
                    }
                }
                Execucao::draw(&mut canvas);               
            }

            State::MANUAL => {

                if let Some(key) = keyboard.get_key() {
                    manual_mode.send_key(key)
                }
                if manual_mode.current_state == ManualModeState::DISABLED {
                    manual_mode.current_state = ManualModeState::FIRST_SCREEN;
                    current_state = State::MAIN_MENU;
                }
                manual_mode.draw(&mut canvas); 

            }

            State::PROGRAMA => {

                if let Some(key) = keyboard.get_key() {
                    menu.send_key(key);
                }
                menu.update();
                menu.draw(&mut canvas);

            }
        }

        canvas.render();
    }

    
}