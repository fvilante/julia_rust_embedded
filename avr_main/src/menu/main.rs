use alloc::string::ToString;
use heapless::String;
use heapless::Vec;
use crate::board::output_expander::OutputExpander;
use crate::board::{lcd, output_expander};
use crate::board::keyboard::KeyCode;
use crate::enviroment::front_panel::FrontPanel;
use super::database::DataBase;
use super::flash::FlashString;
use super::keyboard::Keyboard;
use super::canvas::Canvas;
use super::widget::caption::Caption;
use super::widget::execucao;
use super::widget::execucao::Execucao;
use super::widget::field::Field;
use super::widget::main_menu;
use super::widget::main_menu::MainMenu;
use super::widget::main_menu::State;
use super::widget::manual_mode::ManualMode;
use super::widget::manual_mode::ManualModeState;
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




pub fn development_entry_point() -> ! {

    // initialization
    lcd::lcd_initialize();
    let mut output_expander = OutputExpander::new();
    let _front_panel = FrontPanel::new(&mut output_expander).reset();
    let beep = |on:bool| { OutputExpander::new().BUZZER(on).commit(); };
    let mut keyboard = Keyboard::new(beep);
    let mut canvas = Canvas::new();
    canvas.render();  
    
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
                
                lcd::clear();
                let mut database = DataBase::new();
                database.parameter_03().set(12);
                let x = database.parameter_03().get();
                lcd::print_u16_in_hex(x);
                loop { }

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