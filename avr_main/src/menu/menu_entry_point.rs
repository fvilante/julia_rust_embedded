use core::str::FromStr;
use core::ops::Range;
use alloc::string::ToString;
use avr_progmem::string::PmString;
use heapless::String;
use heapless::Vec;
use lib_1::utils::common::convert_u16_to_string_decimal;
use crate::board::output_expander::OutputExpander;
use crate::board::{lcd, output_expander};
use crate::board::keyboard::KeyCode;
use crate::enviroment::front_panel::FrontPanel;
use crate::menu::accessor::Accessor;
use crate::menu::widget::optional::OptionsBuffer;
use crate::menu::widget::sub_menu::SubMenu;
use crate::menu::widget::widget_tests::SystemEnviroment;
use crate::microcontroler::delay::delay_ms;
use crate::microcontroler::timer::now;
use super::flash::FlashString;
use super::keyboard::Keyboard;
use super::canvas::Canvas;
use super::menu_manager::MenuManager;
use super::point::Point;
use super::point::Point1d;
use super::widget::caption::Caption;
use super::widget::execucao;
use super::widget::execucao::MenuExecucao;
use super::widget::field::Field;
use super::widget::field::FieldBuffer;
use super::widget::main_menu;
use super::widget::main_menu::MainMenu;
use super::widget::main_menu::State;
use super::widget::manual_mode::ManualModeMenu;
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
    static progmem string POSICAO_INICIAL = "Posicao Inicial";
    static progmem string POSICAO_FINAL = "Posicao Final";
    static progmem string VELOCIDADE_DE_AVANCO = "Velocidade de Avanco";
    static progmem string VELOCIDADE_DE_RETORNO = "Velocidade de Retorno";
    static progmem string S4 = "Aceleracao de Avanco";
    static progmem string S5 = "Aceleracao de Retorno";
    static progmem string START_AUTOMATICO_NO_AVANCO = "Start Automatico no Avanco";
    static progmem string START_AUTOMATICO_NO_RETORNO = "Start Automatico no Retorno";
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

//    //main menu
//    let mut menu_execucao = MenuExecucao::new();
//    let mut menu_manual = ManualModeMenu::new();
//    let mut main_menu: MainMenu = MainMenu::new(menu_manual, menu_execucao);
//
//    // main loop
//    loop {
//        
//        if let Some(key) = keyboard.get_key() {
//            main_menu.send_key(key);
//        }
//
//        main_menu.update();
//        main_menu.draw(&mut canvas);
//        canvas.render();
//    }
//
//    // menu view
//    let splash1 = &mut Splash::new(None);
//    let splash2 = &mut Splash::new(Some(splash1));
//    let mut menu_manager = MenuManager::new(Some(splash2));
//    loop {
//        if let Some(key) = keyboard.get_key() {
//            menu_manager.send_key(key);
//        }
//
//        menu_manager.update();
//        menu_manager.draw(&mut canvas);
//        canvas.render();
//    }


    // submenu
    let mut menu_list: MenuList = Vec::new();

    fn make_menu_item_helper<const N: usize>(point1_: u8, point2_: u8, pgm_text: &'static PmString<N>) -> (Point1d, Point1d, FlashString, ) {
        let point1 = Point1d::new(1);
        let point2 = Point1d::new(33);
        let text: FlashString = FlashString::new(pgm_text);
        (point1, point2, text)
    }

    // =========================================================
    let (point1, point2, text) = make_menu_item_helper(1, 33, &POSICAO_INICIAL);
    let accessor = Accessor::from_variable( unsafe{ &mut FILE[0] });
    let field = Field::from_numerical(accessor, 0, 4, 10..100);
    let mut menu_item = MenuItem::new(point1, text, point2, field, None);
    menu_list.push(menu_item);

    // =========================================================
    let (point1, point2, text) = make_menu_item_helper(1, 33, &POSICAO_FINAL);
    let accessor = Accessor::from_variable( unsafe{ &mut FILE[1] });
    let field = Field::from_numerical(accessor, 0, 4, 0..0xFFFF);
    let mut menu_item = MenuItem::new(point1, text, point2, field, None);
    menu_list.push(menu_item);

    
    // =========================================================
    //options
    let (point1, point2, text) = make_menu_item_helper(1, 33, &START_AUTOMATICO_NO_RETORNO);
    let accessor = Accessor::from_variable( unsafe{ &mut CURSOR });
    let mut options: OptionsBuffer = Vec::new();
    options.push(FlashString::new(&O1));
    options.push(FlashString::new(&O2));
    let field = Field::from_optional(options, accessor);
    let mut menu_item = MenuItem::new(point1, text, point2, field, None);
    menu_list.push(menu_item);

//    // =========================================================
//    //options
//    let mut options: OptionsBuffer = Vec::new();
//    options.push(FlashString::new(&O1));
//    options.push(FlashString::new(&O2));
//    let point1 = Point1d::new(1);
//    let point2 = Point1d::new(30);
//    let text: FlashString = FlashString::new(&START_AUTOMATICO_NO_AVANCO);
//    fn setter_d(cursor: Cursor) {
//        unsafe {
//            CURSOR = cursor;
//        }
//    }
//    fn getter_d() -> Cursor {
//        unsafe {
//            CURSOR.clone()
//        }
//    }
//    let accessor = Accessor::new(setter_d, getter_d);
//    let field = Field::from_optional(options, accessor);
//    let mut menu_item = MenuItem::new(point1, text, point2, field, None);
//    menu_list.push(menu_item);


//    // =========================================================
//    let point1 = Point1d::new(1);
//    let point2 = Point1d::new(33);
//    let text: FlashString = FlashString::new(&VELOCIDADE_DE_RETORNO);
//    fn setter_e(value: u16) {
//        unsafe {
//            FILE[2] = value;
//        }
//    }
//    
//    fn getter_e() -> u16 {
//        unsafe {
//            FILE[2]
//        }
//    }
//    let accessor = Accessor::new(setter_e, getter_e);
//    let field = Field::from_numerical(accessor, 0, 4, 0..0xFFFF);
//    let mut menu_item = MenuItem::new(point1, text, point2, field, None);
//    menu_list.push(menu_item);

//    // =========================================================
//    let point1 = Point1d::new(1);
//    let point2 = Point1d::new(33);
//    let text: FlashString = FlashString::new(&VELOCIDADE_DE_AVANCO);
//    fn setter_f(value: u16) {
//        unsafe {
//            FILE[2] = value;
//        }
//    }
//    
//    fn getter_f() -> u16 {
//        unsafe {
//            FILE[2]
//        }
//    }
//    let accessor = Accessor::new(setter_f, getter_f);
//    let field = Field::from_numerical(accessor, 0, 4, 0..0xFFFF);
//    let mut menu_item = MenuItem::new(point1, text, point2, field, None);
//    menu_list.push(menu_item);


    // -----------------------------
    
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

    
}