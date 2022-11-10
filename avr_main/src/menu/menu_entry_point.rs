use core::str::FromStr;
use core::ops::Range;
use alloc::string::ToString;
use heapless::String;
use heapless::Vec;
use lib_1::utils::common::convert_u16_to_string_decimal;
use crate::board::output_expander::OutputExpander;
use crate::board::{lcd, output_expander};
use crate::board::keyboard::KeyCode;
use crate::enviroment::front_panel::FrontPanel;
use crate::menu::accessor::Accessor;
use crate::menu::widget::optional::OptionsBuffer;
use crate::menu::widget::sub_menu::MenuItemEnum;
use crate::menu::widget::sub_menu::MenuItemEnumGetter;
use crate::menu::widget::sub_menu::SubMenu;
use crate::menu::widget::widget_tests::SystemEnviroment;
use crate::microcontroler::delay::delay_ms;
use crate::microcontroler::timer::now;
use super::flash::FlashString;
use super::keyboard::Keyboard;
use super::canvas::Canvas;
use super::point::Point;
use super::point::Point1d;
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
use super::widget::optional::Optional;
use super::widget::splash::Splash;
use super::widget::sub_menu::MenuList;
use super::widget::widget_tests::optional_widget_test;
use crate::menu::widget::widget::Widget;
use crate::menu::widget::cursor::Cursor;


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
    static progmem string O1 = "Ligado";
    static progmem string O2 = "Deslig";

    //NOTE: it is possible to load any type in progmem not only strings
    static progmem A0: [u8; 6] = [0,1,2,3,4,5];
    static progmem string ERRO_01 = "Erro de construcao de string";
}




static mut FILE: [u16; 4] = [0x00;4];
static mut CURSOR: Cursor = Cursor::new(0..2, 0);



pub fn development_entry_point() -> ! {

    //optional_widget_test();

    let SystemEnviroment{mut canvas, mut keyboard, ..} = SystemEnviroment::new();

    canvas.render();  

    // submenu
    let mut menu_list: MenuList = Vec::new();
    menu_list.push(|| {
        let point1 = Point1d::new(1);
        let point2 = Point1d::new(33);
        let text: FlashString = FlashString::new(&S0);
        fn setter(value: u16) {
            unsafe {
                FILE[0] = value;
            }
        }
        fn getter() -> u16 {
            unsafe {
                FILE[0]
            }
        }
        let accessor = Accessor::new(setter, getter);
        let field = Field::from_numerical(accessor, 0, 4, 10..100);
        let mut menu_item = MenuItem::new(point1, text, point2, field);
        MenuItemEnum::MenuItem(menu_item)
    });

    menu_list.push(|| {
        let point1 = Point1d::new(1);
        let point2 = Point1d::new(33);
        let text: FlashString = FlashString::new(&S1);
        fn setter(value: u16) {
            unsafe {
                FILE[1] = value;
            }
        }
        
        fn getter() -> u16 {
            unsafe {
                FILE[1]
            }
        }
        let accessor = Accessor::new(setter, getter);
        let field = Field::from_numerical(accessor, 0, 4, 0..0xFFFF);
        let mut menu_item = MenuItem::new(point1, text, point2, field);
        MenuItemEnum::MenuItem(menu_item)
    });

    //options
    menu_list.push(|| {
        let mut options: OptionsBuffer = Vec::new();
        options.push(FlashString::new(&O1));
        options.push(FlashString::new(&O2));

        let point1 = Point1d::new(1);
        let point2 = Point1d::new(30);
        let text: FlashString = FlashString::new(&S7);
        
        fn setter(cursor: Cursor) {
            unsafe {
                CURSOR = cursor;
            }
        }
    
        fn getter() -> Cursor {
            unsafe {
                CURSOR.clone()
            }
        }
        let accessor = Accessor::new(setter, getter);
        let field = Field::from_optional(options, accessor);
        let mut menu_item = MenuItem::new(point1, text, point2, field);
        MenuItemEnum::MenuItem(menu_item)
    });

    menu_list.push(|| {
        let point1 = Point1d::new(1);
        let point2 = Point1d::new(33);
        let text: FlashString = FlashString::new(&S3);
        fn setter(value: u16) {
            unsafe {
                FILE[2] = value;
            }
        }
        
        fn getter() -> u16 {
            unsafe {
                FILE[2]
            }
        }
        let accessor = Accessor::new(setter, getter);
        let field = Field::from_numerical(accessor, 0, 4, 0..0xFFFF);
        let mut menu_item = MenuItem::new(point1, text, point2, field);
        MenuItemEnum::MenuItem(menu_item)
    });



    let mut submenu = SubMenu::new(menu_list);

    let fps = 30; // frames_per_second
    let mut next_frame: u64 = now() + (1000/fps);
    
    loop { 

        if let Some(key) = keyboard.get_key() {
            submenu.send_key(key);
        }
        
        if now() > next_frame {
            next_frame = now() + (1000/fps);
            submenu.update();
            submenu.draw(&mut canvas);
            canvas.render();
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

                // todo: submenu programa

            }
        }

        canvas.render();
    }

    
}